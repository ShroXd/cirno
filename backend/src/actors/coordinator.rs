use std::fmt::Debug;

use actix::prelude::*;
use actix::Message;
use dev::ToEnvelope;
use tracing::*;

use super::pipeline_actor::PipelineActor;

#[derive(Clone, Debug)]
pub struct Coordinator<T: Actor + Debug> {
    pub websocket_addr: Option<Addr<T>>,
    pipeline_addr: Option<Addr<PipelineActor>>,
}

impl<T> Actor for Coordinator<T>
where
    T: Actor + Debug,
{
    type Context = Context<Self>;
}

impl<T> Default for Coordinator<T>
where
    T: Actor + Debug,
{
    fn default() -> Self {
        Self {
            websocket_addr: None,
            pipeline_addr: None,
        }
    }
}

impl<T> Coordinator<T>
where
    T: Actor + Debug,
{
    pub fn set_websocket_addr(&mut self, addr: Addr<T>) {
        self.websocket_addr = Some(addr);
    }
}

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub enum WebSocketForwardMessage {
    Send(String),
}

impl<T> Handler<WebSocketForwardMessage> for Coordinator<T>
where
    T: Actor + Debug + Handler<WebSocketForwardMessage>,
    T::Context: ToEnvelope<T, WebSocketForwardMessage>,
{
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
pub struct RegisterWebSocket<T: Actor + Debug>(pub Addr<T>);

impl<T: Actor + Debug> Handler<RegisterWebSocket<T>> for Coordinator<T> {
    type Result = ();

    fn handle(&mut self, msg: RegisterWebSocket<T>, _: &mut Self::Context) -> Self::Result {
        self.set_websocket_addr(msg.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[derive(Clone, Debug)]
    struct MockWebSocketActor {
        pub received_messages: Arc<Mutex<Vec<String>>>,
    }

    impl Actor for MockWebSocketActor {
        type Context = Context<Self>;
    }

    impl Handler<WebSocketForwardMessage> for MockWebSocketActor {
        type Result = ();

        fn handle(&mut self, msg: WebSocketForwardMessage, _: &mut Self::Context) -> Self::Result {
            // TODO: after add move kinds of messages, use if let
            let WebSocketForwardMessage::Send(message) = msg;
            self.received_messages.lock().unwrap().push(message);
        }
    }

    #[actix::test]
    async fn test_register_websocket() {
        // Setup
        let received_messages = Arc::new(Mutex::new(Vec::new()));
        let mock_ws_actor = MockWebSocketActor {
            received_messages: received_messages.clone(),
        }
        .start();
        let coordinator_addr = Coordinator::<MockWebSocketActor>::default().start();

        // Register WebSocket
        coordinator_addr
            .send(RegisterWebSocket(mock_ws_actor.clone()))
            .await
            .unwrap();

        // Send test message
        let test_message = "this is a test".to_string();
        coordinator_addr
            .send(WebSocketForwardMessage::Send(test_message.clone()))
            .await
            .unwrap();

        // Assert
        let msgs = received_messages.lock().unwrap();
        assert_eq!(msgs.len(), 1, "Expected one message to be received");
        assert_eq!(
            msgs[0], test_message,
            "Received message doesn't match sent message"
        );
    }
}
