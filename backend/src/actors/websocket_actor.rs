use actix::{Actor, StreamHandler};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use tracing::*;
use ts_rs::TS;

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

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum Message {
    PipelineAction(PipelineAction),
    System(System),
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct WebSocketActor;

impl Actor for WebSocketActor {
    type Context = ws::WebsocketContext<Self>;

    #[instrument(skip(self))]
    fn started(&mut self, _: &mut Self::Context) {
        info!("WebSocket actor started");
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
            Ok(ws::Message::Text(msg)) => match from_str::<Message>(&msg) {
                Ok(message) => match message {
                    Message::PipelineAction(action) => self.handle_pipeline_action(action, ctx),
                    Message::System(system) => self.handle_system(system, ctx),
                },
                Err(e) => {
                    error!("WebSocket actor received invalid message: {:?}", e);
                }
            },
            _ => {}
        }
    }
}

impl WebSocketActor {
    fn handle_pipeline_action(
        &self,
        action: PipelineAction,
        _: &mut <WebSocketActor as Actor>::Context,
    ) {
        match action {
            PipelineAction::Play => {
                info!("WebSocket actor received play action");
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
