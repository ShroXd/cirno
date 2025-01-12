use std::sync::Arc;

use actix::Addr;
use actix_web::web::Data;
use anyhow::Result;
use tracing::*;

use super::media_library::model::Library;
use crate::infrastructure::{
    event_bus::event_bus::EventBus,
    organizer::organizer::{ParserActor, ScanLibrary},
};

#[instrument(skip(event_bus))]
pub async fn scan_library(
    directory: String,
    event_bus: Arc<EventBus>,
    parser_addr: Data<Addr<ParserActor>>,
) -> Result<Library> {
    debug!("Scanning library: {}", directory);

    // TODO: move complex logic in the scanner into domain layer
    match parser_addr.send(ScanLibrary(directory, event_bus)).await {
        Ok(result) => result,
        Err(e) => return Err(anyhow::anyhow!("Error scanning library: {:?}", e)),
    }
}
