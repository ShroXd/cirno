use actix::{prelude::*, Actor, Addr, Message, StreamHandler, WrapFuture};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use tracing::*;
use ts_rs::TS;

use super::parser_actor::ParserActor;
use super::pipeline_actor::PipelineAction;
use crate::actors::database_actor::InsertSeries;
use crate::actors::parser_actor::ScanMediaLibrary;
use crate::database::database::Database;
use crate::process_pipeline_action;
use crate::services::gstreamer_pipeline::pipeline::Pipeline;

#[derive(Debug, Clone)]
pub struct WebSocketActor {
    pub pipeline_addr: Option<Addr<Pipeline>>,
    pub parser_addr: Option<Addr<ParserActor>>,
    pub database_addr: Option<Addr<Database>>,
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
                    WebSocketMessage::Parser(parser) => {
                        let parser_addr = match self.parser_addr.as_ref() {
                            Some(parser_addr) => parser_addr.clone(),
                            None => {
                                error!("Parser address is not set");
                                return;
                            }
                        };

                        let database_addr = match self.database_addr.as_ref() {
                            Some(database_addr) => database_addr.clone(),
                            None => {
                                error!("Database address is not set");
                                return;
                            }
                        };

                        let future = async move {
                            WebSocketActor::handle_parser(parser, parser_addr, database_addr).await
                        };

                        ctx.spawn(fut::wrap_future::<_, Self>(future));
                    }
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
    fn new(
        pipeline_addr: Addr<Pipeline>,
        parser_addr: Addr<ParserActor>,
        database_addr: Addr<Database>,
    ) -> Self;
    fn handle_pipeline_action(
        &self,
        action: PipelineAction,
        _: &mut <WebSocketActor as Actor>::Context,
    );
    fn handle_system(&self, system: System, _: &mut <WebSocketActor as Actor>::Context);
    async fn handle_parser(
        parser: Parser,
        parser_addr: Addr<ParserActor>,
        database_addr: Addr<Database>,
    );
}

impl WebSocketActorBehavior for WebSocketActor {
    fn new(
        pipeline_addr: Addr<Pipeline>,
        parser_addr: Addr<ParserActor>,
        database_addr: Addr<Database>,
    ) -> Self {
        Self {
            pipeline_addr: Some(pipeline_addr),
            parser_addr: Some(parser_addr),
            database_addr: Some(database_addr),
        }
    }

    fn handle_pipeline_action(
        &self,
        action: PipelineAction,
        _: &mut <WebSocketActor as Actor>::Context,
    ) {
        match action {
            PipelineAction::Play => process_pipeline_action!(self, Play),
            PipelineAction::Pause => process_pipeline_action!(self, Pause),
            PipelineAction::Stop => process_pipeline_action!(self, Stop),
            // TODO: refactor and use the macro to avoid code duplication
            PipelineAction::Seek(position) => {
                debug!("WebSocket actor received seek action: {:?}", position);

                if let Some(pipeline_addr) = self.pipeline_addr.as_ref() {
                    if let Err(e) = pipeline_addr.try_send(PipelineAction::Seek(position)) {
                        error!("Failed to forward message to pipeline: {:?}", e);
                    }
                }
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

    async fn handle_parser(
        parser: Parser,
        parser_addr: Addr<ParserActor>,
        database_addr: Addr<Database>,
    ) {
        match parser {
            Parser::Scan(path) => {
                info!(
                    "WebSocket actor received message for parser from stream: {:?}",
                    path
                );

                match parser_addr.send(ScanMediaLibrary(path)).await {
                    Ok(Ok(library)) => {
                        info!("Media library scanned: {:?}", library.series.len());

                        for series in library.series {
                            // BUG: did not insert episode and actors
                            if let Err(e) = database_addr.try_send(InsertSeries(series)) {
                                error!("Failed to forward message to database: {:?}", e);
                            }
                        }
                    }
                    Ok(Err(e)) => error!("Failed to scan media library: {:?}", e),
                    Err(e) => error!("Failed to send message to parser: {:?}", e),
                }
            }
        }
    }
}
