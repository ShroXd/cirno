use actix::Addr;
use actix_web::web::Data;
use anyhow::*;
use std::result::Result::Ok;
use tracing::*;

use crate::{
    actors::database_actor::GetSeasons,
    database::{database::Database, query::SeasonDTO},
};

#[instrument(skip(database_addr))]
pub async fn get_tv_show_seasons_controller(
    database_addr: Data<Addr<Database>>,
    id: i64,
) -> Result<Vec<SeasonDTO>> {
    match database_addr.send(GetSeasons(id)).await {
        Ok(seasons) => {
            debug!("Got {} tv show seasons", seasons.len());
            Ok(seasons)
        }
        Err(e) => {
            error!("Failed to get tv show seasons: {:?}", e);
            return Err(anyhow!("Failed to get seasons"));
        }
    }
}
