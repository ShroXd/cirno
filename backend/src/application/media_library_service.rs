use actix::{spawn, Addr};
use actix_web::web::Data;
use anyhow::*;
use std::result::Result::Ok;
use tracing::*;

use crate::{
    actors::{
        database_actor::{CreateMediaLibrary, InsertSeries, SENTINEL_MEDIA_LIBRARY_ID},
        parser_actor::{ParserActor, ScanMediaLibrary},
        utils::WsConnections,
        websocket_actor::Notification,
    },
    database::database::Database,
    interfaces::http_api::controllers::api_models::{
        CreateMediaLibraryPayload, CreateMediaLibraryResponse,
    },
};

pub async fn create_media_library(
    payload: CreateMediaLibraryPayload,
    database_addr: Data<Addr<Database>>,
    parser_addr: Data<Addr<ParserActor>>,
    ws_connections: Data<WsConnections>,
    ws_client_key: String,
) -> Result<CreateMediaLibraryResponse> {
    match database_addr
        .send(CreateMediaLibrary(payload.clone()))
        .await
    {
        Ok(media_library_id) => {
            debug!("Media library created with id: {:?}", media_library_id);
            if media_library_id == SENTINEL_MEDIA_LIBRARY_ID {
                error!("Failed to create media library");
                return Err(anyhow!("Failed to create media library"));
            } else {
                spawn(async move {
                    // Scan library, insert data into DB, and notify frontend via websocket
                    let directory = payload.directory.clone();
                    match parser_addr.send(ScanMediaLibrary(directory)).await {
                        Ok(result) => {
                            let media_library = result.expect("Failed to scan media library");
                            for serie in media_library.series {
                                match database_addr
                                    .send(InsertSeries(serie, media_library_id))
                                    .await
                                {
                                    Ok(_) => debug!("Series inserted"),
                                    Err(e) => error!("Failed to insert series: {:?}", e),
                                }
                            }

                            // Artificial delay to test frontend async UI behavior, will be removed
                            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

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
                        }
                        Err(e) => error!("Failed to scan media library: {:?}", e),
                    }
                });

                Ok(CreateMediaLibraryResponse {
                    id: media_library_id,
                })
            }
        }
        Err(e) => {
            error!("Failed to create media library: {:?}", e);
            return Err(anyhow!("Failed to create media library"));
        }
    }
}
