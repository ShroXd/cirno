use actix_web::{
    web::{Data, Path},
    HttpResponse, Responder,
};
use std::result::Result::Ok;
use tracing::*;

use crate::{handle_controller_result, init::repository_manager::Repositories};

#[instrument(skip(repositories))]
pub async fn get_all_media_controller(
    library_id: Path<i64>,
    repositories: Data<Repositories>,
) -> impl Responder {
    handle_controller_result!(
        repositories
            .media
            .get_all_media(library_id.into_inner())
            .await,
        HttpResponse::Ok(),
        HttpResponse::InternalServerError()
    )
}

#[instrument(skip(repositories))]
pub async fn get_media_controller(
    path: Path<(i64, i64)>,
    repositories: Data<Repositories>,
) -> impl Responder {
    let (library_id, media_id) = path.into_inner();
    handle_controller_result!(
        repositories
            .media
            .get_media_by_id(library_id, media_id)
            .await,
        HttpResponse::Ok(),
        HttpResponse::InternalServerError()
    )
}

#[instrument(skip(repositories))]
pub async fn get_media_episodes_controller(
    path: Path<(i64, i64)>,
    repositories: Data<Repositories>,
) -> impl Responder {
    let (library_id, media_id) = path.into_inner();

    handle_controller_result!(
        repositories
            .media
            .get_media_episodes(library_id, media_id)
            .await,
        HttpResponse::Ok(),
        HttpResponse::InternalServerError()
    )
}
