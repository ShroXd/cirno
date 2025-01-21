use actix_web::{
    web::{Data, Json, Path},
    HttpRequest, HttpResponse, Responder,
};
use std::result::Result::Ok;
use tracing::*;

use super::api_models::{SaveLibraryPayload, UpdateLibraryPayload};
use crate::{
    application::library_service::create_library_service,
    domain::media_library::library::{delete_library, get_libraries, get_library_by_id},
    handle_controller_result,
    init::app_state::AppState,
    interfaces::http_api::controllers::consts::WS_CLIENT_KEY_HEADER,
};

pub async fn create_library_controller(
    payload: Json<SaveLibraryPayload>,
    req: HttpRequest,
    app_state: Data<AppState>,
) -> impl Responder {
    let ws_client_key = match req.headers().get(WS_CLIENT_KEY_HEADER) {
        // TODO: handle the case where the key is not a string
        Some(key) => key.to_str().unwrap().to_string(),
        None => return HttpResponse::Unauthorized().json("Unauthorized"),
    };
    let payload = payload.into_inner();

    debug!(
        "Creating library with name: {}, directory: {}, category: {:?}",
        payload.name, payload.directory, payload.category
    );

    handle_controller_result!(
        create_library_service(payload, ws_client_key, app_state).await,
        HttpResponse::Ok(),
        HttpResponse::InternalServerError()
    )
}

#[instrument(skip(app_state))]
pub async fn update_library_controller(
    library_id: Path<i64>,
    payload: Json<UpdateLibraryPayload>,
    app_state: Data<AppState>,
) -> impl Responder {
    debug!("Updating library for id: {}", library_id);
    handle_controller_result!(
        app_state
            .storage()
            .repositories()
            .library
            .update_library(library_id.into_inner(), payload.into_inner())
            .await,
        HttpResponse::Ok(),
        HttpResponse::InternalServerError()
    )
}

#[instrument(skip(app_state))]
pub async fn get_libraries_controller(app_state: Data<AppState>) -> impl Responder {
    debug!("Getting all libraries");
    handle_controller_result!(
        get_libraries(app_state.storage().repositories().library.clone()).await,
        HttpResponse::Ok(),
        HttpResponse::NotFound()
    )
}

#[instrument(skip(app_state))]
pub async fn get_library_by_id_controller(
    id: Path<i64>,
    app_state: Data<AppState>,
) -> impl Responder {
    debug!("Getting library for id: {}", id);
    handle_controller_result!(
        get_library_by_id(
            id.into_inner(),
            app_state.storage().repositories().library.clone()
        )
        .await,
        HttpResponse::Ok(),
        HttpResponse::NotFound()
    )
}

#[instrument(skip(app_state))]
pub async fn delete_library_controller(id: Path<i64>, app_state: Data<AppState>) -> impl Responder {
    let library_id = id.into_inner();
    debug!("Deleting library for id: {}", library_id);

    handle_controller_result!(
        delete_library(
            library_id,
            app_state.storage().repositories().library.clone()
        )
        .await,
        HttpResponse::Ok(),
        HttpResponse::InternalServerError()
    )
}
