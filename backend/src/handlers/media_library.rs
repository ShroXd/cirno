use actix::{spawn, Addr};
use actix_web::{delete, get, post, web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use tracing::*;
use ts_rs::TS;

use crate::{
    actors::{
        database_actor::{
            CreateMediaLibrary, DeleteMediaLibrary, GetMediaLibraries, GetSeasons, GetSeries,
            InsertSeries, SENTINEL_MEDIA_LIBRARY_ID,
        },
        parser_actor::{ParserActor, ScanMediaLibrary},
        utils::WsConnections,
        websocket_actor::{Notification, WebSocketActor},
    },
    database::database::Database,
};

// TODO: 1. move data models to database/models.rs
// TODO: 2. return error messages in the response
// TODO: 3. rename series to content or media etc.

#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[ts(export)]
pub struct GetSeriesQuery {
    pub media_library_id: Option<i64>,
}

#[get("/series")]
async fn get_series(
    database_addr: web::Data<Addr<Database>>,
    query: web::Query<GetSeriesQuery>,
) -> impl Responder {
    let media_library_id = query.into_inner().media_library_id;
    let series = database_addr
        .send(GetSeries(media_library_id))
        .await
        .expect("Failed to get series");

    HttpResponse::Ok().json(series)
}

#[get("/series/{id}/seasons")]
async fn get_seasons(
    database_addr: web::Data<Addr<Database>>,
    id: web::Path<i64>,
) -> impl Responder {
    let seasons = database_addr
        .send(GetSeasons(id.into_inner()))
        .await
        .expect("Failed to get seasons");
    HttpResponse::Ok().json(seasons)
}

#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[ts(export)]
pub struct CreateMediaLibraryPayload {
    pub name: String,
    pub directory: String,
    pub category: MediaLibraryCategory,
}

#[derive(Debug, Deserialize, Serialize, TS, Clone)]
#[ts(export)]
pub enum MediaLibraryCategory {
    Movie,
    TvShow,
    Animation,
}

impl From<MediaLibraryCategory> for i64 {
    fn from(category: MediaLibraryCategory) -> Self {
        match category {
            MediaLibraryCategory::Movie => 1,
            MediaLibraryCategory::TvShow => 2,
            MediaLibraryCategory::Animation => 3,
        }
    }
}

impl TryFrom<i64> for MediaLibraryCategory {
    type Error = String;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => MediaLibraryCategory::Movie,
            2 => MediaLibraryCategory::TvShow,
            3 => MediaLibraryCategory::Animation,
            _ => return Err("Invalid media library category".to_string()),
        })
    }
}

#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct CreateMediaLibraryResponse {
    pub id: i64,
}

#[post("/")]
async fn create_media_library(
    payload: web::Json<CreateMediaLibraryPayload>,
    database_addr: web::Data<Addr<Database>>,
    parser_addr: web::Data<Addr<ParserActor>>,
    ws_connections: web::Data<WsConnections>,
    req: HttpRequest,
) -> impl Responder {
    let ws_client_key = match req.headers().get("X-WS-CLIENT-KEY") {
        // TODO: handle the case where the key is not a string
        Some(key) => key.to_str().unwrap().to_string(),
        None => return HttpResponse::Unauthorized().json("Unauthorized"),
    };
    debug!("WS client key: {:?}", ws_client_key);

    let payload = payload.into_inner();
    match database_addr
        .send(CreateMediaLibrary(payload.clone()))
        .await
    {
        Ok(media_library_id) => {
            debug!("Media library created with id: {:?}", media_library_id);
            if media_library_id == SENTINEL_MEDIA_LIBRARY_ID {
                error!("Failed to create media library");
                HttpResponse::InternalServerError().json("Failed to create media library")
            } else {
                spawn(async move {
                    // Scan library, insert data into DB, and notify frontend via websocket
                    let directory = payload.directory.clone();
                    match parser_addr.send(ScanMediaLibrary(directory)).await {
                        Ok(result) => {
                            let media_library = result.expect("Failed to scan media library");
                            for serie in media_library.series {
                                match database_addr
                                    .send(InsertSeries(serie, media_library_id))
                                    .await
                                {
                                    Ok(_) => debug!("Series inserted"),
                                    Err(e) => error!("Failed to insert series: {:?}", e),
                                }
                            }

                            // Artificial delay to test frontend async UI behavior, will be removed
                            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

                            let ws_connections = ws_connections.get(ws_client_key).await;
                            if let Some(ws_connection) = ws_connections {
                                match ws_connection
                                    .send(Notification::MediaLibraryScanned(media_library_id))
                                    .await
                                {
                                    Ok(_) => debug!("Media library scanned notification sent"),
                                    Err(e) => error!("Failed to send notification: {:?}", e),
                                }
                            }
                        }
                        Err(e) => error!("Failed to scan media library: {:?}", e),
                    }
                });

                HttpResponse::Ok().json(CreateMediaLibraryResponse {
                    id: media_library_id,
                })
            }
        }
        Err(e) => {
            error!("Failed to create media library: {:?}", e);
            HttpResponse::InternalServerError().json("Failed to create media library")
        }
    }
}

#[get("/")]
async fn get_media_libraries(database_addr: web::Data<Addr<Database>>) -> impl Responder {
    let media_libraries = database_addr
        .send(GetMediaLibraries)
        .await
        .expect("Failed to get media libraries");

    HttpResponse::Ok().json(media_libraries)
}

#[delete("/{id}")]
async fn delete_media_library(
    id: web::Path<i64>,
    database_addr: web::Data<Addr<Database>>,
) -> impl Responder {
    database_addr
        .send(DeleteMediaLibrary(id.into_inner()))
        .await
        .expect("Failed to delete media library");

    HttpResponse::Ok().json(())
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/media-libraries")
            .service(get_series)
            .service(get_seasons)
            .service(create_media_library)
            .service(get_media_libraries)
            .service(delete_media_library),
    );
}
