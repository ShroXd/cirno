use serde::Serialize;

use crate::interfaces::ws::notification::ToJsonPayload;

use super::model::PipelineState;

#[derive(Debug, Clone, Serialize)]
pub enum PipelineEvent {
    #[allow(dead_code)]
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
    PipelineStopped,
}

impl ToJsonPayload for PipelineEvent {
    fn to_json_payload(&self) -> serde_json::Value {
        match self {
            PipelineEvent::HlsStreamInitialized { path } => {
                serde_json::json!({ "type": "HlsStreamInitialized", "path": path })
            }
            _ => unimplemented!(),
        }
    }
}
