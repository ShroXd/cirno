use actix::AsyncContext;
use actix::{Actor, Addr, Handler, Message, StreamHandler};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use tracing::*;
use ts_rs::TS;

use super::coordinator::{Coordinator, WebSocketForwardMessage};
use crate::actors::coordinator::RegisterWebSocket;

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum PipelineAction {
    Play,
    Pause,
    Stop,
    SetSource(String),
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum System {
    Log(String),
}

#[derive(Debug, Serialize, Deserialize, TS, Message)]
#[rtype(result = "()")]
#[ts(export)]
pub enum WebSocketMessage {
    PipelineAction(PipelineAction),
    System(System),
}

#[derive(Debug)]
pub struct WebSocketActor {
    pub coordinator_addr: Addr<Coordinator>,
}

impl Actor for WebSocketActor {
    type Context = ws::WebsocketContext<Self>;

    #[instrument(skip(self, ctx))]
    fn started(&mut self, ctx: &mut Self::Context) {
        info!("WebSocket actor started");

        let self_addr = ctx.address().clone();

        if let Err(e) = self.coordinator_addr.try_send(RegisterWebSocket(self_addr)) {
            error!("Failed to register WebSocket actor: {:?}", e);
        }
    }

    #[instrument(skip(self))]
    fn stopped(&mut self, _: &mut Self::Context) {
        info!("WebSocket actor stopped");
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketActor {
    #[instrument(skip(self, ctx))]
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                ctx.pong(&msg);
            }
            Ok(ws::Message::Text(msg)) => match from_str::<WebSocketMessage>(&msg) {
                Ok(message) => match message {
                    WebSocketMessage::PipelineAction(action) => {
                        self.handle_pipeline_action(action, ctx)
                    }
                    WebSocketMessage::System(system) => self.handle_system(system, ctx),
                },
                Err(e) => {
                    error!("WebSocket actor received invalid message: {:?}", e);
                }
            },
            _ => {}
        }
    }
}

impl Handler<WebSocketForwardMessage> for WebSocketActor {
    type Result = ();

    fn handle(&mut self, msg: WebSocketForwardMessage, _: &mut Self::Context) {
        info!("WebSocket actor received message: {:?}", msg);
    }
}

impl WebSocketActor {
    pub fn new(coordinator_addr: Addr<Coordinator>) -> Self {
        Self { coordinator_addr }
    }

    fn handle_pipeline_action(
        &self,
        action: PipelineAction,
        _: &mut <WebSocketActor as Actor>::Context,
    ) {
        match action {
            PipelineAction::Play => {
                info!("WebSocket actor received play action");

                // TODO: here is just a quick test, will be removed
                // if let Err(e) = self
                //     .coordinator_addr
                //     .try_send(WebSocketForwardMessage::Send("from play".to_string()))
                // {
                //     error!("Failed to forward message to coordinator: {:?}", e);
                // }
            }
            PipelineAction::Pause => {
                info!("WebSocket actor received pause action");
            }
            PipelineAction::Stop => {
                info!("WebSocket actor received stop action");
            }
            PipelineAction::SetSource(_) => {}
        }
    }

    fn handle_system(&self, system: System, _: &mut <WebSocketActor as Actor>::Context) {
        match system {
            System::Log(message) => info!("WebSocket actor received log message: {}", message),
        }
    }
}
