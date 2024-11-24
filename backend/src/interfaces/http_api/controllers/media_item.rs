use actix::Addr;
use actix_web::{
    web::{Data, Query},
    HttpResponse, Responder,
};
use std::result::Result::Ok;
use tracing::*;

use super::api_models::GetMediaItemsQuery;
use crate::{
    database::database::Database, domain::media_item::media_item::get_media_items,
    handle_controller_result,
};

#[instrument(skip(database_addr))]
pub async fn get_media_items_controller(
    database_addr: Data<Addr<Database>>,
    query: Query<GetMediaItemsQuery>,
) -> impl Responder {
    let media_library_id = query.into_inner().media_library_id;

    handle_controller_result!(
        get_media_items(media_library_id, database_addr).await,
        HttpResponse::Ok(),
        HttpResponse::InternalServerError()
    )
}
