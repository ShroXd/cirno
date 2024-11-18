use actix::Addr;
use actix_web::{
    delete, get, post,
    web::{scope, Data, Json, Path, Query, ServiceConfig},
    HttpRequest, HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{
    actors::{
        database_actor::{DeleteMediaLibrary, GetSeasons},
        parser_actor::ParserActor,
        utils::WsConnections,
    },
    application::http_api::controllers::{
        api_models::CreateMediaLibraryPayload,
        consts::WS_CLIENT_KEY_HEADER,
        media_item::get_media_item_controller,
        media_library::{create_media_library_controller, get_media_libraries_controller},
    },
    database::database::Database,
    handle_controller_result,
};

// TODO: 1. move data models to database/models.rs
// TODO: 2. return error messages in the response
// TODO: 3. rename series to content or media etc.

#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[ts(export)]
pub struct GetMediaItemsQuery {
    pub media_library_id: Option<i64>,
}

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
async fn get_seasons(database_addr: Data<Addr<Database>>, id: Path<i64>) -> impl Responder {
    let seasons = database_addr
        .send(GetSeasons(id.into_inner()))
        .await
        .expect("Failed to get seasons");

    HttpResponse::Ok().json(seasons)
}

#[post("/")]
async fn create_media_library_route(
    payload: Json<CreateMediaLibraryPayload>,
    database_addr: Data<Addr<Database>>,
    parser_addr: Data<Addr<ParserActor>>,
    ws_connections: Data<WsConnections>,
    req: HttpRequest,
) -> impl Responder {
    let ws_client_key = match req.headers().get(WS_CLIENT_KEY_HEADER) {
        // TODO: handle the case where the key is not a string
        Some(key) => key.to_str().unwrap().to_string(),
        None => return HttpResponse::Unauthorized().json("Unauthorized"),
    };
    let payload = payload.into_inner();

    handle_controller_result!(
        create_media_library_controller(
            payload,
            database_addr,
            parser_addr,
            ws_connections,
            ws_client_key,
        )
        .await,
        HttpResponse::Ok(),
        HttpResponse::InternalServerError()
    )
}

#[get("/")]
async fn get_media_libraries_route(database_addr: Data<Addr<Database>>) -> impl Responder {
    handle_controller_result!(
        get_media_libraries_controller(database_addr).await,
        HttpResponse::Ok(),
        HttpResponse::InternalServerError()
    )
}

#[delete("/{id}")]
async fn delete_media_library_route(
    id: Path<i64>,
    database_addr: Data<Addr<Database>>,
) -> impl Responder {
    handle_controller_result!(
        database_addr
            .send(DeleteMediaLibrary(id.into_inner()))
            .await,
        HttpResponse::Ok(),
        HttpResponse::InternalServerError()
    )
}

pub fn init_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/media-libraries")
            .service(get_media_items_route)
            .service(get_seasons)
            .service(create_media_library_route)
            .service(get_media_libraries_route)
            .service(delete_media_library_route),
    );
}
