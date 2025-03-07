use actix::{Actor, Context, Handler, Message};
use gstreamer::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::*;
use ts_rs::TS;

use crate::domain::pipeline::model::{PipelineState, Position};
use crate::{
    domain::pipeline::ports::PipelinePort, infrastructure::video_pipeline::pipeline::Pipeline,
};

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
    Seek(u32),
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
                debug!("Pipeline actor received pause action");

                if let Err(e) = self.pause() {
                    error!("Failed to pause the pipeline: {}", e);
                }
            }
            PipelineAction::Stop => {
                debug!("Pipeline actor received stop action");

                if let Err(e) = self.stop() {
                    error!("Failed to stop the pipeline: {}", e);
                }
            }
            PipelineAction::Seek(position) => {
                // TODO: handle u32 to u64
                let position = match Position::from_secs(position as u64) {
                    Ok(position) => position,
                    Err(e) => {
                        error!("Failed to seek the pipeline: {}", e);
                        return;
                    }
                };

                info!("Seek position: {:?}", position);
                if let Err(e) = self.seek(position) {
                    error!("Failed to seek the pipeline: {}", e);
                }
            }
            PipelineAction::SetSource(new_file_path) => {
                if let Ok(PipelineState::Playing) = self.get_state() {
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
        match self.get_duration() {
            Ok(duration) => duration.as_nanos(),
            Err(e) => {
                error!("Failed to query duration of the pipeline: {}", e);
                0
            }
        }
    }
}
