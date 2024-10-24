use actix::Addr;
use actix_web::{get, web, HttpRequest, Responder};
use actix_web_actors::ws;

use crate::actors::{
    coordinator::Coordinator,
    websocket_actor::{WebSocketActor, WebSocketActorBehavior},
};

#[get("/ws")]
pub async fn ws_index(
    r: HttpRequest,
    stream: web::Payload,
    coordinator_addr: web::Data<Addr<Coordinator<WebSocketActor>>>,
) -> impl Responder {
    let ws_actor = WebSocketActor::new(coordinator_addr.get_ref().clone());
    ws::start(ws_actor, &r, stream)
}
