use actix::{Actor, Addr, Message, StreamHandler};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use tracing::*;
use ts_rs::TS;

use crate::actors::parser_actor::ScanMediaLibrary;
use crate::services::stream::pipeline::Pipeline;

use super::parser_actor::ParserActor;
use super::pipeline_actor::PipelineAction;

#[derive(Debug)]
pub struct WebSocketActor {
    pub pipeline_addr: Option<Addr<Pipeline>>,
    pub parser_addr: Option<Addr<ParserActor>>,
}

impl Actor for WebSocketActor {
    type Context = ws::WebsocketContext<Self>;

    #[instrument(skip(self))]
    fn started(&mut self, _: &mut Self::Context) {
        info!("WebSocket actor started");
        // TODO: consider if it's need to let pipeline & parser holds the address of websocket
    }

    #[instrument(skip(self))]
    fn stopped(&mut self, _: &mut Self::Context) {
        info!("WebSocket actor stopped");
    }
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum System {
    Log(String),
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum Parser {
    Scan(String),
}

#[derive(Debug, Serialize, Deserialize, TS, Message)]
#[rtype(result = "()")]
#[ts(export)]
pub enum WebSocketMessage {
    PipelineAction(PipelineAction),
    System(System),
    Parser(Parser),
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
                    WebSocketMessage::Parser(parser) => self.handle_parser(parser, ctx),
                },
                Err(e) => {
                    error!("WebSocket actor received invalid message: {:?}", e);
                }
            },
            _ => {}
        }
    }
}

pub trait WebSocketActorBehavior {
    fn new(pipeline_addr: Addr<Pipeline>, parser_addr: Addr<ParserActor>) -> Self;
    fn handle_pipeline_action(
        &self,
        action: PipelineAction,
        _: &mut <WebSocketActor as Actor>::Context,
    );
    fn handle_system(&self, system: System, _: &mut <WebSocketActor as Actor>::Context);
    fn handle_parser(&self, parser: Parser, _: &mut <WebSocketActor as Actor>::Context);
}

impl WebSocketActorBehavior for WebSocketActor {
    fn new(pipeline_addr: Addr<Pipeline>, parser_addr: Addr<ParserActor>) -> Self {
        Self {
            pipeline_addr: Some(pipeline_addr),
            parser_addr: Some(parser_addr),
        }
    }

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
            System::Log(message) => info!(
                "WebSocket actor received message for logger from stream: {}",
                message
            ),
        }
    }

    fn handle_parser(&self, parser: Parser, _: &mut <WebSocketActor as Actor>::Context) {
        match parser {
            Parser::Scan(path) => {
                info!(
                    "WebSocket actor received message for parser from stream: {:?}",
                    path
                );

                match self.parser_addr.as_ref() {
                    Some(parser_addr) => {
                        if let Err(e) = parser_addr.try_send(ScanMediaLibrary(path)) {
                            error!("Failed to forward message to parser: {:?}", e);
                        }
                    }
                    None => {
                        error!("Parser address is not set");
                    }
                }
            }
        }
    }
}
