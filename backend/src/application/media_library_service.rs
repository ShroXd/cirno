use actix::Addr;
use actix_web::web::Data;
use anyhow::*;
use std::{result::Result::Ok, sync::Arc, time::Duration};
use tracing::*;

use crate::{
    application::media_item_service::insert_media_item,
    domain::media_library::{
        constant::SENTINEL_MEDIA_LIBRARY_ID, event::MediaLibraryEventType,
        media_library::create_media_library, task::MediaLibraryScanTask,
    },
    infrastructure::{
        database::database::Database,
        event_bus::{
            event_bus::{DomainEvent, EventBus},
            handler::EventHandlerConfig,
        },
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

#[instrument(skip(database_addr, parser_addr, ws_connections, task_pool, event_bus))]
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

    event_bus
        .on(
            |event| {
                matches!(
                    event,
                    DomainEvent::MediaLibrary(MediaLibraryEventType::MediaLibraryScanned(_)),
                )
            },
            move |event, task_id, event_bus| {
                let database_addr = database_addr.clone();
                let media_library_id = media_library_id.clone();

                async move {
                    debug!(
                        "Processing media library scanned event with task id: {}",
                        task_id
                    );

                    // TODO: Update task progress
                    if let DomainEvent::MediaLibrary(MediaLibraryEventType::MediaLibraryScanned(
                        media_library,
                    )) = event
                    {
                        for media_item in media_library.tv_show {
                            debug!("Processing media item: {:?}", media_item.title);
                            insert_media_item(media_library_id, media_item, database_addr.clone())
                                .await
                                .inspect_err(|e| error!("Failed to insert media item: {:?}", e))?;
                        }

                        debug!("Publishing media library saved event");
                        event_bus
                            .publish(
                                DomainEvent::MediaLibrary(MediaLibraryEventType::MediaLibrarySaved),
                                task_id,
                            )
                            .inspect_err(|e| error!("Failed to publish event: {:?}", e))?;
                    }

                    Ok(())
                }
            },
            EventHandlerConfig::one_time(),
        )
        .await;

    event_bus
        .on(
            |event| {
                matches!(
                    event,
                    DomainEvent::MediaLibrary(MediaLibraryEventType::MediaLibrarySaved)
                )
            },
            move |_, task_id, _| {
                let ws_connection_clone = ws_connection.clone();
                async move {
                    debug!(
                        "Sending media library saved notification for library {} with task {}",
                        media_library_id, task_id
                    );
                    ws_connection_clone
                        .try_send(Notification::MediaLibraryScanned(media_library_id, task_id))
                        .inspect_err(|e| error!("Failed to send notification: {:?}", e))?;

                    Ok(())
                }
            },
            EventHandlerConfig::with_exponential_retry(
                Duration::from_secs(1),
                Duration::from_secs(10),
                3,
                2.0,
                0.5,
            ),
        )
        .await;

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
