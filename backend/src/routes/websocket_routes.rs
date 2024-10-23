use actix_web::{get, web, HttpRequest, Responder};
use actix_web_actors::ws;

use crate::actors::websocket_actor::WebSocketActor;

#[get("/ws")]
pub async fn ws_index(r: HttpRequest, stream: web::Payload) -> impl Responder {
    ws::start(WebSocketActor {}, &r, stream)
}
