use actix_web::{
    delete, get, post, put,
    web::{scope, Data, Json, Path, ServiceConfig},
    HttpRequest, Responder,
};

use crate::{
    init::app_state::AppState,
    interfaces::http_api::controllers::{
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
};

use super::controllers::api_models::{PlayVideoWithPathPayload, UpdateLibraryPayload};

// TODO: 1. move data models to database/models.rs
// TODO: 2. return error messages in the response
// TODO: 3. rename series to content or media etc.

// --------------------------------
// Library Routes
// --------------------------------

#[get("/{library_id}/media")]
async fn get_all_media_route(library_id: Path<i64>, app_state: Data<AppState>) -> impl Responder {
    get_all_media_controller(library_id, app_state).await
}

#[get("/{library_id}/media/{media_id}")]
async fn get_media_route(path: Path<(i64, i64)>, app_state: Data<AppState>) -> impl Responder {
    get_media_controller(path, app_state).await
}

#[get("/{library_id}/media/{media_id}/episodes")]
async fn get_media_episodes_route(
    path: Path<(i64, i64)>,
    app_state: Data<AppState>,
) -> impl Responder {
    get_media_episodes_controller(path, app_state).await
}

// TODO: fetch at most 10 media items randomly?

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
    app_state: Data<AppState>,
) -> impl Responder {
    update_library_controller(library_id, payload, app_state).await
}

#[get("/")]
async fn get_libraries_route(app_state: Data<AppState>) -> impl Responder {
    get_libraries_controller(app_state).await
}

#[get("/{id}")]
async fn get_library_route(id: Path<i64>, app_state: Data<AppState>) -> impl Responder {
    get_library_by_id_controller(id, app_state).await
}

#[delete("/{id}")]
async fn delete_library_route(id: Path<i64>, app_state: Data<AppState>) -> impl Responder {
    delete_library_controller(id, app_state).await
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
    app_state: Data<AppState>,
) -> impl Responder {
    play_video_with_path_controller(payload, req, app_state).await
}

#[post("/stop")]
async fn stop_video_player_route(app_state: Data<AppState>) -> impl Responder {
    stop_video_player_controller(app_state).await
}

pub fn init_video_player_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/video-player")
            .service(play_video_with_path)
            .service(stop_video_player_route),
    );
}
