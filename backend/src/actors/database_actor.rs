use actix::{fut, prelude::*, Actor, Context, Handler, Message, WrapFuture};

use serde::{Deserialize, Serialize};
use tracing::*;
use ts_rs::TS;

use crate::{
    database::{create::insert_tv_series, database::Database},
    services::library_parser::parsers::TVSerie,
};

// TODO: extend this to enum until we have more types to be inserted
#[derive(Debug, Serialize, Deserialize, TS, Message)]
#[rtype(result = "()")]
pub struct InsertSeries(pub TVSerie);

impl Actor for Database {
    type Context = Context<Self>;
}

impl Handler<InsertSeries> for Database {
    type Result = ResponseActFuture<Self, ()>;

    fn handle(&mut self, msg: InsertSeries, _: &mut Self::Context) -> Self::Result {
        info!("Inserting series: {:?}", msg.0.title);

        let pool = self.get_connection_pool();

        Box::pin(
            async move { insert_tv_series(&pool, &msg.0).await }
                .into_actor(self)
                .then(|result, _actor, _ctx| {
                    match result {
                        Ok(_) => (),
                        Err(e) => error!("Error inserting series: {:?}", e),
                    }
                    fut::ready(())
                }),
        )
    }
}
