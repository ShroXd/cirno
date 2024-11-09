use actix::Addr;
use actix_web::{get, web, HttpResponse, Responder};

use crate::{
    actors::database_actor::{GetSeasons, GetSeries},
    database::database::Database,
};

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

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_series);
    cfg.service(get_seasons);
}
