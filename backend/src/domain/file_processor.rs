use std::sync::Arc;

use actix::Addr;
use actix_web::web::Data;
use anyhow::Result;
use tracing::*;

use super::media_library::model::MediaLibrary;
use crate::infrastructure::{
    event_bus::event_bus::EventBus,
    organizer::organizer::{ParserActor, ScanMediaLibrary},
};

#[instrument]
pub async fn scan_media_library(
    directory: String,
    task_id: String,
    event_bus: Arc<EventBus>,
    parser_addr: Data<Addr<ParserActor>>,
) -> Result<MediaLibrary> {
    debug!("Scanning media library: {}", directory);

    // TODO: move complex logic in the scanner into domain layer
    match parser_addr
        .send(ScanMediaLibrary(directory, task_id, event_bus))
        .await
    {
        Ok(result) => result,
        Err(e) => return Err(anyhow::anyhow!("Error scanning media library: {:?}", e)),
    }
}
