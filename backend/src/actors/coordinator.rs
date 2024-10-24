use actix::prelude::*;
use actix::Message;
use tracing::*;

use super::pipeline_actor::PipelineActor;
use super::websocket_actor::WebSocketActor;

pub struct Coordinator {
    websocket_addr: Option<Addr<WebSocketActor>>,
    pipeline_addr: Option<Addr<PipelineActor>>,
}

impl Actor for Coordinator {
    type Context = Context<Self>;
}

impl Default for Coordinator {
    fn default() -> Self {
        Self {
            websocket_addr: None,
            pipeline_addr: None,
        }
    }
}

impl Coordinator {
    pub fn set_websocket_addr(&mut self, addr: Addr<WebSocketActor>) {
        self.websocket_addr = Some(addr);
    }
}

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub enum WebSocketForwardMessage {
    Send(String),
}

impl Handler<WebSocketForwardMessage> for Coordinator {
    type Result = ();

    fn handle(&mut self, msg: WebSocketForwardMessage, _: &mut Self::Context) -> Self::Result {
        if let Some(addr) = &self.websocket_addr {
            match addr.try_send(msg) {
                Ok(_) => {
                    info!("Forwarded WebSocket message to actor");
                }
                Err(e) => {
                    error!("Failed to forward WebSocket message: {:?}", e);
                }
            }
        } else {
            warn!("WebSocket actor not found");
        }
    }
}

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct RegisterWebSocket(pub Addr<WebSocketActor>);

impl Handler<RegisterWebSocket> for Coordinator {
    type Result = ();

    fn handle(&mut self, msg: RegisterWebSocket, _: &mut Self::Context) -> Self::Result {
        self.set_websocket_addr(msg.0);
    }
}
