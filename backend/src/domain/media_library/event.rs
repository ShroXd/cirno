#[derive(Debug, Clone)]
pub enum MediaLibraryEventType {
    MediaLibraryScanned { ws_client_id: String },
}
