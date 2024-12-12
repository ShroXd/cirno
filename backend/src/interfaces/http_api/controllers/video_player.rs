use actix_web::{
    web::{Data, Json},
    HttpRequest, HttpResponse, Responder,
};
use tracing::*;

use super::api_models::PlayVideoWithPathPayload;
use crate::application::{file_service::FileService, pipeline_service::PipelineService};

#[instrument(skip(req, pipeline_service, file_service))]
pub async fn play_video_with_path_controller(
    payload: Json<PlayVideoWithPathPayload>,
    req: HttpRequest,
    pipeline_service: Data<PipelineService>,
    file_service: Data<FileService>,
) -> impl Responder {
    // let ws_client_key = match extract_ws_client_key(&req) {
    //     Ok(key) => key,
    //     Err(e) => return HttpResponse::Unauthorized().json(e.to_string()),
    // };
    let ws_client_key = "test";

    pipeline_service
        .start_playback(&payload.path, file_service.into_inner())
        .await
        .unwrap();

    // Register the async task with the event bus
    // Return task id

    HttpResponse::Ok().json(ws_client_key)
}
