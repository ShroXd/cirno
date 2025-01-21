use actix_web::{
    web::{Data, Json},
    HttpRequest, HttpResponse, Responder,
};
use std::sync::Arc;
use tracing::*;

use super::api_models::PlayVideoWithPathPayload;
use crate::{init::app_state::AppState, shared::utils::extract_ws_client_key};

#[instrument(skip(req, app_state))]
pub async fn play_video_with_path_controller(
    payload: Json<PlayVideoWithPathPayload>,
    req: HttpRequest,
    app_state: Data<AppState>,
) -> impl Responder {
    let ws_client_key = match extract_ws_client_key(&req) {
        Ok(key) => key,
        Err(e) => return HttpResponse::Unauthorized().json(e.to_string()),
    };

    let pipeline_service = app_state.media().pipeline_service();
    let file_service = app_state.storage().file_service();
    let task_pool = app_state.infrastructure().task_pool();
    let ws_connections = app_state.communication().ws_connections();

    match pipeline_service
        .start_playback(
            &payload.path,
            Arc::new(file_service.clone()),
            Arc::new(task_pool.clone()),
            ws_client_key.clone(),
            ws_connections.clone(),
        )
        .await
    {
        Ok(task_id) => HttpResponse::Ok().json(task_id),
        Err(e) => {
            error!("Failed to start playback: {:?}", e);
            HttpResponse::InternalServerError().json("Failed to start playback")
        }
    }
}

#[instrument(skip(app_state))]
pub async fn stop_video_player_controller(app_state: Data<AppState>) -> impl Responder {
    match app_state.media().pipeline_service().stop_and_clean().await {
        Ok(_) => HttpResponse::Ok().json("Pipeline stopped"),
        Err(e) => {
            error!("Failed to stop pipeline: {:?}", e);
            HttpResponse::InternalServerError().json("Failed to stop pipeline")
        }
    }
}
