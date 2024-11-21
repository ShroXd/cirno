use actix::{spawn, Addr};
use actix_web::web::Data;
use anyhow::*;
use std::result::Result::Ok;
use tracing::*;

use crate::{
    actors::{
        database_actor::SENTINEL_MEDIA_LIBRARY_ID, parser_actor::ParserActor, utils::WsConnections,
        websocket_actor::Notification,
    },
    database::database::Database,
    domain::{
        file_processor::scan_media_library, media_item::insert_media_item,
        media_library::create_media_library,
    },
    interfaces::http_api::controllers::api_models::{
        CreateMediaLibraryPayload, CreateMediaLibraryResponse,
    },
};

#[instrument(skip(database_addr, parser_addr, ws_connections))]
pub async fn create_media_library_service(
    payload: CreateMediaLibraryPayload,
    database_addr: Data<Addr<Database>>,
    parser_addr: Data<Addr<ParserActor>>,
    ws_connections: Data<WsConnections>,
    ws_client_key: String,
) -> Result<CreateMediaLibraryResponse> {
    let directory_clone = payload.directory.clone();
    let database_addr = database_addr.into_inner();

    let media_library_id = create_media_library(payload, database_addr.clone()).await?;
    debug!("Media library created with id: {:?}", media_library_id);

    if media_library_id == SENTINEL_MEDIA_LIBRARY_ID {
        error!("Failed to create media library");
        return Err(anyhow!("Failed to create media library"));
    }

    spawn(async move {
        debug!("Scanning media library");
        let media_items = match scan_media_library(directory_clone, parser_addr).await {
            Ok(media_library) => media_library.series,
            Err(e) => {
                error!("Failed to scan media library: {:?}", e);
                return;
            }
        };

        debug!("Inserting media items");
        for media_item in media_items {
            if let Err(e) =
                insert_media_item(media_library_id, media_item, database_addr.clone()).await
            {
                error!("Failed to insert media item: {:?}", e);
                return;
            }
        }

        // Artificial delay to test frontend async UI behavior, will be removed
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

        debug!(
            "Sending media library scanned notification via websocket to client: {:?}",
            ws_client_key
        );
        // TODO: refactor this to single async manager actor
        let ws_connections = ws_connections.get(ws_client_key).await;
        if let Some(ws_connection) = ws_connections {
            match ws_connection
                .send(Notification::MediaLibraryScanned(media_library_id))
                .await
            {
                Ok(_) => debug!("Media library scanned notification sent"),
                Err(e) => error!("Failed to send notification: {:?}", e),
            }
        }
    });

    Ok(CreateMediaLibraryResponse {
        id: media_library_id,
    })
}
