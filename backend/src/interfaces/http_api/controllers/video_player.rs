use actix_web::{
    web::{Data, Json},
    HttpRequest, HttpResponse, Responder,
};
use tracing::*;

use super::api_models::PlayVideoWithPathPayload;
use crate::{
    application::{file_service::FileService, pipeline_service::PipelineService},
    infrastructure::async_task_pool::task_pool::TaskPool,
    interfaces::ws::utils::WsConnections,
    shared::utils::extract_ws_client_key,
};

#[instrument(skip(req, pipeline_service, file_service, task_pool, ws_connections,))]
pub async fn play_video_with_path_controller(
    payload: Json<PlayVideoWithPathPayload>,
    req: HttpRequest,
    pipeline_service: Data<PipelineService>,
    file_service: Data<FileService>,
    task_pool: Data<TaskPool>,
    ws_connections: Data<WsConnections>,
) -> impl Responder {
    let ws_client_key = match extract_ws_client_key(&req) {
        Ok(key) => key,
        Err(e) => return HttpResponse::Unauthorized().json(e.to_string()),
    };

    match pipeline_service
        .start_playback(
            &payload.path,
            file_service.into_inner(),
            task_pool.into_inner(),
            ws_client_key.clone(),
            ws_connections.get_ref().clone(),
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

#[instrument(skip(pipeline_service))]
pub async fn stop_video_player_controller(
    pipeline_service: Data<PipelineService>,
) -> impl Responder {
    match pipeline_service.stop_and_clean().await {
        Ok(_) => HttpResponse::Ok().json("Pipeline stopped"),
        Err(e) => {
            error!("Failed to stop pipeline: {:?}", e);
            HttpResponse::InternalServerError().json("Failed to stop pipeline")
        }
    }
}
