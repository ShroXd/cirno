use actix::Addr;
use actix_web::web::Data;
use anyhow::Result;
use tracing::*;

use crate::{
    actors::database_actor::GetSeasons, database::database::Database, interfaces::dtos::SeasonDto,
};

#[instrument(skip(database_addr))]
pub async fn get_tv_show_seasons(
    database_addr: Data<Addr<Database>>,
    id: i64,
) -> Result<Vec<SeasonDto>> {
    database_addr
        .send(GetSeasons(id))
        .await
        .map_err(|e| anyhow::anyhow!("Error getting seasons: {:?}", e))
}
