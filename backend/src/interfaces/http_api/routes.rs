use actix::Addr;
use actix_web::{
    delete, get, post,
    web::{scope, Data, Json, Path, Query, ServiceConfig},
    HttpRequest, HttpResponse, Responder,
};

use crate::{
    actors::{parser_actor::ParserActor, utils::WsConnections},
    database::database::Database,
    handle_controller_result,
    interfaces::http_api::controllers::{
        api_models::{CreateMediaLibraryPayload, GetMediaItemsQuery},
        media_item::get_media_item_controller,
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
    let media_library_id = query.into_inner().media_library_id;

    handle_controller_result!(
        get_media_item_controller(database_addr, media_library_id).await,
        HttpResponse::Ok(),
        HttpResponse::InternalServerError()
    )
}

#[get("/series/{id}/seasons")]
async fn get_tv_show_seasons_route(
    database_addr: Data<Addr<Database>>,
    id: Path<i64>,
) -> impl Responder {
    let tv_show_id = id.into_inner();

    handle_controller_result!(
        get_tv_show_seasons_controller(database_addr, tv_show_id).await,
        HttpResponse::Ok(),
        HttpResponse::InternalServerError()
    )
}

#[post("/")]
async fn create_media_library_route(
    payload: Json<CreateMediaLibraryPayload>,
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
    delete_media_library_controller(id.into_inner(), database_addr).await
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
