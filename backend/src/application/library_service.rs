use actix_web::web::Data;
use anyhow::{Ok, *};
use std::sync::Arc;
use tracing::*;

use crate::{
    application::media_item_service::insert_media_item,
    chain_events,
    domain::{
        media_library::{
            constant::SENTINEL_LIBRARY_ID, event::LibraryEventType, library::create_library,
            task::LibraryScanTask,
        },
        task::async_task::{AsyncTaskResponse, TaskIdentifiable, TaskType},
    },
    infrastructure::event_dispatcher::{domain_event::DomainEvent, handler::EventHandlerConfig},
    init::app_state::AppState,
    interfaces::http_api::controllers::api_models::SaveLibraryPayload,
};

#[instrument(skip(app_state))]
pub async fn create_library_service(
    payload: SaveLibraryPayload,
    ws_client_key: String,
    app_state: Data<AppState>,
) -> Result<AsyncTaskResponse<i64>> {
    let database_addr = app_state.storage().database_addr().clone();
    let parser_addr = app_state.media().parser_addr();
    let ws_connections = app_state.communication().ws_connections();
    let task_pool = app_state.infrastructure().task_pool();
    let event_bus = app_state.infrastructure().event_bus();
    let library_repository = app_state.storage().repositories().library.clone();

    let directory_clone = payload.directory.clone();
    let library_name = payload.name.clone();

    let library_id = create_library(payload, library_repository.clone()).await?;
    debug!("Library created with id: {:?}", library_id);

    if library_id == SENTINEL_LIBRARY_ID {
        error!("Failed to create library");
        return Err(anyhow!("Failed to create library"));
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
            match_pattern: DomainEvent::Library(LibraryEventType::LibraryScanned { .. }),
            handler: move |event, event_bus| {
                let database_addr = database_addr.clone();
                let library_name = library_name.clone();

                async move {
                    if let DomainEvent::Library(LibraryEventType::LibraryScanned { library, task_identifier }) = event {
                        for media_item in library.tv_show {
                            debug!("Processing media item: {:?}", media_item.title);
                            insert_media_item(library_id, media_item, Arc::new(database_addr.clone()))
                                .await
                                .inspect_err(|e| error!("Failed to insert media item: {:?}", e))?;
                        }
                        event_bus.publish(DomainEvent::Library(LibraryEventType::LibrarySaved {
                            task_identifier,
                            library_id,
                            library_name,
                        }))?;
                    }
                    Ok(())
                }
            },
            config: EventHandlerConfig::one_time()
        },
        {
            match_pattern: DomainEvent::Library(LibraryEventType::LibrarySaved { .. }),
            handler: move |event, _| {
                let ws_connection_clone = ws_connection.clone();
                async move {
                    event.send_notification::<serde_json::Value>(ws_connection_clone)
                }
            },
            config: EventHandlerConfig::one_time()
        }
    );

    let mut task = LibraryScanTask::new(directory_clone, Arc::new(parser_addr.clone()));
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
        payload: Some(library_id),
    })
}
