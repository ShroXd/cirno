use actix::{spawn, Addr};
use actix_web::web::Data;
use anyhow::*;
use std::result::Result::Ok;
use tracing::*;

use super::api_models::{CreateMediaLibraryPayload, CreateMediaLibraryResponse};
use crate::{
    actors::{
        database_actor::{
            CreateMediaLibrary, DeleteMediaLibrary, GetMediaLibraries, InsertSeries,
            SENTINEL_MEDIA_LIBRARY_ID,
        },
        parser_actor::{ParserActor, ScanMediaLibrary},
        utils::WsConnections,
        websocket_actor::Notification,
    },
    database::database::Database,
    interfaces::dtos::MediaLibraryDto,
};

pub async fn create_media_library_controller(
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

#[instrument(skip(database_addr))]
pub async fn get_media_libraries_controller(
    database_addr: Data<Addr<Database>>,
) -> Result<Vec<MediaLibraryDto>> {
    match database_addr.send(GetMediaLibraries).await {
        Ok(media_libraries) => {
            debug!("Found {} media libraries", media_libraries.len());
            Ok(media_libraries)
        }
        Err(e) => {
            error!("Failed to get media libraries: {:?}", e);
            return Err(anyhow!("Failed to get media libraries"));
        }
    }
}

#[instrument(skip(database_addr))]
pub async fn delete_media_library_controller(
    id: i64,
    database_addr: Data<Addr<Database>>,
) -> Result<()> {
    match database_addr.send(DeleteMediaLibrary(id)).await {
        Ok(_) => {
            debug!("Deleted media library with id {}", id);
            Ok(())
        }
        Err(e) => {
            error!("Failed to delete media library: {:?}", e);
            return Err(anyhow!("Failed to delete media library"));
        }
    }
}
