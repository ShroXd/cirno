use anyhow::*;
use async_trait::async_trait;
use gstreamer::{
    glib::WeakRef, prelude::*, query, BusSyncReply, ClockTime, Element as GstElement, Format,
    GenericFormattedValue, Message as GstMessage, MessageView, Pipeline as GstPipeline, SeekFlags,
    State,
};
use std::{
    result::Result::Ok,
    sync::{Arc, Mutex, RwLock},
    time::Duration,
};
use tokio::sync::mpsc::UnboundedReceiver;
use tracing::*;

use crate::{
    domain::pipeline::{
        events::PipelineEvent,
        model::{Duration as DomainDuration, PipelineState, Position},
        ports::{DecodebinSignal, Decoder, HlsSink, PipelinePort, Source, StreamBranch},
    },
    infrastructure::event_bus::event_bus::EventBus,
    init::app_state::{get_pipeline_segment_duration, set_segment_index},
};

// TODO: 1. Avoid using Box<dyn Source>, use Arc<dyn Source> instead
// TODO: 2. Each element should have more general type instead of using WebRtcElement or other specific elements
pub struct Pipeline {
    pub source: Arc<dyn Source + Send>,
    decoder: Arc<dyn Decoder + Send>,
    video_branch: Arc<dyn StreamBranch + Send>,
    audio_branch: Arc<dyn StreamBranch + Send>,
    hls_sink: Arc<dyn HlsSink + Send>,
    gst_pipeline: Option<GstPipeline>,

    state: Arc<RwLock<State>>,
    event_bus: Arc<EventBus>,
}

impl Pipeline {
    pub fn new(
        source: Arc<dyn Source + Send>,
        decoder: Arc<dyn Decoder + Send>,
        video_branch: Arc<dyn StreamBranch + Send>,
        audio_branch: Arc<dyn StreamBranch + Send>,
        hls_sink: Arc<dyn HlsSink + Send>,
        event_bus: Arc<EventBus>,
    ) -> Self {
        Self {
            source,
            decoder,
            video_branch,
            audio_branch,
            hls_sink,
            gst_pipeline: None,
            state: Arc::new(RwLock::new(State::Null)),
            event_bus,
        }
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

    // #[instrument(skip(self))]
    // pub fn is_valid_position(&self, position: u64) -> Result<bool> {
    //     let duration_ns = self.get_duration()?;
    //     let requested_position = ClockTime::try_from(Duration::from_secs(position))?;
    //     let requested_position_ns = requested_position.nseconds();

    //     debug!("Duration: {:?} ns", duration_ns);
    //     info!("Requested position: {:?} ns", requested_position_ns);

    //     Ok(duration_ns < requested_position_ns)
    // }
}

#[async_trait]
impl PipelinePort for Pipeline {
    #[instrument(skip(self))]
    fn build(&mut self, path: &str) -> Result<()> {
        // TODO: Figure out how to call new() on each element
        let source = self.source.get_element();
        source.set_property("location", &path);
        debug!("Source element location set to {}", path);

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

        Arc::get_mut(&mut self.decoder).unwrap().handle_signal(
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

        tokio::spawn(gst_bus_watch_task(
            gst_pipeline.downgrade(),
            bus_rx,
            self.event_bus.clone(),
        ));

        self.gst_pipeline = Some(gst_pipeline);
        debug!("Pipeline successfully built");
        Ok(())
    }

    #[instrument(skip(self))]
    fn play(&self) -> Result<()> {
        let gst_pipeline = match &self.gst_pipeline {
            Some(pipeline) => pipeline.clone(),
            None => return Err(anyhow::anyhow!("Pipeline not built")),
        };

        gst_pipeline.set_state(State::Playing)?;
        debug!("Pipeline playing");

        Ok(())
    }

    #[instrument(skip(self))]
    fn pause(&self) -> Result<()> {
        let gst_pipeline = self
            .gst_pipeline
            .as_ref()
            .ok_or(anyhow::anyhow!("Pipeline not built"))?;
        gst_pipeline.set_state(State::Paused)?;
        debug!("Pipeline paused");

        Ok(())
    }

    #[instrument(skip(self))]
    fn stop(&self) -> Result<()> {
        let gst_pipeline = self
            .gst_pipeline
            .as_ref()
            .ok_or(anyhow::anyhow!("Pipeline not built"))?;
        gst_pipeline.set_state(State::Null)?;
        debug!("Pipeline stopped");

        Ok(())
    }

    #[instrument(skip(self))]
    fn seek(&self, position: Position) -> Result<()> {
        // if !self.is_valid_position(position)? {
        //     error!("Invalid position: {:?} ns", position);
        //     return Err(anyhow::anyhow!("Invalid position: {:?} ns", position));
        // }
        // if self.query_pipeline_state()? != State::Playing {
        //     error!("Pipeline is not playing");
        //     return Err(anyhow::anyhow!("Pipeline is not playing"));
        // }

        let gst_pipeline = self
            .gst_pipeline
            .as_ref()
            .ok_or(anyhow::anyhow!("Pipeline not built"))?;

        let duration = get_pipeline_segment_duration();
        info!("Query duration: {:?} ns", duration);

        let position_ns = ClockTime::try_from(Duration::from_secs(position.as_nanos()))?;
        let position_ns = position_ns.nseconds();
        info!("Seeked position: {:?} ns", position_ns);

        // let start_index = (position_ns / duration) + 30;
        let start_index = position_ns / duration;
        info!("Start index: {:?}", start_index);
        set_segment_index(start_index as u32);

        let position = ClockTime::try_from(Duration::from_secs(position.as_nanos()))?;
        match gst_pipeline.seek_simple(SeekFlags::KEY_UNIT, position) {
            Ok(_) => info!("Seek to position {:?} success", position),
            Err(e) => error!("Seek to position {:?} failed: {}", position, e),
        }

        Ok(())
    }

    #[instrument(skip(self))]
    fn get_duration(&self) -> Result<DomainDuration> {
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

        Ok(DomainDuration::from_secs(q.nseconds() / 1_000_000_000)?)
    }

    #[instrument(skip(self))]
    fn get_state(&self) -> Result<PipelineState> {
        let state = match self.state.read() {
            Ok(state) => state,
            Err(e) => {
                error!("Failed to read state: {:?}", e);
                return Err(anyhow::anyhow!("Failed to read state: {:?}", e));
            }
        };

        debug!("Pipeline state: {:?}", state);

        Ok(match *state {
            State::Null => PipelineState::Null,
            State::Ready => PipelineState::Ready,
            State::Paused => PipelineState::Paused,
            State::Playing => PipelineState::Playing,
            _ => PipelineState::Null,
        })
    }
}

async fn gst_bus_watch_task(
    gst_pipeline_weak: WeakRef<GstPipeline>,
    mut bus_rx: UnboundedReceiver<GstMessage>,
    event_bus: Arc<EventBus>,
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
                error!("Pipeline error: {}", e.error());
                let _ = event_bus
                    .publish(PipelineEvent::ErrorOccurred {
                        message: e.error().to_string(),
                        component: e
                            .src()
                            .map(|s| s.path_string().to_string())
                            .unwrap_or_default(),
                    })
                    .map_err(|e| error!("Failed to publish error event: {}", e));
            }
            MessageView::Eos(..) => {
                info!("End of stream received from pipeline");

                let _ = event_bus
                    .publish(PipelineEvent::EndOfStream)
                    .map_err(|e| error!("Failed to publish end of stream event: {}", e));
            }
            MessageView::StateChanged(state_changed) => {
                // if state_changed
                //     .src()
                //     .map(|s| s.name() == "normal_pipeline")
                //     .unwrap_or(false)
                // {
                //     debug!(
                //         "Pipeline state changed from {:?} to {:?}",
                //         state_changed.old(),
                //         state_changed.current(),
                //     )
                // }

                if state_changed
                    .src()
                    .map(|s| s.path_string())
                    .unwrap_or_default()
                    .contains("pipeline")
                {
                    if let (Some(old), Some(new)) = (
                        convert_gst_state(state_changed.old()),
                        convert_gst_state(state_changed.current()),
                    ) {
                        debug!("Pipeline state changed from {:?} to {:?}", old, new);
                        let _ = event_bus
                            .publish(PipelineEvent::StateChanged {
                                old_state: old,
                                new_state: new,
                            })
                            .map_err(|e| error!("Failed to publish state changed event: {}", e));
                    }
                }
            }
            _ => {}
        }
    }
}

fn convert_gst_state(state: State) -> Option<PipelineState> {
    match state {
        State::Null => Some(PipelineState::Null),
        State::Ready => Some(PipelineState::Ready),
        State::Paused => Some(PipelineState::Paused),
        State::Playing => Some(PipelineState::Playing),
        _ => None,
    }
}
