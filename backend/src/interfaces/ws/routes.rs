use actix_web::{error::ErrorInternalServerError, get, web, HttpRequest, Responder};
use actix_web_actors::ws;
use tracing::*;

use crate::interfaces::ws::actor::WebSocketActor;

#[get("/ws")]
pub async fn ws_index(
    r: HttpRequest,
    stream: web::Payload,
    app_state: web::Data<crate::init::app_state::AppState>,
) -> impl Responder {
    info!("Starting websocket");

    let ws_connections = app_state.communication().ws_connections();
    let event_bus = app_state.infrastructure().event_bus();

    let ws_actor = WebSocketActor::new(ws_connections.clone(), event_bus.clone());

    match ws::start(ws_actor, &r, stream) {
        Ok(response) => Ok(response),
        Err(e) => {
            error!("Failed to start websocket: {:?}", e);
            Err(ErrorInternalServerError("Failed to start websocket"))
        }
    }
}
