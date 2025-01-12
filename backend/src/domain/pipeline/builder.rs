use actix::Addr;
use anyhow::*;
use std::{result::Result::Ok, sync::Arc};
use tracing::*;

use crate::{
    domain::pipeline::ports::{Decoder, HlsSink, Source, StreamBranch},
    infrastructure::{
        event_dispatcher::event_bus::EventBus,
        hls::hls_state_actor::HlsStateActor,
        video_pipeline::{
            elements::{
                branch::{AudioBranch, VideoBranch},
                decode::Decodebin,
                hlssink::HlsSinkImpl,
                source::FileSource,
            },
            pipeline::Pipeline,
        },
    },
    shared::utils::ElementFactory,
};

// Since the builder process doesn't need to hold any state currently,
// we use a functional approach here. We can migrate to OOP later if
// state management becomes necessary.
#[instrument(skip(event_bus))]
pub fn build_pipeline(
    source_path: &str,
    event_bus: Arc<EventBus>,
    hls_state_actor_addr: Addr<HlsStateActor>,
) -> Result<Pipeline> {
    debug!("Building pipeline");

    let element_factory = Arc::new(ElementFactory);

    let source = match FileSource::new(source_path) {
        Ok(source) => source,
        Err(e) => return Err(anyhow::anyhow!("Failed to create file source: {}", e)),
    };
    debug!("File source created");

    let decoder = match Decodebin::new(&*element_factory) {
        Ok(decoder) => decoder,
        Err(e) => return Err(anyhow::anyhow!("Failed to create decoder: {}", e)),
    };
    debug!("Decoder created");

    let video_branch = match VideoBranch::new(&*element_factory) {
        Ok(video_branch) => video_branch,
        Err(e) => return Err(anyhow::anyhow!("Failed to create video branch: {}", e)),
    };
    debug!("Video branch created");

    let audio_branch = match AudioBranch::new(&*element_factory) {
        Ok(audio_branch) => audio_branch,
        Err(e) => return Err(anyhow::anyhow!("Failed to create audio branch: {}", e)),
    };
    debug!("Audio branch created");

    let hls_sink = match HlsSinkImpl::new(hls_state_actor_addr.clone()) {
        Ok(hls_sink) => hls_sink,
        Err(e) => return Err(anyhow::anyhow!("Failed to initialize hls sink: {}", e)),
    };
    debug!("Hls sink created");

    let pipeline = Pipeline::new(
        Arc::new(source),
        Arc::new(decoder),
        Arc::new(video_branch),
        Arc::new(audio_branch),
        Arc::new(hls_sink),
        event_bus,
    );
    debug!("Pipeline created");

    Ok(pipeline)
}
