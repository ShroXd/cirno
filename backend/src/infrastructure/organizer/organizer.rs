use actix::{Actor, Context, Handler, Message};
use anyhow::Error;
use std::{path::Path, sync::Arc};

use super::library::scanner::scanner::scan_library;
use crate::{domain::library::model::Library, infrastructure::event_bus::event_bus::EventBus};

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

#[derive(Message)]
#[rtype(result = "Result<Library, Error>")]
pub struct ScanLibrary(pub String, pub Arc<EventBus>);

impl Handler<ScanLibrary> for ParserActor {
    type Result = Result<Library, Error>;

    fn handle(&mut self, msg: ScanLibrary, _: &mut Self::Context) -> Self::Result {
        let root_dir = Path::new(&msg.0);
        let library = scan_library(root_dir, msg.1);
        // TODO: insert into database
        Ok(library)
    }
}
