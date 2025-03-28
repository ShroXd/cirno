use super::stream::HlsStream;
use crate::infrastructure::event_dispatcher::event_bus::EventBus;
use crate::infrastructure::video_pipeline::pipeline::Pipeline;
use actix::{Actor, Addr, AsyncContext, Context, Handler, Message};
use anyhow::*;
use std::collections::HashMap;
use std::sync::atomic::AtomicU32;
use std::sync::Arc;

pub struct HlsStateActor {
    // path -> playlist stream
    streams: HashMap<String, HlsStream>,
    pipeline_addr: Option<Addr<Pipeline>>,
    segment_index: AtomicU32,
    pipeline_duration: Option<u64>,
    segment_duration: Option<u64>,

    event_bus: Arc<EventBus>,
}

impl Actor for HlsStateActor {
    type Context = Context<Self>;
}

impl HlsStateActor {
    pub fn new(event_bus: Arc<EventBus>) -> Self {
        Self {
            pipeline_addr: None,
            streams: HashMap::new(),
            segment_index: AtomicU32::new(0),
            pipeline_duration: None,
            segment_duration: None,
            event_bus,
        }
    }
}

#[derive(Debug, Message)]
#[rtype(result = "Result<()>")]
pub struct Reset;

impl Handler<Reset> for HlsStateActor {
    type Result = Result<()>;

    fn handle(&mut self, _: Reset, _: &mut Self::Context) -> Self::Result {
        self.segment_index
            .store(0, std::sync::atomic::Ordering::Relaxed);
        self.pipeline_duration = None;
        self.segment_duration = None;

        Ok(())
    }
}

#[derive(Debug, Message)]
#[rtype(result = "Result<HlsStream>")]
pub struct GetPlaylistStream(pub String); // path

impl Handler<GetPlaylistStream> for HlsStateActor {
    type Result = Result<HlsStream>;

    fn handle(&mut self, msg: GetPlaylistStream, context: &mut Self::Context) -> Self::Result {
        let stream = self
            .streams
            .entry(msg.0.clone())
            .or_insert_with(|| {
                HlsStream::new(
                    msg.0.clone(),
                    context.address().clone(),
                    self.event_bus.clone(),
                )
            })
            .clone();

        Ok(stream)
    }
}

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct SetPipelineAddr(pub Addr<Pipeline>);

impl Handler<SetPipelineAddr> for HlsStateActor {
    type Result = ();

    fn handle(&mut self, msg: SetPipelineAddr, _: &mut Self::Context) -> Self::Result {
        self.pipeline_addr = Some(msg.0);
    }
}

#[derive(Debug, Message)]
#[rtype(result = "Result<Option<Addr<Pipeline>>>")]
pub struct GetPipelineAddr;

impl Handler<GetPipelineAddr> for HlsStateActor {
    type Result = Result<Option<Addr<Pipeline>>>;

    fn handle(&mut self, _: GetPipelineAddr, _: &mut Self::Context) -> Self::Result {
        Ok(self.pipeline_addr.clone())
    }
}

#[derive(Debug, Message)]
#[rtype(result = "Result<()>")]
pub struct IncrementSegmentIndex;

impl Handler<IncrementSegmentIndex> for HlsStateActor {
    type Result = Result<()>;

    fn handle(&mut self, _: IncrementSegmentIndex, _: &mut Self::Context) -> Self::Result {
        self.segment_index
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Ok(())
    }
}

#[derive(Debug, Message)]
#[rtype(result = "Result<u32>")]
pub struct GetSegmentIndex;

impl Handler<GetSegmentIndex> for HlsStateActor {
    type Result = Result<u32>;

    fn handle(&mut self, _: GetSegmentIndex, _: &mut Self::Context) -> Self::Result {
        Ok(self
            .segment_index
            .load(std::sync::atomic::Ordering::Relaxed))
    }
}

#[derive(Debug, Message)]
#[rtype(result = "Result<()>")]
pub struct SetPipelineDuration(pub u64);

impl Handler<SetPipelineDuration> for HlsStateActor {
    type Result = Result<()>;

    fn handle(&mut self, msg: SetPipelineDuration, _: &mut Self::Context) -> Self::Result {
        self.pipeline_duration = Some(msg.0);
        Ok(())
    }
}

#[derive(Debug, Message)]
#[rtype(result = "Result<u64>")]
pub struct GetPipelineDuration;

impl Handler<GetPipelineDuration> for HlsStateActor {
    type Result = Result<u64>;

    fn handle(&mut self, _: GetPipelineDuration, _: &mut Self::Context) -> Self::Result {
        Ok(self
            .pipeline_duration
            .expect("pipeline_duration is not set"))
    }
}

#[derive(Debug, Message)]
#[rtype(result = "Result<()>")]
pub struct SetSegmentDuration(pub u64);

impl Handler<SetSegmentDuration> for HlsStateActor {
    type Result = Result<()>;

    fn handle(&mut self, msg: SetSegmentDuration, _: &mut Self::Context) -> Self::Result {
        self.segment_duration = Some(msg.0);
        Ok(())
    }
}

#[derive(Debug, Message)]
#[rtype(result = "Result<u64>")]
pub struct GetSegmentDuration;

impl Handler<GetSegmentDuration> for HlsStateActor {
    type Result = Result<u64>;

    fn handle(&mut self, _: GetSegmentDuration, _: &mut Self::Context) -> Self::Result {
        Ok(self.segment_duration.expect("segment_duration is not set"))
    }
}
