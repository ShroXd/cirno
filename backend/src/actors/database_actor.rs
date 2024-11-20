use actix::{fut, prelude::*, Actor, Context, Handler, Message, WrapFuture};
use serde::{Deserialize, Serialize};
use tracing::*;
use ts_rs::TS;

use crate::{
    database::{
        create::{insert_media_library, insert_tv_series},
        database::Database,
        delete::delete_media_library,
        query::{query_media_libraries, query_seasons_with_episodes, query_series},
    },
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

impl Handler<InsertSeries> for Database {
    type Result = ResponseActFuture<Self, ()>;

    fn handle(&mut self, msg: InsertSeries, _: &mut Self::Context) -> Self::Result {
        info!("Inserting series: {:?}", msg.0.title);

        let pool = self.get_connection_pool();

        Box::pin(
            async move { insert_tv_series(&pool, &msg.0, msg.1).await }
                .into_actor(self)
                .then(|result, _actor, _ctx| match result {
                    Ok(_) => fut::ready(()),
                    Err(e) => {
                        error!("Error inserting series: {:?}", e);
                        fut::ready(())
                    }
                }),
        )
    }
}

#[derive(Debug, Serialize, Deserialize, TS, Message)]
#[rtype(result = "Vec<MediaItemDto>")]
pub struct GetMediaItems(pub Option<i64>);

impl Handler<GetMediaItems> for Database {
    type Result = ResponseActFuture<Self, Vec<MediaItemDto>>;

    fn handle(&mut self, msg: GetMediaItems, _: &mut Self::Context) -> Self::Result {
        info!("Getting series");
        let pool = self.get_connection_pool();

        Box::pin(
            async move { query_series(&pool, msg.0).await }
                .into_actor(self)
                .then(|result, _actor, _ctx| match result {
                    Ok(series) => fut::ready(series),
                    Err(e) => {
                        error!("Error getting series: {:?}", e);
                        fut::ready(Vec::new())
                    }
                }),
        )
    }
}

#[derive(Debug, Serialize, Deserialize, TS, Message)]
#[rtype(result = "Vec<SeasonDto>")]
pub struct GetSeasons(pub i64);

impl Handler<GetSeasons> for Database {
    type Result = ResponseActFuture<Self, Vec<SeasonDto>>;

    fn handle(&mut self, msg: GetSeasons, _: &mut Self::Context) -> Self::Result {
        info!("Getting seasons for series: {:?}", msg.0);
        let pool = self.get_connection_pool();

        Box::pin(
            async move { query_seasons_with_episodes(&pool, msg.0).await }
                .into_actor(self)
                .then(|result, _actor, _ctx| match result {
                    Ok(seasons) => fut::ready(seasons),
                    Err(e) => {
                        error!("Error getting seasons: {:?}", e);
                        fut::ready(Vec::new())
                    }
                }),
        )
    }
}

pub const SENTINEL_MEDIA_LIBRARY_ID: i64 = -1;

#[derive(Debug, Serialize, Deserialize, TS, Message)]
#[rtype(result = "i64")]
pub struct CreateMediaLibrary(pub CreateMediaLibraryPayload);

impl Handler<CreateMediaLibrary> for Database {
    type Result = ResponseActFuture<Self, i64>;

    fn handle(&mut self, msg: CreateMediaLibrary, _: &mut Self::Context) -> Self::Result {
        info!("Creating media library: {:?}", msg.0);
        let pool = self.get_connection_pool();

        Box::pin(
            async move { insert_media_library(&pool, &msg.0).await }
                .into_actor(self)
                .then(|result, _actor, _ctx| match result {
                    Ok(media_library_id) => fut::ready(media_library_id),
                    Err(e) => {
                        error!("Error creating media library: {:?}", e);
                        fut::ready(SENTINEL_MEDIA_LIBRARY_ID)
                    }
                }),
        )
    }
}

#[derive(Debug, Serialize, Deserialize, TS, Message)]
#[rtype(result = "Vec<MediaLibraryDto>")]
pub struct GetMediaLibraries;

impl Handler<GetMediaLibraries> for Database {
    type Result = ResponseActFuture<Self, Vec<MediaLibraryDto>>;

    fn handle(&mut self, _: GetMediaLibraries, _: &mut Self::Context) -> Self::Result {
        info!("Getting media libraries");
        let pool = self.get_connection_pool();

        Box::pin(
            async move { query_media_libraries(&pool).await }
                .into_actor(self)
                .then(|result, _actor, _ctx| match result {
                    Ok(media_libraries) => fut::ready(media_libraries),
                    Err(e) => {
                        error!("Error getting media libraries: {:?}", e);
                        fut::ready(Vec::new())
                    }
                }),
        )
    }
}

#[derive(Debug, Serialize, Deserialize, TS, Message)]
#[rtype(result = "()")]
pub struct DeleteMediaLibrary(pub i64);

impl Handler<DeleteMediaLibrary> for Database {
    type Result = ResponseActFuture<Self, ()>;

    fn handle(&mut self, msg: DeleteMediaLibrary, _: &mut Self::Context) -> Self::Result {
        info!("Deleting media library: {:?}", msg.0);
        let pool = self.get_connection_pool();

        Box::pin(
            async move { delete_media_library(&pool, msg.0).await }
                .into_actor(self)
                .then(|result, _actor, _ctx| match result {
                    Ok(_) => fut::ready(()),
                    Err(e) => {
                        error!("Error deleting media library: {:?}", e);
                        fut::ready(())
                    }
                }),
        )
    }
}
