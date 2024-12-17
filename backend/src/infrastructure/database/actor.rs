use actix::{fut, prelude::*, Actor, Context, Handler, Message, WrapFuture};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use tracing::*;
use ts_rs::TS;

use crate::{
    define_actor_message_handler,
    domain::{
        episode::model::Episode,
        media_actor::model::MediaActor,
        media_library::{
            constant::SENTINEL_MEDIA_LIBRARY_ID,
            model::{MediaLibraryBrief, MediaLibraryPoster},
        },
        season::model::Season,
        tv_show::model::TvShow,
    },
    infrastructure::database::{
        category::query::check_category_exists,
        database::Database,
        episode::{create::save_episode, query::query_episodes},
        genre::create::save_genre,
        media_actor::create::save_actor,
        media_item::query::{query_all_media_items, query_series_by_media_library_id},
        media_library::{
            create::save_media_library,
            delete::delete_media_library,
            query::{query_media_libraries, query_media_library_posters},
        },
        season::{create::save_season, query::query_seasons},
        tv_show::create::save_tv_show,
    },
    interfaces::{
        dtos::{EpisodeDto, MediaItemDto, SeasonDto},
        http_api::controllers::api_models::SaveMediaLibraryPayload,
    },
    shared::util_traits::map_rows,
};

impl Actor for Database {
    type Context = Context<Self>;
}

#[derive(Debug, Serialize, Deserialize, TS, Message)]
#[rtype(result = "i64")]
pub struct SaveTvShow(pub TvShow, pub i64);

impl Display for SaveTvShow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "WriteTvShowes({:?}, media_library_id: {})",
            self.0.title, self.1
        )
    }
}

define_actor_message_handler!(
    message_type = SaveTvShow,
    return_type = i64,
    db_call = |pool, _, msg: SaveTvShow| save_tv_show(pool, msg.0, msg.1),
    success_return = |res| res,
    error_return = -1
);

#[derive(Debug, Serialize, Deserialize, TS, Message)]
#[rtype(result = "()")]
pub struct SaveGenre(pub i64, pub String);

impl Display for SaveGenre {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SaveGenre({})", self.0)
    }
}

define_actor_message_handler!(
    message_type = SaveGenre,
    return_type = (),
    db_call = |pool, _, msg: SaveGenre| save_genre(pool, msg.0, msg.1),
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
pub struct SaveSeason(pub i64, pub u8, pub Season);

impl Display for SaveSeason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SaveSeason(tv_show_id: {}, season_number: {}, title: {:?})",
            self.0, self.1, self.2.title
        )
    }
}

define_actor_message_handler!(
    message_type = SaveSeason,
    return_type = i64,
    db_call = |pool, _, msg: SaveSeason| save_season(pool, msg.0, msg.1, msg.2),
    success_return = |res| res,
    error_return = -1
);

#[derive(Debug, Serialize, Deserialize, TS, Message)]
#[rtype(result = "()")]
pub struct SaveEpisode(pub i64, pub i64, pub u8, pub Episode);

impl Display for SaveEpisode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SaveEpisode(tv_show_id: {}, season_id: {}, season_number: {}, title: {:?})",
            self.0, self.1, self.2, self.3.title
        )
    }
}

define_actor_message_handler!(
    message_type = SaveEpisode,
    return_type = (),
    db_call = |pool, _, msg: SaveEpisode| save_episode(pool, msg.0, msg.1, msg.2, msg.3),
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
    db_call = |pool, _, msg: QueryMediaItemsByMediaLibraryId| query_series_by_media_library_id(pool, msg.0, |rows| map_rows(rows)),
    success_return = |res| res,
    error_return = Vec::<MediaItemDto>::new()
);

#[derive(Debug, Serialize, Deserialize, TS, Message)]
#[rtype(result = "Vec<MediaItemDto>")]
pub struct QueryAllMediaItems;

impl Display for QueryAllMediaItems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GetAllMediaItems")
    }
}

define_actor_message_handler!(
    message_type = QueryAllMediaItems,
    return_type = Vec<MediaItemDto>,
    db_call = |pool, _, _: QueryAllMediaItems| query_all_media_items(pool, |rows| map_rows(rows)),
    success_return = |res| res,
    error_return = Vec::<MediaItemDto>::new()
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
    db_call = |pool, _, msg: QuerySeasons| query_seasons(pool, msg.0, |rows| map_rows(rows)),
    success_return = |res| res,
    error_return = Vec::<SeasonDto>::new()
);

#[derive(Debug, Serialize, Deserialize, TS, Message)]
#[rtype(result = "Vec<EpisodeDto>")]
pub struct QueryEpisodes(pub i64, pub i64);

impl Display for QueryEpisodes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Querying episodes for TV show {} and season {}",
            self.0, self.1
        )
    }
}

define_actor_message_handler!(
    message_type = QueryEpisodes,
    return_type = Vec<EpisodeDto>,
    db_call = |pool, _, msg: QueryEpisodes| query_episodes(pool, msg.0, msg.1),
    success_return = |res| res,
    error_return = Vec::<EpisodeDto>::new()
);

#[derive(Debug, Serialize, Deserialize, TS, Message)]
#[rtype(result = "i64")]
pub struct SaveMediaLibrary {
    pub payload: SaveMediaLibraryPayload,
}

impl Display for SaveMediaLibrary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CreateMediaLibrary({:?})", self.payload)
    }
}

define_actor_message_handler!(
    message_type = SaveMediaLibrary,
    return_type = i64,
    db_call = |pool, _, msg: SaveMediaLibrary| save_media_library(pool, msg.payload),
    success_return = |res| res,
    error_return = SENTINEL_MEDIA_LIBRARY_ID
);

#[derive(Debug, Serialize, Deserialize, TS, Message)]
#[rtype(result = "Vec<MediaLibraryBrief>")]
pub struct QueryMediaLibraries;
impl Display for QueryMediaLibraries {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GetMediaLibraries")
    }
}

define_actor_message_handler!(
    message_type = QueryMediaLibraries,
    return_type = Vec<MediaLibraryBrief>,
    db_call = |pool, query_manager, _: QueryMediaLibraries| query_media_libraries(pool, query_manager, |rows| map_rows(rows)),
    success_return = |res| res,
    error_return = Vec::<MediaLibraryBrief>::new()
);

#[derive(Debug, Serialize, Deserialize, TS, Message)]
#[rtype(result = "Vec<MediaLibraryPoster>")]
pub struct QueryMediaLibraryPosters {
    pub media_library_id: i64,
}

impl Display for QueryMediaLibraryPosters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GetMediaLibraryPosters({})", self.media_library_id)
    }
}

define_actor_message_handler!(
    message_type = QueryMediaLibraryPosters,
    return_type = Vec<MediaLibraryPoster>,
    db_call = |pool, query_manager, msg: QueryMediaLibraryPosters| query_media_library_posters(msg.media_library_id, pool, query_manager, |rows| map_rows(rows)),
    success_return = |res| res,
    error_return = Vec::<MediaLibraryPoster>::new()
);

#[derive(Debug, Serialize, Deserialize, TS, Message)]
#[rtype(result = "()")]
pub struct DeleteMediaLibrary {
    pub id: i64,
}

impl Display for DeleteMediaLibrary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DeleteMediaLibrary({})", self.id)
    }
}

define_actor_message_handler!(
    message_type = DeleteMediaLibrary,
    return_type = (),
    db_call = |pool, _, msg: DeleteMediaLibrary| delete_media_library(pool, msg.id),
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
