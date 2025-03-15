use actix::{Actor, Context, Handler, Message};
use anyhow::Error;
use std::{path::Path, sync::Arc};

use super::library::library_scanner::scanner::scan_library;
use crate::{
    domain::media_library::model::Library, infrastructure::event_dispatcher::event_bus::EventBus,
};

#[derive(Debug, Default)]
pub struct ParserActor;

impl Actor for ParserActor {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "Result<Library, Error>")]
pub struct ScanLibrary {
    pub library_path: String,
    pub event_bus: Arc<EventBus>,
}

impl Handler<ScanLibrary> for ParserActor {
    type Result = Result<Library, Error>;

    fn handle(&mut self, msg: ScanLibrary, _: &mut Self::Context) -> Self::Result {
        let root_dir = Path::new(&msg.library_path);
        let library = scan_library(root_dir, msg.event_bus);
        // TODO: insert into database
        Ok(library)
    }
}
