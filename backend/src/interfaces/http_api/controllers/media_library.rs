use actix::Addr;
use actix_web::{
    web::{Data, Json},
    HttpRequest, HttpResponse, Responder,
};
use anyhow::*;
use std::result::Result::Ok;
use tracing::*;

use super::api_models::CreateMediaLibraryPayload;
use crate::{
    actors::{
        database_actor::{DeleteMediaLibrary, GetMediaLibraries},
        parser_actor::ParserActor,
        utils::WsConnections,
    },
    application::media_library_service::create_media_library_service,
    database::database::Database,
    handle_controller_result,
    interfaces::{dtos::MediaLibraryDto, http_api::controllers::consts::WS_CLIENT_KEY_HEADER},
};

pub async fn create_media_library_controller(
    payload: Json<CreateMediaLibraryPayload>,
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
pub async fn get_media_libraries_controller(
    database_addr: Data<Addr<Database>>,
) -> Result<Vec<MediaLibraryDto>> {
    match database_addr.send(GetMediaLibraries).await {
        Ok(media_libraries) => {
            debug!("Found {} media libraries", media_libraries.len());
            Ok(media_libraries)
        }
        Err(e) => {
            error!("Failed to get media libraries: {:?}", e);
            return Err(anyhow!("Failed to get media libraries"));
        }
    }
}

#[instrument(skip(database_addr))]
pub async fn delete_media_library_controller(
    id: i64,
    database_addr: Data<Addr<Database>>,
) -> Result<()> {
    match database_addr.send(DeleteMediaLibrary(id)).await {
        Ok(_) => {
            debug!("Deleted media library with id {}", id);
            Ok(())
        }
        Err(e) => {
            error!("Failed to delete media library: {:?}", e);
            return Err(anyhow!("Failed to delete media library"));
        }
    }
}
