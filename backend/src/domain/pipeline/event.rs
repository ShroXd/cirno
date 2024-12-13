use super::model::{Duration, PipelineState, Position};

#[derive(Debug, Clone)]
pub enum PipelineEvent {
    StateChanged {
        old_state: PipelineState,
        new_state: PipelineState,
    },
    // TODO: maybe we dont need this
    ErrorOccurred {
        message: String,
        component: String,
    },
    PositionUpdated {
        position: Position,
        duration: Duration,
    },
    EndOfStream,
    PreparationStarted,
    PreparationFinished,
    HlsStreamInitialized {
        path: String,
    },
}
