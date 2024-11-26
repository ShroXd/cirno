use actix::Addr;
use actix_web::{
    delete, get, post,
    web::{scope, Data, Json, Path, Query, ServiceConfig},
    HttpRequest, Responder,
};

use crate::{
    actors::utils::WsConnections,
    infrastructure::{database::database::Database, organizer::organizer::ParserActor},
    interfaces::http_api::controllers::{
        api_models::{GetMediaItemsQuery, SaveMediaLibraryPayload},
        media_item::get_media_items_controller,
        media_library::{
            create_media_library_controller, delete_media_library_controller,
            get_media_libraries_controller,
        },
        tv_show::get_tv_show_seasons_controller,
    },
};

// TODO: 1. move data models to database/models.rs
// TODO: 2. return error messages in the response
// TODO: 3. rename series to content or media etc.

#[get("/media-items")]
async fn get_media_items_route(
    database_addr: Data<Addr<Database>>,
    query: Query<GetMediaItemsQuery>,
) -> impl Responder {
    get_media_items_controller(database_addr, query).await
}

#[get("/series/{id}/seasons")]
async fn get_tv_show_seasons_route(
    database_addr: Data<Addr<Database>>,
    id: Path<i64>,
) -> impl Responder {
    get_tv_show_seasons_controller(database_addr, id).await
}

#[post("/")]
async fn create_media_library_route(
    payload: Json<SaveMediaLibraryPayload>,
    database_addr: Data<Addr<Database>>,
    parser_addr: Data<Addr<ParserActor>>,
    ws_connections: Data<WsConnections>,
    req: HttpRequest,
) -> impl Responder {
    create_media_library_controller(payload, database_addr, parser_addr, ws_connections, req).await
}

#[get("/")]
async fn get_media_libraries_route(database_addr: Data<Addr<Database>>) -> impl Responder {
    get_media_libraries_controller(database_addr).await
}

#[delete("/{id}")]
async fn delete_media_library_route(
    id: Path<i64>,
    database_addr: Data<Addr<Database>>,
) -> impl Responder {
    delete_media_library_controller(id, database_addr).await
}

pub fn init_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/media-libraries")
            .service(get_media_items_route)
            .service(get_tv_show_seasons_route)
            .service(create_media_library_route)
            .service(get_media_libraries_route)
            .service(delete_media_library_route),
    );
}
