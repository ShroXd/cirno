use actix_web::{error::ErrorInternalServerError, get, web, HttpRequest, Responder};
use actix_web_actors::ws;
use std::sync::Arc;
use tracing::*;

use crate::{
    infrastructure::event_bus::event_bus::EventBus,
    interfaces::ws::{actor::WebSocketActor, utils::WsConnections},
};

#[get("/ws")]
pub async fn ws_index(
    r: HttpRequest,
    stream: web::Payload,
    ws_connections: web::Data<WsConnections>,
    event_bus: web::Data<Arc<EventBus>>,
) -> impl Responder {
    info!("Starting websocket");
    let ws_actor = WebSocketActor::new(
        ws_connections.get_ref().clone(),
        event_bus.get_ref().clone(),
    );

    match ws::start(ws_actor, &r, stream) {
        Ok(response) => Ok(response),
        Err(e) => {
            error!("Failed to start websocket: {:?}", e);
            Err(ErrorInternalServerError("Failed to start websocket"))
        }
    }
}
