use actix::Addr;
use actix_web::{
    web::{Data, Json, Path},
    HttpRequest, HttpResponse, Responder,
};
use std::result::Result::Ok;
use tracing::*;

use super::api_models::SaveMediaLibraryPayload;
use crate::{
    actors::utils::WsConnections,
    application::media_library_service::create_media_library_service,
    domain::media_library::media_library::{delete_media_library, get_media_libraries},
    handle_controller_result,
    infrastructure::{database::database::Database, organizer::organizer::ParserActor},
    interfaces::http_api::controllers::consts::WS_CLIENT_KEY_HEADER,
};

pub async fn create_media_library_controller(
    payload: Json<SaveMediaLibraryPayload>,
    database_addr: Data<Addr<Database>>,
    parser_addr: Data<Addr<ParserActor>>,
    ws_connections: Data<WsConnections>,
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
            ws_client_key,
        )
        .await,
        HttpResponse::Ok(),
        HttpResponse::InternalServerError()
    )
}

#[instrument(skip(database_addr))]
pub async fn get_media_libraries_controller(database_addr: Data<Addr<Database>>) -> impl Responder {
    debug!("Getting all media libraries");

    handle_controller_result!(
        get_media_libraries(database_addr).await,
        HttpResponse::Ok(),
        HttpResponse::InternalServerError()
    )
}

#[instrument(skip(database_addr))]
pub async fn delete_media_library_controller(
    id: Path<i64>,
    database_addr: Data<Addr<Database>>,
) -> impl Responder {
    let media_library_id = id.into_inner();
    debug!("Deleting media library for id: {}", media_library_id);

    handle_controller_result!(
        delete_media_library(media_library_id, database_addr).await,
        HttpResponse::Ok(),
        HttpResponse::InternalServerError()
    )
}
