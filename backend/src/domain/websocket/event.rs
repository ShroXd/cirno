#[derive(Debug, Clone)]
pub enum WebSocketEvent {
    RegisterClient { key: String },
}
