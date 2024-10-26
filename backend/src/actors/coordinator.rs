use actix::prelude::*;
use actix::Message;
use std::fmt::Debug;
use tracing::*;

use super::parser_actor::ParserActor;
use super::parser_actor::ScanMediaLibrary;
use super::websocket_actor::WebSocketActor;
use crate::services::stream::pipeline::Pipeline;

#[derive(Clone, Debug)]
pub struct Coordinator {
    pub websocket_addr: Option<Addr<WebSocketActor>>,
    pub pipeline_addr: Option<Addr<Pipeline>>,
    pub parser_addr: Option<Addr<ParserActor>>,
}

impl Actor for Coordinator {
    type Context = Context<Self>;
}

impl Default for Coordinator {
    fn default() -> Self {
        Self {
            websocket_addr: None,
            pipeline_addr: None,
            parser_addr: None,
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
pub enum ParserForwardMessage {
    Scan(String),
}

impl Handler<ParserForwardMessage> for Coordinator {
    type Result = ();

    fn handle(&mut self, msg: ParserForwardMessage, _: &mut Self::Context) -> Self::Result {
        if let Some(addr) = &self.parser_addr {
            match msg {
                ParserForwardMessage::Scan(path) => {
                    if let Err(e) = addr.try_send(ScanMediaLibrary(path)) {
                        error!("Failed to forward ScanMediaLibrary message: {:?}", e);
                    }
                }
            }
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::sync::{Arc, Mutex};

//     #[derive(Clone, Debug)]
//     struct MockWebSocketActor {
//         pub received_messages: Arc<Mutex<Vec<String>>>,
//     }

//     impl Actor for MockWebSocketActor {
//         type Context = Context<Self>;
//     }

//     impl Handler<WebSocketForwardMessage> for MockWebSocketActor {
//         type Result = ();

//         fn handle(&mut self, msg: WebSocketForwardMessage, _: &mut Self::Context) -> Self::Result {
//             // TODO: after add move kinds of messages, use if let
//             let WebSocketForwardMessage::Send(message) = msg;
//             self.received_messages.lock().unwrap().push(message);
//         }
//     }

//     #[actix::test]
//     async fn test_register_websocket() {
//         // Setup
//         let received_messages = Arc::new(Mutex::new(Vec::new()));
//         let mock_ws_actor = MockWebSocketActor {
//             received_messages: received_messages.clone(),
//         }
//         .start();
//         let coordinator_addr = Coordinator::<MockWebSocketActor>::default().start();

//         // Register WebSocket
//         coordinator_addr
//             .send(RegisterWebSocket(mock_ws_actor.clone()))
//             .await
//             .unwrap();

//         // Send test message
//         let test_message = "this is a test".to_string();
//         coordinator_addr
//             .send(WebSocketForwardMessage::Send(test_message.clone()))
//             .await
//             .unwrap();

//         // Assert
//         let msgs = received_messages.lock().unwrap();
//         assert_eq!(msgs.len(), 1, "Expected one message to be received");
//         assert_eq!(
//             msgs[0], test_message,
//             "Received message doesn't match sent message"
//         );
//     }
// }
