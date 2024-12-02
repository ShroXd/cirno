use std::sync::Arc;

use actix::Addr;
use actix_web::{error::ErrorInternalServerError, get, web, HttpRequest, Responder};
use actix_web_actors::ws;
use tracing::*;

use crate::{
    infrastructure::{
        database::database::Database, event_bus::event_bus::EventBus,
        organizer::organizer::ParserActor, pipeline::pipeline::Pipeline,
        task_pool::task_pool::TaskPool,
    },
    interfaces::ws::{
        actor::{WebSocketActor, WebSocketActorBehavior},
        utils::WsConnections,
    },
};

#[get("/ws")]
pub async fn ws_index(
    r: HttpRequest,
    stream: web::Payload,
    pipeline_addr: web::Data<Addr<Pipeline>>,
    parser_addr: web::Data<Addr<ParserActor>>,
    database_addr: web::Data<Addr<Database>>,
    ws_connections: web::Data<WsConnections>,
    task_pool: web::Data<TaskPool>,
    event_bus: web::Data<Arc<EventBus>>,
) -> impl Responder {
    info!("Starting websocket");
    let ws_actor = WebSocketActor::new(
        pipeline_addr.get_ref().clone(),
        parser_addr.get_ref().clone(),
        database_addr.get_ref().clone(),
        ws_connections.get_ref().clone(),
        task_pool.get_ref().clone(),
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
