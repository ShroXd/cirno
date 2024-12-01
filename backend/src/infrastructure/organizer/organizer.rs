use actix::{Actor, Context, Handler, Message};
use anyhow::Error;
use serde::{Deserialize, Serialize};
use std::{path::Path, sync::Arc};
use ts_rs::TS;

use super::media_library::scanner::scanner::scan_media_library;
use crate::{
    domain::media_library::model::MediaLibrary, infrastructure::event_bus::event_bus::EventBus,
};

#[derive(Debug)]
pub struct ParserActor;

impl Default for ParserActor {
    fn default() -> Self {
        Self {}
    }
}

impl Actor for ParserActor {
    type Context = Context<Self>;
}

#[derive(Debug, Message)]
#[rtype(result = "Result<MediaLibrary, Error>")]
pub struct ScanMediaLibrary(pub String, pub String, pub Arc<EventBus>);

impl Handler<ScanMediaLibrary> for ParserActor {
    type Result = Result<MediaLibrary, Error>;

    fn handle(&mut self, msg: ScanMediaLibrary, _: &mut Self::Context) -> Self::Result {
        let root_dir = Path::new(&msg.0);
        let media_library = scan_media_library(root_dir, msg.1, msg.2);
        // TODO: insert into database
        Ok(media_library)
    }
}
