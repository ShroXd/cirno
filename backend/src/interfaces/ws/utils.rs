use actix::Addr;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

use super::actor::WebSocketActor;

#[derive(Debug, Default, Clone)]
pub struct WsConnections {
    connections: Arc<RwLock<HashMap<String, Addr<WebSocketActor>>>>,
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
