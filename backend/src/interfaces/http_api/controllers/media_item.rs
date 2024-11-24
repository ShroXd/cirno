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
    let media_library_id = match query.into_inner().media_library_id {
        Some(id) => id,
        // TODO: Currently using a default id since media library work is not complete
        // In the future, this will be replaced by a dedicated API endpoint that returns
        // curated content like "Latest" and "You May Like" sections for the main page
        // None => return HttpResponse::BadRequest().body("Media library id is required"),
        None => 0,
    };

    handle_controller_result!(
        get_media_items(media_library_id, database_addr).await,
        HttpResponse::Ok(),
        HttpResponse::InternalServerError()
    )
}
