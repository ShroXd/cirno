use actix::Addr;
use actix_web::{
    web::{Data, Json},
    HttpRequest, HttpResponse, Responder,
};
use std::sync::Arc;
use tracing::*;

use super::api_models::PlayVideoWithPathPayload;
use crate::{
    application::{file_service::FileService, pipeline_service::PipelineService},
    infrastructure::{
        event_bus::event_bus::EventBus, hls::hls_state_actor::HlsStateActor,
        task_pool::task_pool::TaskPool,
    },
    interfaces::ws::utils::WsConnections,
    shared::utils::extract_ws_client_key,
};

#[instrument(skip(req, pipeline_service, file_service, event_bus, task_pool))]
pub async fn play_video_with_path_controller(
    payload: Json<PlayVideoWithPathPayload>,
    req: HttpRequest,
    pipeline_service: Data<PipelineService>,
    file_service: Data<FileService>,
    event_bus: Data<Arc<EventBus>>,
    hls_state_actor_addr: Data<Addr<HlsStateActor>>,
    task_pool: Data<TaskPool>,
    ws_connections: Data<WsConnections>,
) -> impl Responder {
    let ws_client_key = match extract_ws_client_key(&req) {
        Ok(key) => key,
        Err(e) => return HttpResponse::Unauthorized().json(e.to_string()),
    };

    let task_id = pipeline_service
        .start_playback(
            &payload.path,
            file_service.into_inner(),
            event_bus.get_ref().clone(),
            task_pool.into_inner(),
            ws_client_key.clone(),
            ws_connections.get_ref().clone(),
        )
        .await
        .unwrap();

    HttpResponse::Ok().json(task_id)
}
