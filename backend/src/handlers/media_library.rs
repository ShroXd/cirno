use actix::Addr;
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{
    actors::database_actor::{CreateMediaLibrary, GetSeasons, GetSeries},
    database::database::Database,
};

// TODO: 1. move data models to database/models.rs
// TODO: 2. return error messages in the response

#[get("/series")]
async fn get_series(database_addr: web::Data<Addr<Database>>) -> impl Responder {
    let series = database_addr
        .send(GetSeries)
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

#[derive(Debug, Deserialize, Serialize, TS)]
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

impl From<MediaLibraryCategory> for i32 {
    fn from(category: MediaLibraryCategory) -> Self {
        match category {
            MediaLibraryCategory::Movie => 1,
            MediaLibraryCategory::TvShow => 2,
            MediaLibraryCategory::Animation => 3,
        }
    }
}

impl TryFrom<i32> for MediaLibraryCategory {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => MediaLibraryCategory::Movie,
            2 => MediaLibraryCategory::TvShow,
            3 => MediaLibraryCategory::Animation,
            _ => return Err("Invalid media library category".to_string()),
        })
    }
}

#[post("/create")]
async fn create_media_library(
    database_addr: web::Data<Addr<Database>>,
    payload: web::Json<CreateMediaLibraryPayload>,
) -> impl Responder {
    match database_addr
        .send(CreateMediaLibrary(payload.into_inner()))
        .await
    {
        Ok(_) => HttpResponse::Ok().json(()),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/media-library")
            .service(get_series)
            .service(get_seasons)
            .service(create_media_library),
    );
}
