use super::model::MediaLibrary;

#[derive(Debug, Clone)]
pub enum MediaLibraryEventType {
    MediaLibraryScanned(MediaLibrary),
    MediaLibrarySaved,
}
