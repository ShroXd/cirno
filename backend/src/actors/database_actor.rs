use actix::{fut, prelude::*, Actor, Context, Handler, Message, WrapFuture};
use serde::{Deserialize, Serialize};
use tracing::*;
use ts_rs::TS;

use crate::{
    database::{
        create::insert_tv_series,
        database::Database,
        query::{query_seasons_with_episodes, query_series, SeasonDTO, TVSeriesDTO},
    },
    services::library_parser::parsers::TVSerie,
};

impl Actor for Database {
    type Context = Context<Self>;
}

// TODO: extend this to enum until we have more types to be inserted
#[derive(Debug, Serialize, Deserialize, TS, Message)]
#[rtype(result = "()")]
pub struct InsertSeries(pub TVSerie);

impl Handler<InsertSeries> for Database {
    type Result = ResponseActFuture<Self, ()>;

    fn handle(&mut self, msg: InsertSeries, _: &mut Self::Context) -> Self::Result {
        info!("Inserting series: {:?}", msg.0.title);

        let pool = self.get_connection_pool();

        Box::pin(
            async move { insert_tv_series(&pool, &msg.0).await }
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
#[rtype(result = "Vec<TVSeriesDTO>")]
pub struct GetSeries;

impl Handler<GetSeries> for Database {
    type Result = ResponseActFuture<Self, Vec<TVSeriesDTO>>;

    fn handle(&mut self, _: GetSeries, _: &mut Self::Context) -> Self::Result {
        info!("Getting series");
        let pool = self.get_connection_pool();

        Box::pin(
            async move { query_series(&pool).await }
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
#[rtype(result = "Vec<SeasonDTO>")]
pub struct GetSeasons(pub i64);

impl Handler<GetSeasons> for Database {
    type Result = ResponseActFuture<Self, Vec<SeasonDTO>>;

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
