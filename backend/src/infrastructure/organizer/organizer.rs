use actix::{Actor, Context, Handler, Message};
use anyhow::Error;
use serde::{Deserialize, Serialize};
use std::path::Path;
use ts_rs::TS;

use super::media_library::scanner::scanner::scan_media_library;
use crate::domain::media_library::model::MediaLibrary;

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

#[derive(Debug, TS, Deserialize, Serialize, Message)]
#[rtype(result = "Result<MediaLibrary, Error>")]
pub struct ScanMediaLibrary(pub String);

impl Handler<ScanMediaLibrary> for ParserActor {
    type Result = Result<MediaLibrary, Error>;

    fn handle(&mut self, msg: ScanMediaLibrary, _: &mut Self::Context) -> Self::Result {
        let root_dir = Path::new(&msg.0);
        let media_library = scan_media_library(root_dir);
        // TODO: insert into database
        Ok(media_library)
    }
}
