#[derive(Debug, Clone)]
pub enum GeneralEvent {
    // ==================== Task Events ====================
    TaskStarted,
    TaskProgressUpdated { progress: f32 },
    TaskCompleted,
    TaskError { error: String },
}
