use actix_web::{
    delete, get, post, put,
    web::{scope, Data, Json, Path, ServiceConfig},
    HttpRequest, Responder,
};

use crate::{
    application::{file_service::FileService, pipeline_service::PipelineService},
    infrastructure::async_task_pool::task_pool::TaskPool,
    init::{app_state::AppState, repository_manager::Repositories},
    interfaces::{
        http_api::controllers::{
            api_models::SaveLibraryPayload,
            library::{
                create_library_controller, delete_library_controller, get_libraries_controller,
                get_library_by_id_controller, update_library_controller,
            },
            media_item::{
                get_all_media_controller, get_media_controller, get_media_episodes_controller,
            },
            video_player::{play_video_with_path_controller, stop_video_player_controller},
        },
        ws::utils::WsConnections,
    },
};

use super::controllers::api_models::{PlayVideoWithPathPayload, UpdateLibraryPayload};

// TODO: 1. move data models to database/models.rs
// TODO: 2. return error messages in the response
// TODO: 3. rename series to content or media etc.

// --------------------------------
// Library Routes
// --------------------------------

#[get("/{library_id}/media")]
async fn get_all_media_route(
    library_id: Path<i64>,
    repositories: Data<Repositories>,
) -> impl Responder {
    get_all_media_controller(library_id, repositories).await
}

#[get("/{library_id}/media/{media_id}")]
async fn get_media_route(
    path: Path<(i64, i64)>,
    repositories: Data<Repositories>,
) -> impl Responder {
    get_media_controller(path, repositories).await
}

#[get("/{library_id}/media/{media_id}/episodes")]
async fn get_media_episodes_route(
    path: Path<(i64, i64)>,
    repositories: Data<Repositories>,
) -> impl Responder {
    get_media_episodes_controller(path, repositories).await
}

#[post("/")]
async fn create_library_route(
    payload: Json<SaveLibraryPayload>,
    req: HttpRequest,
    app_state: Data<AppState>,
) -> impl Responder {
    create_library_controller(payload, req, app_state).await
}

#[put("/{library_id}")]
async fn update_library_route(
    library_id: Path<i64>,
    payload: Json<UpdateLibraryPayload>,
    repositories: Data<Repositories>,
) -> impl Responder {
    update_library_controller(library_id, payload, repositories).await
}

#[get("/")]
async fn get_libraries_route(repositories: Data<Repositories>) -> impl Responder {
    get_libraries_controller(repositories).await
}

#[get("/{id}")]
async fn get_library_route(id: Path<i64>, repositories: Data<Repositories>) -> impl Responder {
    get_library_by_id_controller(id, repositories).await
}

#[delete("/{id}")]
async fn delete_library_route(id: Path<i64>, repositories: Data<Repositories>) -> impl Responder {
    delete_library_controller(id, repositories).await
}

pub fn init_library_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/library")
            .service(get_all_media_route)
            .service(get_media_route)
            .service(get_media_episodes_route)
            .service(create_library_route)
            .service(update_library_route)
            .service(get_libraries_route)
            .service(get_library_route)
            .service(delete_library_route),
    );
}

// --------------------------------
// Video Player Routes
// --------------------------------

#[post("/play")]
async fn play_video_with_path(
    payload: Json<PlayVideoWithPathPayload>,
    req: HttpRequest,
    pipeline_service: Data<PipelineService>,
    file_service: Data<FileService>,
    task_pool: Data<TaskPool>,
    ws_connections: Data<WsConnections>,
) -> impl Responder {
    play_video_with_path_controller(
        payload,
        req,
        pipeline_service,
        file_service,
        task_pool,
        ws_connections,
    )
    .await
}

#[post("/stop")]
async fn stop_video_player_route(pipeline_service: Data<PipelineService>) -> impl Responder {
    stop_video_player_controller(pipeline_service).await
}

pub fn init_video_player_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/video-player")
            .service(play_video_with_path)
            .service(stop_video_player_route),
    );
}
