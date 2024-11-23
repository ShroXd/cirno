use actix::{fut, prelude::*, Actor, Context, Handler, Message, WrapFuture};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use tracing::*;
use ts_rs::TS;

use crate::{
    database::{
        create::{insert_media_library, insert_tv_series},
        database::Database,
        delete::delete_media_library,
        query::{
            check_category_exists, query_media_libraries, query_seasons_with_episodes, query_series,
        },
    },
    define_actor_message_handler,
    interfaces::{
        dtos::{MediaItemDto, MediaLibraryDto, SeasonDto},
        http_api::controllers::api_models::CreateMediaLibraryPayload,
    },
    services::library_parser::parsers::TVSerie,
};

impl Actor for Database {
    type Context = Context<Self>;
}

// TODO: extend this to enum until we have more types to be inserted
#[derive(Debug, Serialize, Deserialize, TS, Message)]
#[rtype(result = "()")]
pub struct InsertSeries(pub TVSerie, pub i64);

impl Display for InsertSeries {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InsertSeries({:?}, {})", self.0.title, self.1)
    }
}

define_actor_message_handler!(
    message_type = InsertSeries,
    return_type = (),
    db_call = |pool, msg: InsertSeries| insert_tv_series(pool, msg.0, msg.1),
    success_return = |_| (),
    error_return = ()
);

#[derive(Debug, Serialize, Deserialize, TS, Message)]
#[rtype(result = "Vec<MediaItemDto>")]
pub struct GetMediaItems(pub Option<i64>);

impl Display for GetMediaItems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GetMediaItems({})", self.0.unwrap_or(-1))
    }
}

define_actor_message_handler!(
    message_type = GetMediaItems,
    return_type = Vec<MediaItemDto>,
    db_call = |pool, msg: GetMediaItems| query_series(pool, msg.0),
    success_return = |res| res,
    error_return = Vec::<MediaItemDto>::new()
);

#[derive(Debug, Serialize, Deserialize, TS, Message)]
#[rtype(result = "Vec<SeasonDto>")]
pub struct GetSeasons(pub i64);

impl Display for GetSeasons {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GetSeasons({})", self.0)
    }
}

define_actor_message_handler!(
    message_type = GetSeasons,
    return_type = Vec<SeasonDto>,
    db_call = |pool, msg: GetSeasons| query_seasons_with_episodes(pool, msg.0),
    success_return = |res| res,
    error_return = Vec::<SeasonDto>::new()
);

#[derive(Debug, Serialize, Deserialize, TS, Message)]
#[rtype(result = "i64")]
pub struct CreateMediaLibrary(pub CreateMediaLibraryPayload, pub i64);

impl Display for CreateMediaLibrary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CreateMediaLibrary({:?}, {})", self.0, self.1)
    }
}

pub const SENTINEL_MEDIA_LIBRARY_ID: i64 = -1;
define_actor_message_handler!(
    message_type = CreateMediaLibrary,
    return_type = i64,
    db_call = |pool, msg: CreateMediaLibrary| insert_media_library(pool, msg.0, msg.1),
    success_return = |res| res,
    error_return = SENTINEL_MEDIA_LIBRARY_ID
);

#[derive(Debug, Serialize, Deserialize, TS, Message)]
#[rtype(result = "Vec<MediaLibraryDto>")]
pub struct GetMediaLibraries;

impl Display for GetMediaLibraries {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GetMediaLibraries")
    }
}

define_actor_message_handler!(
    message_type = GetMediaLibraries,
    return_type = Vec<MediaLibraryDto>,
    db_call = |pool, _: GetMediaLibraries| query_media_libraries(pool),
    success_return = |res| res,
    error_return = Vec::<MediaLibraryDto>::new()
);

#[derive(Debug, Serialize, Deserialize, TS, Message)]
#[rtype(result = "()")]
pub struct DeleteMediaLibrary(pub i64);

impl Display for DeleteMediaLibrary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DeleteMediaLibrary({})", self.0)
    }
}

define_actor_message_handler!(
    message_type = DeleteMediaLibrary,
    return_type = (),
    db_call = |pool, msg: DeleteMediaLibrary| delete_media_library(pool, msg.0),
    success_return = |_| (),
    error_return = ()
);

#[derive(Debug, Serialize, Deserialize, TS, Message)]
#[rtype(result = "()")]
pub struct CheckCategoryExists(pub i64);

impl Display for CheckCategoryExists {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CheckCategoryExists({})", self.0)
    }
}

define_actor_message_handler!(
    message_type = CheckCategoryExists,
    return_type = (),
    db_call = |pool, msg: CheckCategoryExists| check_category_exists(pool, msg.0),
    success_return = |_| (),
    error_return = ()
);
