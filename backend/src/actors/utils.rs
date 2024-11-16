use actix::Addr;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

use super::websocket_actor::WebSocketActor;

#[macro_export]
macro_rules! process_pipeline_action {
    ($self:ident, $pipeline_action:ident) => {{
        debug!(
            "WebSocket actor received {} action",
            stringify!($pipeline_action)
        );

        if let Some(pipeline_addr) = $self.pipeline_addr.as_ref() {
            debug!(
                "WebSocket actor sending {} action to pipeline",
                stringify!($pipeline_action)
            );

            if let Err(e) = pipeline_addr.try_send(PipelineAction::$pipeline_action) {
                error!("Failed to forward message to pipeline: {:?}", e);
            }
        }
    }};
}

#[derive(Debug, Clone)]
pub struct WsConnections {
    connections: Arc<RwLock<HashMap<String, Addr<WebSocketActor>>>>,
}

impl Default for WsConnections {
    fn default() -> Self {
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl WsConnections {
    pub async fn add(&self, id: String, addr: Addr<WebSocketActor>) {
        let mut connections = self.connections.write().await;
        connections.insert(id, addr);
    }

    pub async fn remove(&self, id: String) {
        let mut connections = self.connections.write().await;
        connections.remove(&id);
    }

    pub async fn get(&self, id: String) -> Option<Addr<WebSocketActor>> {
        let connections = self.connections.read().await;
        connections.get(&id).cloned()
    }
}
