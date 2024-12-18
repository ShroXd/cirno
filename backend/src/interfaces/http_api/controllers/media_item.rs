use actix::Addr;
use actix_web::{
    web::{Data, Path},
    HttpResponse, Responder,
};
use std::result::Result::Ok;
use tracing::*;

use crate::{
    domain::media_item::media_item::get_media_items, handle_controller_result,
    infrastructure::database::database::Database,
};

#[instrument(skip(database_addr))]
pub async fn get_media_items_controller(
    database_addr: Data<Addr<Database>>,
    id: Path<i64>,
) -> impl Responder {
    let media_library_id = id.into_inner();

    handle_controller_result!(
        get_media_items(Some(media_library_id), database_addr).await,
        HttpResponse::Ok(),
        HttpResponse::InternalServerError()
    )
}
