use serde::Serialize;

use super::model::{Duration, PipelineState, Position};

#[derive(Debug, Clone, Serialize)]
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
    // TODO: implement custom serde for Position and Duration
    // PositionUpdated {
    //     position: Position,
    //     duration: Duration,
    // },
    EndOfStream,
    PreparationStarted,
    PreparationFinished,
    HlsStreamInitialized {
        path: String,
    },
}
