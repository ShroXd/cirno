#[derive(Debug, Clone)]
pub enum GeneralEvent {
    // ==================== Task Events ====================
    TaskStarted,
    TaskProgressUpdated {
        progress: f32,
    },
    TaskCompleted,
    TaskError {
        error: String,
    },

    /// Test event used only in unit tests
    /// Contains an integer ID for test identification
    TestEvent(i32),
}
