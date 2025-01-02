use actix::{fut, prelude::*, Actor, Context, Handler, Message, WrapFuture};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use tracing::*;
use ts_rs::TS;

use crate::{
    define_actor_message_handler,
    domain::{
        episode::model::Episode,
        library::{
            constant::SENTINEL_LIBRARY_ID,
            model::{LibraryBrief, LibraryPoster},
        },
        media_actor::model::MediaActor,
        season::model::Season,
        tv_show::model::TvShow,
    },
    infrastructure::database::{
        category::query::check_category_exists,
        database::Database,
        episode::{create::save_episode, query::query_episodes},
        genre::create::save_genre,
        library::{
            create::save_library,
            delete::delete_library,
            query::{query_library, query_library_posters},
        },
        media_actor::create::save_actor,
        media_item::query::{
            query_all_media, query_all_media_items, query_media_by_id, query_series_by_library_id,
        },
        season::{create::save_season, query::query_seasons},
        tv_show::create::save_tv_show,
    },
    interfaces::{
        dtos::{EpisodeDto, MediaItemDto, SeasonDto},
        http_api::controllers::api_models::SaveLibraryPayload,
    },
    shared::util_traits::map_rows,
};

impl Actor for Database {
    type Context = Context<Self>;
}

#[derive(Debug, Serialize, Deserialize, TS, Message)]
#[rtype(result = "i64")]
pub struct SaveTvShow {
    pub tv_show: TvShow,
    pub library_id: i64,
}

impl Display for SaveTvShow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "WriteTvShowes({:?}, library_id: {})",
            self.tv_show.title, self.library_id
        )
    }
}

define_actor_message_handler!(
    message_type = SaveTvShow,
    return_type = i64,
    db_call = |pool, query_manager, msg: SaveTvShow| save_tv_show(
        pool,
        query_manager,
        msg.tv_show,
        msg.library_id
    ),
    success_return = |res| res,
    error_return = -1
);

#[derive(Debug, Serialize, Deserialize, TS, Message)]
#[rtype(result = "()")]
pub struct SaveGenre {
    pub tv_show_id: i64,
    pub genre: String,
}

impl Display for SaveGenre {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SaveGenre({})", self.genre)
    }
}

define_actor_message_handler!(
    message_type = SaveGenre,
    return_type = (),
    db_call = |pool, query_manager, msg: SaveGenre| save_genre(
        pool,
        query_manager,
        msg.tv_show_id,
        msg.genre
    ),
    success_return = |_| (),
    error_return = ()
);

#[derive(Debug, Serialize, Deserialize, TS, Message)]
#[rtype(result = "()")]
pub struct SaveActor(pub i64, pub MediaActor);

impl Display for SaveActor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SaveActor({})", self.0)
    }
}

define_actor_message_handler!(
    message_type = SaveActor,
    return_type = (),
    db_call = |pool, _, msg: SaveActor| save_actor(pool, msg.0, msg.1),
    success_return = |_| (),
    error_return = ()
);

#[derive(Debug, Serialize, Deserialize, TS, Message)]
#[rtype(result = "i64")]
pub struct SaveSeason {
    pub tv_show_id: i64,
    pub season_number: u8,
    pub season: Season,
}

impl Display for SaveSeason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SaveSeason(tv_show_id: {}, season_number: {}, title: {:?})",
            self.tv_show_id, self.season_number, self.season.title
        )
    }
}

define_actor_message_handler!(
    message_type = SaveSeason,
    return_type = i64,
    db_call = |pool, query_manager, msg: SaveSeason| save_season(
        pool,
        query_manager,
        msg.tv_show_id,
        msg.season_number,
        msg.season
    ),
    success_return = |res| res,
    error_return = -1
);

#[derive(Debug, Serialize, Deserialize, TS, Message)]
#[rtype(result = "()")]
pub struct SaveEpisode {
    pub season_id: i64,
    pub episode: Episode,
}

impl Display for SaveEpisode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SaveEpisode(season_id: {}, title: {:?})",
            self.season_id, self.episode.title
        )
    }
}

define_actor_message_handler!(
    message_type = SaveEpisode,
    return_type = (),
    db_call = |pool, query_manager, msg: SaveEpisode| save_episode(
        pool,
        query_manager,
        msg.season_id,
        msg.episode
    ),
    success_return = |_| (),
    error_return = ()
);

#[derive(Debug, Serialize, Deserialize, TS, Message)]
#[rtype(result = "Vec<MediaItemDto>")]
pub struct QueryMediaItemsByMediaLibraryId(pub i64);

impl Display for QueryMediaItemsByMediaLibraryId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GetMediaItemsByMediaLibraryId({})", self.0)
    }
}

define_actor_message_handler!(
    message_type = QueryMediaItemsByMediaLibraryId,
    return_type = Vec<MediaItemDto>,
    db_call = |pool, query_manager, msg: QueryMediaItemsByMediaLibraryId| query_series_by_library_id(pool, query_manager, |rows| map_rows(rows), msg.0),
    success_return = |res| res,
    error_return = Vec::<MediaItemDto>::new()
);

#[derive(Debug, Serialize, Deserialize, TS, Message)]
#[rtype(result = "Vec<MediaItemDto>")]
pub struct QueryAllMediaItems {
    pub library_id: i64,
}

impl Display for QueryAllMediaItems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GetAllMediaItems")
    }
}

define_actor_message_handler!(
    message_type = QueryAllMediaItems,
    return_type = Vec<MediaItemDto>,
    db_call = |pool, query_manager, _: QueryAllMediaItems| query_all_media_items(pool, query_manager, |rows| map_rows(rows)),
    success_return = |res| res,
    error_return = Vec::<MediaItemDto>::new()
);

#[derive(Debug, Serialize, Deserialize, TS, Message)]
#[rtype(result = "Vec<MediaItemDto>")]
pub struct QueryMediaById {
    pub library_id: i64,
    pub media_id: i64,
}

impl Display for QueryMediaById {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "GetMediaById(library_id: {}, media_id: {})",
            self.library_id, self.media_id
        )
    }
}

define_actor_message_handler!(
    message_type = QueryMediaById,
    return_type = Vec<MediaItemDto>,
    db_call = |pool, query_manager, msg: QueryMediaById| query_media_by_id(
        pool,
        query_manager,
        |rows| map_rows(rows),
        msg.library_id,
        msg.media_id
    ),
    success_return = |res| res,
    error_return = Vec::<MediaItemDto>::new()
);

#[derive(Debug, Serialize, Deserialize, TS, Message)]
#[rtype(result = "Vec<MediaItemDto>")]
pub struct QueryAllMedia {
    pub library_id: i64,
}

impl Display for QueryAllMedia {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GetAllMedia({})", self.library_id)
    }
}

define_actor_message_handler!(
    message_type = QueryAllMedia,
    return_type = Vec<MediaItemDto>,
    db_call = |pool, query_manager, msg: QueryAllMedia| query_all_media(pool, query_manager, |rows| map_rows(rows), msg.library_id),
    success_return = |res| res,
    error_return = Vec::<MediaItemDto>::new()
);

#[derive(Debug, Serialize, Deserialize, TS, Message)]
#[rtype(result = "Vec<EpisodeDto>")]
pub struct QueryEpisodes {
    pub library_id: i64,
    pub media_id: i64,
}

impl Display for QueryEpisodes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Querying episodes for media(library_id: {}, media_id: {})",
            self.library_id, self.media_id
        )
    }
}

define_actor_message_handler!(
    message_type = QueryEpisodes,
    return_type = Vec<EpisodeDto>,
    db_call = |pool, query_manager, msg: QueryEpisodes| query_episodes(
        pool,
        query_manager,
        |rows| map_rows(rows),
        msg.library_id,
        msg.media_id,
    ),
    success_return = |res| res,
    error_return = Vec::<EpisodeDto>::new()
);

#[derive(Debug, Serialize, Deserialize, TS, Message)]
#[rtype(result = "Vec<SeasonDto>")]
pub struct QuerySeasons(pub i64);

impl Display for QuerySeasons {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Querying seasons for TV show {}", self.0)
    }
}

define_actor_message_handler!(
    message_type = QuerySeasons,
    return_type = Vec<SeasonDto>,
    db_call = |pool, query_manager, msg: QuerySeasons| query_seasons(pool, query_manager, |rows| map_rows(rows), msg.0),
    success_return = |res| res,
    error_return = Vec::<SeasonDto>::new()
);

#[derive(Debug, Serialize, Deserialize, TS, Message)]
#[rtype(result = "i64")]
pub struct SaveLibrary {
    pub payload: SaveLibraryPayload,
}

impl Display for SaveLibrary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SaveLibrary({:?})", self.payload)
    }
}

define_actor_message_handler!(
    message_type = SaveLibrary,
    return_type = i64,
    db_call =
        |pool, query_manager, msg: SaveLibrary| save_library(pool, query_manager, msg.payload),
    success_return = |res| res,
    error_return = SENTINEL_LIBRARY_ID
);

#[derive(Debug, Serialize, Deserialize, TS, Message)]
#[rtype(result = "Vec<LibraryBrief>")]
pub struct QueryLibrary {
    pub id: Option<i64>,
}

impl Display for QueryLibrary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GetLibrary({:?})", self.id)
    }
}

define_actor_message_handler!(
    message_type = QueryLibrary,
    return_type = Vec<LibraryBrief>,
    db_call = |pool, query_manager, msg: QueryLibrary| query_library(pool, query_manager, |rows| map_rows(rows), msg.id),
    success_return = |res| res,
    error_return = Vec::<LibraryBrief>::new()
);

#[derive(Debug, Serialize, Deserialize, TS, Message)]
#[rtype(result = "Vec<LibraryPoster>")]
pub struct QueryLibraryPosters {
    pub library_id: i64,
}

impl Display for QueryLibraryPosters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GetLibraryPosters({})", self.library_id)
    }
}

define_actor_message_handler!(
    message_type = QueryLibraryPosters,
    return_type = Vec<LibraryPoster>,
    db_call = |pool, query_manager, msg: QueryLibraryPosters| query_library_posters(pool, query_manager, |rows| map_rows(rows), msg.library_id),
    success_return = |res| res,
    error_return = Vec::<LibraryPoster>::new()
);

#[derive(Debug, Serialize, Deserialize, TS, Message)]
#[rtype(result = "()")]
pub struct DeleteLibrary {
    pub id: i64,
}

impl Display for DeleteLibrary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DeleteLibrary({})", self.id)
    }
}

define_actor_message_handler!(
    message_type = DeleteLibrary,
    return_type = (),
    db_call = |pool, query_manager, msg: DeleteLibrary| delete_library(pool, query_manager, msg.id),
    success_return = |_| (),
    error_return = ()
);

#[derive(Debug, Serialize, Deserialize, TS, Message)]
#[rtype(result = "bool")]
pub struct ValidateCategory {
    pub category_id: i64,
}

impl Display for ValidateCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CheckCategoryExists({})", self.category_id)
    }
}

define_actor_message_handler!(
    message_type = ValidateCategory,
    return_type = bool,
    db_call = |pool, query_manager, msg: ValidateCategory| check_category_exists(
        pool,
        query_manager,
        msg.category_id
    ),
    success_return = |_| true,
    error_return = false
);
