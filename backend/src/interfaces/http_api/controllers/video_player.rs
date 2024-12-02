use actix_web::{
    web::{Data, Json},
    HttpRequest, HttpResponse, Responder,
};
use tracing::*;

use crate::{application::pipeline_service::PipelineService, shared::utils::extract_ws_client_key};

use super::api_models::PlayVideoWithPathPayload;

#[instrument(skip(req, pipeline_service))]
pub async fn play_video_with_path_controller(
    payload: Json<PlayVideoWithPathPayload>,
    req: HttpRequest,
    pipeline_service: Data<PipelineService>,
) -> impl Responder {
    // let ws_client_key = match extract_ws_client_key(&req) {
    //     Ok(key) => key,
    //     Err(e) => return HttpResponse::Unauthorized().json(e.to_string()),
    // };
    let ws_client_key = "test";

    pipeline_service
        .start_playback(&payload.path)
        .await
        .unwrap();

    // Register the async task with the event bus
    // Return task id

    HttpResponse::Ok().json(ws_client_key)
}
