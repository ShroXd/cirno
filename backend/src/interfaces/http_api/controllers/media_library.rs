use actix::Addr;
use actix_web::{
    web::{Data, Json, Path},
    HttpRequest, HttpResponse, Responder,
};
use std::{result::Result::Ok, sync::Arc};
use tracing::*;

use super::api_models::SaveMediaLibraryPayload;
use crate::{
    application::media_library_service::create_media_library_service,
    domain::media_library::media_library::{delete_media_library, get_media_libraries},
    handle_controller_result,
    infrastructure::{
        database::database::Database, event_bus::event_bus::EventBus,
        organizer::organizer::ParserActor, task_pool::task_pool::TaskPool,
    },
    init::repository_manager::Repositories,
    interfaces::{http_api::controllers::consts::WS_CLIENT_KEY_HEADER, ws::utils::WsConnections},
};

pub async fn create_media_library_controller(
    payload: Json<SaveMediaLibraryPayload>,
    database_addr: Data<Addr<Database>>,
    parser_addr: Data<Addr<ParserActor>>,
    ws_connections: Data<WsConnections>,
    task_pool: Data<TaskPool>,
    event_bus: Data<Arc<EventBus>>,
    repositories: Data<Repositories>,
    req: HttpRequest,
) -> impl Responder {
    let ws_client_key = match req.headers().get(WS_CLIENT_KEY_HEADER) {
        // TODO: handle the case where the key is not a string
        Some(key) => key.to_str().unwrap().to_string(),
        None => return HttpResponse::Unauthorized().json("Unauthorized"),
    };
    let payload = payload.into_inner();

    debug!(
        "Creating media library with name: {}, directory: {}, category: {:?}",
        payload.name, payload.directory, payload.category
    );

    handle_controller_result!(
        create_media_library_service(
            payload,
            database_addr,
            parser_addr,
            ws_connections,
            task_pool,
            event_bus,
            ws_client_key,
            repositories.media_library.clone()
        )
        .await,
        HttpResponse::Ok(),
        HttpResponse::InternalServerError()
    )
}

#[instrument(skip(repositories))]
pub async fn get_media_libraries_controller(repositories: Data<Repositories>) -> impl Responder {
    debug!("Getting all media libraries");

    handle_controller_result!(
        get_media_libraries(repositories.media_library.clone()).await,
        HttpResponse::Ok(),
        HttpResponse::InternalServerError()
    )
}

#[instrument(skip(repositories))]
pub async fn delete_media_library_controller(
    id: Path<i64>,
    repositories: Data<Repositories>,
) -> impl Responder {
    let media_library_id = id.into_inner();
    debug!("Deleting media library for id: {}", media_library_id);

    handle_controller_result!(
        delete_media_library(media_library_id, repositories.media_library.clone()).await,
        HttpResponse::Ok(),
        HttpResponse::InternalServerError()
    )
}
