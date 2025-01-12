use actix::{prelude::*, Actor, Message, StreamHandler};
use actix_web_actors::ws;
use anyhow::*;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};
use std::result::Result::Ok;
use std::sync::Arc;
use tokio::spawn;
use tracing::*;
use ts_rs::TS;
use uuid::Uuid;

use super::{notification::Notification, utils::WsConnections};
use crate::{
    domain::websocket::event::WebSocketEventType,
    infrastructure::event_bus::{domain_event::DomainEvent, event_bus::EventBus},
};

#[derive(Clone)]
pub struct WebSocketActor {
    pub key: Uuid,
    pub ws_connections: Option<WsConnections>,
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

        let event_bus = match self.event_bus.clone() {
            Some(event_bus) => event_bus,
            None => {
                error!("Event bus not set");
                return;
            }
        };

        let key = self.key.to_string();
        let key_for_ws = key.clone();
        let addr_clone = addr.clone();

        debug!("Registering client to WebSocket connections");
        spawn(async move {
            ws_connections.add(key_for_ws, addr).await;
        });

        debug!("Publishing register client event to event bus");
        let event = DomainEvent::WebSocket(WebSocketEventType::RegisterClient(key));
        let event_clone = event.clone();
        if let Err(e) = event_bus.publish(event) {
            error!("Failed to publish register client event: {:?}", e);
        }

        debug!("Sending register client event to client");
        if let Err(e) = event_clone.send_notification::<serde_json::Value>(addr_clone) {
            error!("Failed to send register client event to client: {:?}", e);
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
                    // WebSocketMessage::PipelineAction(action) => {
                    //     self.handle_pipeline_action(action, ctx)
                    // }
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

impl WebSocketActor {
    pub fn new(ws_connections: WsConnections, event_bus: Arc<EventBus>) -> Self {
        Self {
            key: Uuid::new_v4(),
            ws_connections: Some(ws_connections),
            event_bus: Some(event_bus),
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
