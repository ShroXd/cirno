use actix::Addr;
use anyhow::Result;
use std::sync::Arc;
use tracing::*;

use crate::{
    domain::{episode::model::Episode, season::model::Season, tv_show::model::TvShow},
    infrastructure::database::{
        actor::{SaveActor, SaveEpisode, SaveGenre, SaveSeason, SaveTvShow},
        database::Database,
    },
};

#[instrument(skip(media_item, database_addr))]
pub async fn insert_media_item(
    library_id: i64,
    media_item: TvShow,
    database_addr: Arc<Addr<Database>>,
) -> Result<()> {
    debug!("Inserting media item: {:?}", media_item.title);

    let genres = media_item.genres.clone();
    let actors = media_item.actors.clone();
    let seasons_map = media_item.seasons.clone();
    let seasons = seasons_map.values().collect::<Vec<&Season>>();

    debug!("Inserting tv show: {:?}", media_item.title);
    let tv_show_id = database_addr
        .send(SaveTvShow(media_item, library_id))
        .await
        .map_err(|e| anyhow::anyhow!("Error inserting media item: {:?}", e))?;

    for genre in genres {
        debug!("Inserting genre: {}", genre);
        database_addr
            .send(SaveGenre(tv_show_id, genre))
            .await
            .map_err(|e| anyhow::anyhow!("Error inserting genre: {:?}", e))?;
    }

    for actor in actors {
        debug!("Inserting actor: {:?}", &actor.name);
        database_addr
            .send(SaveActor(tv_show_id, actor))
            .await
            .map_err(|e| anyhow::anyhow!("Error inserting actor: {:?}", e))?;
    }

    for season in seasons {
        debug!("Inserting season: {:?}", season.title);

        let episodes = season.episodes.values().collect::<Vec<&Episode>>();

        let season_number = match season.season_number {
            Some(num) => num,
            None => continue,
        };

        let season_id = database_addr
            .send(SaveSeason(tv_show_id, season_number, season.to_owned()))
            .await
            .map_err(|e| anyhow::anyhow!("Error inserting season: {:?}", e))?;

        for episode in episodes {
            debug!("Inserting episode: {:?}", episode.title);
            database_addr
                .send(SaveEpisode(tv_show_id, season_id, episode.to_owned()))
                .await
                .map_err(|e| anyhow::anyhow!("Error inserting episode: {:?}", e))?;
        }
    }

    Ok(())
}
