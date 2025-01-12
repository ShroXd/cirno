use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum GeneralEvent {
    // ==================== Task Events ====================
    TaskStarted,
    TaskProgressUpdated {
        progress: f32,
    },

    /// Test event used only in unit tests
    /// Contains an integer ID for test identification
    #[allow(dead_code)]
    TestEvent(i32),
}
