use actix::Addr;
use actix_web::web::Data;
use anyhow::Result;
use tracing::*;

use crate::{
    actors::parser_actor::{ParserActor, ScanMediaLibrary},
    services::library_parser::scanner::MediaLibrary,
};

#[instrument]
pub async fn scan_media_library(
    directory: String,
    parser_addr: Data<Addr<ParserActor>>,
) -> Result<MediaLibrary> {
    debug!("Scanning media library: {}", directory);

    // TODO: move complex logic in the scanner into domain layer
    match parser_addr.send(ScanMediaLibrary(directory)).await {
        Ok(result) => result,
        Err(e) => return Err(anyhow::anyhow!("Error scanning media library: {:?}", e)),
    }
}
