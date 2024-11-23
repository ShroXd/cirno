use actix::Addr;
use actix_web::web::Path;
use actix_web::Responder;
use actix_web::{web::Data, HttpResponse};
use tracing::*;

use crate::{
    database::database::Database, domain::tv_show::tv_show::get_tv_show_seasons,
    handle_controller_result,
};

#[instrument(skip(database_addr))]
pub async fn get_tv_show_seasons_controller(
    database_addr: Data<Addr<Database>>,
    id: Path<i64>,
) -> impl Responder {
    let tv_show_id = id.into_inner();
    debug!("Getting TV show seasons for id: {}", tv_show_id);

    handle_controller_result!(
        get_tv_show_seasons(database_addr, tv_show_id).await,
        HttpResponse::Ok(),
        HttpResponse::InternalServerError()
    )
}
