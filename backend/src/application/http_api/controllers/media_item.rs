use actix::Addr;
use actix_web::web::Data;
use anyhow::*;
use std::result::Result::Ok;
use tracing::*;

use crate::{
    actors::database_actor::GetSeries,
    database::{database::Database, query::TVSeriesDTO},
};

#[instrument(skip(database_addr))]
pub async fn get_media_item_controller(
    database_addr: Data<Addr<Database>>,
    id: Option<i64>,
) -> Result<Vec<TVSeriesDTO>> {
    match database_addr.send(GetSeries(id)).await {
        Ok(series) => {
            debug!("Got {} series", series.len());
            Ok(series)
        }
        Err(e) => {
            error!("Failed to get series: {:?}", e);
            return Err(anyhow!("Failed to get series"));
        }
    }
}
