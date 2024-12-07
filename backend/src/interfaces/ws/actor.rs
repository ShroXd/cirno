use std::sync::Arc;

use actix::{prelude::*, spawn, Actor, Addr, Message, StreamHandler};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};
use tracing::*;
use ts_rs::TS;
use uuid::Uuid;

use super::{notification::Notification, utils::WsConnections};
use crate::{
    domain::websocket::event::WebSocketEvent,
    infrastructure::{
        database::database::Database,
        event_bus::{domain_event::DomainEvent, event_bus::EventBus},
        organizer::organizer::ParserActor,
        pipeline::{actor::PipelineAction, pipeline::Pipeline},
        task_pool::task_pool::TaskPool,
    },
    process_pipeline_action,
};

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum EventName {
    RegisterClient,
    MediaLibraryScanned,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct RegisterClient {
    // Generated from uuid
    pub key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct EventMessage<T> {
    pub event: EventName,
    pub payload: T,
}

#[derive(Clone)]
pub struct WebSocketActor {
    pub key: Uuid,
    pub pipeline_addr: Option<Addr<Pipeline>>,
    pub parser_addr: Option<Addr<ParserActor>>,
    pub database_addr: Option<Addr<Database>>,
    pub ws_connections: Option<WsConnections>,
    pub task_pool: Option<TaskPool>,
    pub event_bus: Option<Arc<EventBus>>,
}

impl Actor for WebSocketActor {
    type Context = ws::WebsocketContext<Self>;

    #[instrument(skip(self, ctx))]
    fn started(&mut self, ctx: &mut Self::Context) {
        info!("WebSocket actor started");
        let addr = ctx.address();
        info!("Started WebSocket actor address: {:?}", addr);

        let ws_connections = match self.ws_connections.clone() {
            Some(ws_connections) => ws_connections,
            None => {
                error!("WebSocket connections not set");
                return;
            }
        };
        debug!("WebSocket connections: {:?}", ws_connections);

        let event_bus = match &self.event_bus {
            Some(event_bus) => event_bus.clone(),
            None => {
                error!("Event bus not set");
                return;
            }
        };

        let key = self.key.to_string();

        // register the client to the ws connections
        debug!("Registering client to WebSocket connections");
        let key_for_ws = key.clone();
        spawn(async move {
            ws_connections.add(key_for_ws, addr).await;
        });

        // publish the register client event to the event bus
        debug!("Publishing register client event to event bus");
        let key_for_event_bus = key.clone();
        spawn(async move {
            event_bus.publish(
                DomainEvent::WebSocket(WebSocketEvent::RegisterClient {
                    key: key_for_event_bus.clone(),
                }),
                key_for_event_bus,
            );
        });

        // send the register client event to the client
        debug!("Sending register client event to client");
        match to_string(&EventMessage {
            event: EventName::RegisterClient,
            payload: RegisterClient { key },
        }) {
            Ok(message) => ctx.text(message),
            Err(e) => {
                error!("Failed to serialize register client: {:?}", e)
            }
        }
    }

    #[instrument(skip(self, ctx))]
    fn stopped(&mut self, ctx: &mut Self::Context) {
        info!("WebSocket actor stopped");

        let addr = ctx.address();
        info!("Stopped WebSocket actor address: {:?}", addr);

        // TODO: looks like we can create a macro to avoid code duplication
        let key = self.key.to_string();
        let ws_connections = match self.ws_connections.clone() {
            Some(ws_connections) => ws_connections,
            None => {
                error!("WebSocket connections not set");
                return;
            }
        };

        spawn(async move {
            ws_connections.remove(key).await;
        });
    }
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

#[derive(Debug, Serialize, Deserialize, Message)]
#[rtype(result = "()")]
pub struct SendNotification<T>(pub Notification<T>);

impl<T> Handler<SendNotification<T>> for WebSocketActor
where
    T: Serialize,
{
    type Result = ();

    fn handle(&mut self, msg: SendNotification<T>, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            SendNotification(notification) => match to_string(&notification) {
                Ok(message) => ctx.text(message),
                Err(e) => error!("Failed to serialize notification: {:?}", e),
            },
        }
    }
}

pub trait WebSocketActorBehavior {
    fn new(
        pipeline_addr: Addr<Pipeline>,
        parser_addr: Addr<ParserActor>,
        database_addr: Addr<Database>,
        ws_connections: WsConnections,
        task_pool: TaskPool,
        event_bus: Arc<EventBus>,
    ) -> Self;
    fn handle_pipeline_action(
        &self,
        action: PipelineAction,
        _: &mut <WebSocketActor as Actor>::Context,
    );
    fn handle_system(&self, system: System, _: &mut <WebSocketActor as Actor>::Context);
}

impl WebSocketActorBehavior for WebSocketActor {
    fn new(
        pipeline_addr: Addr<Pipeline>,
        parser_addr: Addr<ParserActor>,
        database_addr: Addr<Database>,
        ws_connections: WsConnections,
        task_pool: TaskPool,
        event_bus: Arc<EventBus>,
    ) -> Self {
        Self {
            key: Uuid::new_v4(),
            pipeline_addr: Some(pipeline_addr),
            parser_addr: Some(parser_addr),
            database_addr: Some(database_addr),
            ws_connections: Some(ws_connections),
            task_pool: Some(task_pool),
            event_bus: Some(event_bus),
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
            PipelineAction::SetSource(path) => {
                if let Some(pipeline_addr) = self.pipeline_addr.as_ref() {
                    if let Err(e) = pipeline_addr.try_send(PipelineAction::SetSource(path)) {
                        error!("Failed to forward message to pipeline: {:?}", e);
                    }
                }
            }
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
}
