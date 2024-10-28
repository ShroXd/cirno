use actix::{Actor, Context, Handler, Message};
use gstreamer::prelude::*;
use gstreamer::State;
use serde::{Deserialize, Serialize};
use tracing::*;
use ts_rs::TS;

use crate::services::stream::pipeline::Pipeline;

impl Actor for Pipeline {
    type Context = Context<Pipeline>;
}

#[derive(Debug, Serialize, Deserialize, TS, Message)]
#[rtype(result = "()")]
#[ts(export)]
pub enum PipelineAction {
    Play,
    Pause,
    Stop,
    SetSource(String),
}

impl Handler<PipelineAction> for Pipeline {
    type Result = ();

    fn handle(&mut self, msg: PipelineAction, _: &mut Self::Context) -> Self::Result {
        match msg {
            PipelineAction::Play => {
                debug!("Pipeline actor received play action");

                if let Err(e) = self.play() {
                    error!("Failed to play the pipeline: {}", e);
                }
            }
            PipelineAction::Pause => {
                if let Err(e) = self.pause() {
                    error!("Failed to pause the pipeline: {}", e);
                }
            }
            PipelineAction::Stop => {
                if let Err(e) = self.stop() {
                    error!("Failed to stop the pipeline: {}", e);
                }
            }
            PipelineAction::SetSource(new_file_path) => {
                if let Ok(State::Playing) = self.query_pipeline_state() {
                    if let Err(e) = self.stop() {
                        error!("Failed to stop the pipeline: {}", e);
                    }
                }

                let source = self.source.get_element();
                source.set_property("location", &new_file_path);
                debug!("Set source to {:?}", new_file_path);
            }
        }
    }
}

// TODO: query duration with specific pipeline name
#[derive(Debug, TS, Deserialize, Serialize, Message)]
#[rtype(result = "u64")]
pub struct QueryDuration;

impl Handler<QueryDuration> for Pipeline {
    type Result = u64;

    fn handle(&mut self, _: QueryDuration, _: &mut Self::Context) -> Self::Result {
        match self.query_duration() {
            Ok(duration) => duration,
            Err(e) => {
                error!("Failed to query duration of the pipeline: {}", e);
                return 0;
            }
        }
    }
}
