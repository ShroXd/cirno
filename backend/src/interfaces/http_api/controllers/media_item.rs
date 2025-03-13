use actix_web::{
    web::{Data, Path},
    HttpResponse, Responder,
};
use std::result::Result::Ok;
use tracing::*;

use crate::{handle_controller_result, init::app_state::AppState};

#[instrument(skip(app_state))]
pub async fn get_library_medias_controller(
    library_id: Path<i64>,
    app_state: Data<AppState>,
) -> impl Responder {
    handle_controller_result!(
        app_state
            .storage()
            .repositories()
            .media
            .get_library_medias(library_id.into_inner())
            .await,
        HttpResponse::Ok(),
        HttpResponse::InternalServerError()
    )
}

#[instrument(skip(app_state))]
pub async fn get_library_media_controller(
    path: Path<(i64, i64)>,
    app_state: Data<AppState>,
) -> impl Responder {
    let (library_id, media_id) = path.into_inner();
    handle_controller_result!(
        app_state
            .storage()
            .repositories()
            .media
            .get_library_media(library_id, media_id)
            .await,
        HttpResponse::Ok(),
        HttpResponse::InternalServerError()
    )
}

#[instrument(skip(app_state))]
pub async fn get_library_media_episodes_controller(
    path: Path<(i64, i64)>,
    app_state: Data<AppState>,
) -> impl Responder {
    let (library_id, media_id) = path.into_inner();

    handle_controller_result!(
        app_state
            .storage()
            .repositories()
            .media
            .get_library_media_episodes(library_id, media_id)
            .await,
        HttpResponse::Ok(),
        HttpResponse::InternalServerError()
    )
}

#[instrument(skip(app_state))]
pub async fn get_media_controller(path: Path<i64>, app_state: Data<AppState>) -> impl Responder {
    let media_id = path.into_inner();
    handle_controller_result!(
        app_state
            .storage()
            .repositories()
            .media
            .get_media_by_id(media_id)
            .await,
        HttpResponse::Ok(),
        HttpResponse::InternalServerError()
    )
}
