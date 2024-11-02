use anyhow::Result;
use gstreamer::{
    glib::WeakRef, prelude::*, query, BusSyncReply, ClockTime, Element as GstElement, Format,
    GenericFormattedValue, Message as GstMessage, MessageView, Pipeline as GstPipeline, SeekFlags,
    State,
};
use std::{
    sync::{Arc, Mutex, RwLock},
    time::Duration,
};
use tokio::sync::mpsc::UnboundedReceiver;
use tracing::*;

use super::elements::{branch::StreamBranch, decode::Decoder, hlssink::HlsSink, source::Source};
use crate::services::gstreamer_pipeline::elements::decode::DecodebinSignal;

// TODO: 1. Avoid using Box<dyn Source>
// TODO: 2. Each element should have more general type instead of using WebRtcElement or other specific elements
pub struct Pipeline {
    pub source: Box<dyn Source + Send>,
    decoder: Box<dyn Decoder + Send>,
    video_branch: Box<dyn StreamBranch + Send>,
    audio_branch: Box<dyn StreamBranch + Send>,
    hls_sink: Box<dyn HlsSink + Send>,
    gst_pipeline: Option<GstPipeline>,

    state: Arc<RwLock<State>>,
}

impl Pipeline {
    pub fn new(
        source: Box<dyn Source + Send>,
        decoder: Box<dyn Decoder + Send>,
        video_branch: Box<dyn StreamBranch + Send>,
        audio_branch: Box<dyn StreamBranch + Send>,
        hls_sink: Box<dyn HlsSink + Send>,
    ) -> Self {
        Self {
            source,
            decoder,
            video_branch,
            audio_branch,
            hls_sink,
            gst_pipeline: None,
            state: Arc::new(RwLock::new(State::Null)),
        }
    }

    pub fn build(&mut self) -> Result<()> {
        // TODO: Figure out how to call new() on each element
        let source = self.source.get_element();
        let decoder = self.decoder.get_element();

        let video_branch_elements = self.video_branch.get_elements();
        let audio_branch_elements = self.audio_branch.get_elements();

        // let mpegtsmux = ElementFactory::make("mpegtsmux")
        //     .build()
        //     .map_err(|e| anyhow::anyhow!("Failed to create mpegtsmux element: {}", e))?;

        let hls_sink = self.hls_sink.get_element();
        debug!("Generate source, decoder, branches and sink elements");

        let gst_pipeline = GstPipeline::with_name("normal_pipeline");
        debug!("Pipeline created");

        let mut elements = vec![source, decoder];
        elements.extend(video_branch_elements);
        elements.extend(audio_branch_elements);
        elements.push(hls_sink);
        // elements.push(&mpegtsmux);
        gst_pipeline.add_many(&elements)?;
        debug!("Elements added to pipeline");

        GstElement::link_many(&[source, decoder])?;

        // TODO: fuck we should make sure the order of video and audio sink, otherwise using enum to store that shit
        let video_sink = &self.video_branch.get_entry();
        let audio_sink = &self.audio_branch.get_entry();
        self.decoder.handle_signal(
            DecodebinSignal::ConnectPadAdded,
            video_sink.clone(),
            audio_sink.clone(),
        );
        debug!("Signal connect-pad-added of decoder connected");

        // TODO: only fool use index to get video and audio branch, fuck
        let video_branch_elements = self.video_branch.get_elements();
        // video_branch_elements.push(sink);
        GstElement::link_many(&video_branch_elements)?;
        debug!("Video branch elements linked");

        let video_branch_elements = self.video_branch.get_elements();
        let video_parser = video_branch_elements.last().unwrap();
        let video_src_pad = video_parser.static_pad("src").unwrap();
        let hlssink_video_pad = hls_sink.request_pad_simple("video").unwrap();
        video_src_pad.link(&hlssink_video_pad)?;

        let audio_branch_elements = self.audio_branch.get_elements();
        GstElement::link_many(&audio_branch_elements)?;
        debug!("Audio branch elements linked");

        let audio_branch_elements = self.audio_branch.get_elements();
        let audio_parser = audio_branch_elements.last().unwrap();
        let audio_src_pad = audio_parser.static_pad("src").unwrap();
        let hlssink_audio_pad = hls_sink.request_pad_simple("audio").unwrap();
        audio_src_pad.link(&hlssink_audio_pad)?;

        let bus = gst_pipeline.bus().ok_or(anyhow::anyhow!("Bus not found"))?;

        let (bus_tx, bus_rx) = tokio::sync::mpsc::unbounded_channel::<GstMessage>();
        let bus_tx = Mutex::new(bus_tx);
        bus.set_sync_handler(move |_, msg| {
            let msg = msg.to_owned();
            let bus_tx = bus_tx.lock().unwrap();
            if let Err(e) = bus_tx.send(msg) {
                error!("Failed to send message to bus: {}", e);
            }
            BusSyncReply::Drop
        });

        tokio::spawn(gst_bus_watch_task(gst_pipeline.downgrade(), bus_rx));

        self.gst_pipeline = Some(gst_pipeline);
        debug!("Pipeline successfully built");
        Ok(())
    }

    pub fn play(&self) -> Result<()> {
        let gst_pipeline = match &self.gst_pipeline {
            Some(pipeline) => pipeline.clone(),
            None => return Err(anyhow::anyhow!("Pipeline not built")),
        };

        gst_pipeline.set_state(State::Playing)?;
        debug!("Pipeline playing");

        Ok(())
    }

    #[instrument(skip(self))]
    fn on_pipeline_msg(&self, msg: &GstMessage) {
        use gstreamer::MessageView;
        match msg.view() {
            MessageView::Error(err) => {
                error!(
                    "Pipeline error received from element {:?}",
                    err.src().map(|s| s.path_string())
                );
                error!("Pipeline error: {}", err.error());
            }
            _ => (),
        }
    }

    #[instrument(skip(self))]
    pub fn pause(&self) -> Result<()> {
        let gst_pipeline = self
            .gst_pipeline
            .as_ref()
            .ok_or(anyhow::anyhow!("Pipeline not built"))?;
        gst_pipeline.set_state(State::Paused)?;
        debug!("Pipeline paused");

        Ok(())
    }

    #[instrument(skip(self))]
    pub fn stop(&self) -> Result<()> {
        let gst_pipeline = self
            .gst_pipeline
            .as_ref()
            .ok_or(anyhow::anyhow!("Pipeline not built"))?;
        gst_pipeline.set_state(State::Null)?;
        debug!("Pipeline stopped");

        Ok(())
    }

    // TODO: consider if we need to use nanoseconds to represent duration and calculate position
    #[instrument(skip(self))]
    pub fn query_duration(&self) -> Result<u64> {
        let gst_pipeline = self
            .gst_pipeline
            .as_ref()
            .ok_or(anyhow::anyhow!("Pipeline not built"))?;

        let mut q = query::Duration::new(Format::Time);
        gst_pipeline.query(&mut q);
        let q = match q.result() {
            GenericFormattedValue::Time(Some(clock_time)) => clock_time,
            _ => {
                error!("Failed to get duration of the stream");
                return Err(anyhow::anyhow!("Failed to get duration of the stream"));
            }
        };

        info!("Duration: {:?}", q);
        info!("Duration: {:?}", q.nseconds());

        Ok(q.nseconds() as u64)
    }

    #[instrument(skip(self))]
    pub fn is_valid_position(&self, position: u64) -> Result<bool> {
        let duration_ns = self.query_duration()?;
        let requested_position = ClockTime::try_from(Duration::from_secs(position))?;
        let requested_position_ns = requested_position.nseconds();

        debug!("Duration: {:?} ns", duration_ns);
        info!("Requested position: {:?} ns", requested_position_ns);

        Ok(duration_ns < requested_position_ns)
    }

    #[instrument(skip(self))]
    pub fn query_pipeline_state(&self) -> Result<State> {
        let state = match self.state.read() {
            Ok(state) => state,
            Err(e) => {
                error!("Failed to read state: {:?}", e);
                return Err(anyhow::anyhow!("Failed to read state: {:?}", e));
            }
        };

        debug!("Pipeline state: {:?}", state);

        Ok(*state)
    }

    #[instrument(skip(self))]
    pub fn seek(&self, position: u64) -> Result<()> {
        if !self.is_valid_position(position)? {
            error!("Invalid position: {:?} ns", position);
            return Err(anyhow::anyhow!("Invalid position: {:?} ns", position));
        }
        if self.query_pipeline_state()? != State::Playing {
            error!("Pipeline is not playing");
            return Err(anyhow::anyhow!("Pipeline is not playing"));
        }

        let gst_pipeline = self
            .gst_pipeline
            .as_ref()
            .ok_or(anyhow::anyhow!("Pipeline not built"))?;
        let position = ClockTime::try_from(Duration::from_secs(position))?;
        match gst_pipeline.seek_simple(SeekFlags::FLUSH | SeekFlags::KEY_UNIT, position) {
            Ok(_) => info!("Seek to position {:?} success", position),
            Err(e) => error!("Seek to position {:?} failed: {}", position, e),
        }

        Ok(())
    }
}

async fn gst_bus_watch_task(
    gst_pipeline_weak: WeakRef<GstPipeline>,
    mut bus_rx: UnboundedReceiver<GstMessage>,
) {
    debug!("Event bus watch task started");
    let pipeline = match gst_pipeline_weak.upgrade() {
        Some(pipeline) => pipeline,
        None => return,
    };

    while let Some(msg) = bus_rx.recv().await {
        match msg.view() {
            MessageView::Error(e) => {
                error!(
                    "Pipeline error received from element {:?}",
                    e.src().map(|s| s.path_string())
                );
                error!("Pipeline error: {}", e.error())
            }
            MessageView::Eos(..) => {
                info!("Pipeline EOS")
            }
            MessageView::StateChanged(state_changed) => {
                if state_changed
                    .src()
                    .map(|s| s.name() == "normal_pipeline")
                    .unwrap_or(false)
                {
                    debug!(
                        "Pipeline state changed from {:?} to {:?}",
                        state_changed.old(),
                        state_changed.current(),
                    )
                }
            }
            _ => {}
        }
    }
}
