use actix::{spawn, Addr};
use actix_web::web::Data;
use anyhow::*;
use std::{result::Result::Ok, sync::Arc};
use tracing::*;

use crate::{
    application::media_item_service::insert_media_item,
    domain::media_library::{
        constant::SENTINEL_MEDIA_LIBRARY_ID, event::MediaLibraryEventType,
        media_library::create_media_library, task::MediaLibraryScanTask,
    },
    infrastructure::{
        database::database::Database,
        event_bus::event_bus::{DomainEvent, EventBus},
        organizer::organizer::ParserActor,
        task_pool::{
            model::{AsyncTask, TaskType},
            task_pool::TaskPool,
        },
    },
    interfaces::{
        http_api::controllers::api_models::{CreateMediaLibraryResponse, SaveMediaLibraryPayload},
        ws::{actor::Notification, utils::WsConnections},
    },
};

#[instrument(skip(database_addr, parser_addr, ws_connections, task_pool))]
pub async fn create_media_library_service(
    payload: SaveMediaLibraryPayload,
    database_addr: Data<Addr<Database>>,
    parser_addr: Data<Addr<ParserActor>>,
    ws_connections: Data<WsConnections>,
    task_pool: Data<TaskPool>,
    event_bus: Data<Arc<EventBus>>,
    ws_client_key: String,
) -> Result<CreateMediaLibraryResponse> {
    let directory_clone = payload.directory.clone();
    let database_addr = database_addr.into_inner();

    let media_library_id = create_media_library(payload, database_addr.clone()).await?;
    debug!("Media library created with id: {:?}", media_library_id);

    if media_library_id == SENTINEL_MEDIA_LIBRARY_ID {
        error!("Failed to create media library");
        return Err(anyhow!("Failed to create media library"));
    }

    let ws_connection = match ws_connections.get(ws_client_key.clone()).await {
        Some(ws_connection) => ws_connection,
        None => {
            error!("WebSocket connection not found");
            return Err(anyhow!("WebSocket connection not found"));
        }
    };

    let _ = spawn(async move {
        let mut subscription = event_bus.subscribe();
        while let Ok(event) = subscription.recv().await {
            match event {
                (
                    DomainEvent::MediaLibrary(MediaLibraryEventType::MediaLibraryScanned(
                        media_library,
                    )),
                    task_id,
                ) => {
                    // TODO: Update task progress
                    for media_item in media_library.tv_show {
                        insert_media_item(media_library_id, media_item, database_addr.clone())
                            .await
                            .unwrap();
                    }
                }
                (DomainEvent::MediaLibrary(MediaLibraryEventType::MediaLibrarySaved), task_id) => {
                    match ws_connection
                        .try_send(Notification::MediaLibraryScanned(media_library_id, task_id))
                    {
                        Ok(_) => debug!("Media library saved notification sent"),
                        Err(e) => error!("Failed to send notification: {:?}", e),
                    }
                }
                _ => {}
            }
        }
    });

    let mut task = MediaLibraryScanTask::new(directory_clone, parser_addr.into_inner());
    task.set_ws_client_id(ws_client_key.clone());
    let task_id = task_pool
        .register_task(
            TaskType::MediaLibraryScan,
            ws_client_key.clone(),
            Box::new(task),
            None,
        )
        .await?;

    Ok(CreateMediaLibraryResponse {
        media_library_id,
        async_task_id: task_id,
    })
}
