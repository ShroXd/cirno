use actix::prelude::*;
use actix::{Actor, Addr, Context, Handler, Message};
use anyhow::Error;
use serde::{Deserialize, Serialize};
use std::path::Path;
use ts_rs::TS;

use super::coordinator::Coordinator;
use crate::services::library_parser::scanner::{scan_media_library, MediaLibrary};

#[derive(Debug)]
pub struct ParserActor {
    pub coordinator_addr: Addr<Coordinator<Self>>,
}

impl Actor for ParserActor {
    type Context = Context<Self>;
}

#[derive(Debug, TS, Deserialize, Serialize, Message)]
#[rtype(result = "Result<MediaLibrary, Error>")]
pub struct ScanMediaLibrary(String);

impl Handler<ScanMediaLibrary> for ParserActor {
    type Result = Result<MediaLibrary, Error>;

    fn handle(&mut self, msg: ScanMediaLibrary, _: &mut Self::Context) -> Self::Result {
        let root_dir = Path::new(&msg.0);
        let media_library = scan_media_library(root_dir);
        Ok(media_library)
    }
}
