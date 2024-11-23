use actix::Addr;
use actix_web::web::Data;
use anyhow::Result;
use tracing::*;

use crate::{
    actors::database_actor::{QueryEpisodes, QuerySeasons},
    database::database::Database,
    interfaces::dtos::SeasonDto,
};

#[instrument(skip(database_addr))]
pub async fn get_tv_show_seasons(
    database_addr: Data<Addr<Database>>,
    id: i64,
) -> Result<Vec<SeasonDto>> {
    debug!("Getting seasons for TV show {}", id);
    let mut seasons = match database_addr.send(QuerySeasons(id)).await {
        Ok(seasons) => seasons,
        Err(e) => return Err(anyhow::anyhow!("Error getting seasons: {:?}", e)),
    };

    for season in seasons.iter_mut() {
        let season_number = season
            .season_number
            .ok_or_else(|| anyhow::anyhow!("Season number is missing"))?;
        debug!("Getting episodes for season {}", season_number);

        let episodes = database_addr
            .send(QueryEpisodes(id, season_number))
            .await
            .or_else(|e| Err(anyhow::anyhow!("Error getting episodes: {:?}", e)))?;
        season.episodes = episodes;
    }

    Ok(seasons)
}
