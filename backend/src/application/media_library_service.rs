use actix::Addr;
use actix_web::web::Data;
use anyhow::{Ok, *};
use std::{sync::Arc, time::Duration};
use tracing::*;

use crate::{
    application::media_item_service::insert_media_item,
    chain_events,
    domain::{
        media_library::{
            constant::SENTINEL_MEDIA_LIBRARY_ID, event::MediaLibraryEventType,
            media_library::create_media_library, task::MediaLibraryScanTask,
        },
        task::task::{AsyncTaskResponse, TaskIdentifiable, TaskType},
    },
    infrastructure::{
        database::database::Database,
        event_bus::{domain_event::DomainEvent, event_bus::EventBus, handler::EventHandlerConfig},
        organizer::organizer::ParserActor,
        task_pool::task_pool::TaskPool,
    },
    interfaces::{
        http_api::controllers::api_models::SaveMediaLibraryPayload, ws::utils::WsConnections,
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
) -> Result<AsyncTaskResponse<i64>> {
    let directory_clone = payload.directory.clone();
    let media_library_name = payload.name.clone();
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

    chain_events!(
        event_bus,
        {
            match_pattern: DomainEvent::MediaLibrary(MediaLibraryEventType::MediaLibraryScanned { .. }),
            handler: move |event, event_bus| {
                let database_addr = database_addr.clone();
                let media_library_id = media_library_id.clone();
                let media_library_name = media_library_name.clone();

                async move {
                    if let DomainEvent::MediaLibrary(MediaLibraryEventType::MediaLibraryScanned { media_library, task_identifier }) = event {
                        for media_item in media_library.tv_show {
                            debug!("Processing media item: {:?}", media_item.title);
                            insert_media_item(media_library_id, media_item, database_addr.clone())
                                .await
                                .inspect_err(|e| error!("Failed to insert media item: {:?}", e))?;
                        }
                        event_bus.publish(DomainEvent::MediaLibrary(MediaLibraryEventType::MediaLibrarySaved {
                            task_identifier,
                            media_library_id,
                            media_library_name,
                        }))?;
                    }
                    Ok(())
                }
            },
            config: EventHandlerConfig::one_time()
        },
        {
            match_pattern: DomainEvent::MediaLibrary(MediaLibraryEventType::MediaLibrarySaved { .. }),
            handler: move |event, _| {
                let ws_connection_clone = ws_connection.clone();
                async move {
                    event.send_notification::<serde_json::Value>(ws_connection_clone);
                    Ok(())
                }
            },
            config: EventHandlerConfig::with_exponential_retry(
                Duration::from_secs(1),
                Duration::from_secs(10),
                3,
                2.0,
                0.5,
            )
        }
    );

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

    Ok(AsyncTaskResponse {
        task_id,
        task_type: TaskType::MediaLibraryScan,
        payload: Some(media_library_id),
    })
}
