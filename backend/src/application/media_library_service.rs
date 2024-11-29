use actix::{spawn, Addr};
use actix_web::web::Data;
use anyhow::*;
use std::time::Duration;
use std::{result::Result::Ok, sync::Arc};
use tokio::select;
use tracing::*;

use crate::{
    application::media_item_service::insert_media_item,
    domain::{
        file_processor::scan_media_library,
        media_library::{media_library::create_media_library, task::MediaLibraryScanTask},
        pipeline::events::PipelineEvent,
    },
    infrastructure::{
        database::{actor::SENTINEL_MEDIA_LIBRARY_ID, database::Database},
        event_bus::event_bus::{EventBus, EventType, OtherEventType},
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

    let listener_handler = spawn(async move {
        let mut subscription = event_bus.subscribe();
        while let Ok(event) = subscription.recv().await {
            match event {
                (EventType::Other(OtherEventType::MediaLibraryScanned(ws_client_id)), task_id) => {
                    debug!("Media library scanned notification received");
                    match ws_connection
                        .try_send(Notification::MediaLibraryScanned(media_library_id))
                    {
                        Ok(_) => debug!("Media library scanned notification sent"),
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
        )
        .await?;

    select! {
        _ = listener_handler => {
            debug!("Listener completed normally");
        }
    }

    // TODO: This is a temporary async notification system
    // After DDD refactoring, this will be encapsulated in a dedicated notification domain service
    // that handles all async notifications and websocket communication
    // The service will:
    // 1. Manage websocket connections and client registrations
    // 2. Handle notification delivery and retries
    // 3. Provide a clean domain interface for sending notifications
    // 4. Support different notification types and channels
    // spawn(async move {
    //     debug!("Scanning media library");
    //     let media_items = match scan_media_library(directory_clone, parser_addr).await {
    //         Ok(media_library) => media_library.tv_show,
    //         Err(e) => {
    //             error!("Failed to scan media library: {:?}", e);
    //             return;
    //         }
    //     };

    //     debug!("Inserting media items");
    //     for media_item in media_items {
    //         if let Err(e) =
    //             insert_media_item(media_library_id, media_item, database_addr.clone()).await
    //         {
    //             error!("Failed to insert media item: {:?}", e);
    //             return;
    //         }
    //     }

    //     // Artificial delay to test frontend async UI behavior, will be removed
    //     tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

    //     debug!(
    //         "Sending media library scanned notification via websocket to client: {:?}",
    //         ws_client_key
    //     );
    //     // TODO: refactor this to single async manager actor
    //     let ws_connections = ws_connections.get(ws_client_key).await;
    //     if let Some(ws_connection) = ws_connections {
    //         match ws_connection
    //             .send(Notification::MediaLibraryScanned(media_library_id))
    //             .await
    //         {
    //             Ok(_) => debug!("Media library scanned notification sent"),
    //             Err(e) => error!("Failed to send notification: {:?}", e),
    //         }
    //     }
    // });

    Ok(CreateMediaLibraryResponse {
        media_library_id,
        async_task_id: task_id,
    })
}
