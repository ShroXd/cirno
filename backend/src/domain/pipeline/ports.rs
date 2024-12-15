use actix::Addr;
use anyhow::Result;
use gstreamer::{prelude::*, Element};
use std::fmt::Debug;
use tracing::*;

use super::model::{Duration, PipelineState, Position};
use crate::{
    infrastructure::hls::hls_state_actor::HlsStateActor, shared::utils::ElementFactoryTrait,
};

pub trait PipelinePort: Send + Sync {
    fn build(&mut self) -> Result<()>;
    fn play(&mut self) -> Result<()>;
    fn pause(&self) -> Result<()>;
    fn stop(&mut self) -> Result<()>;
    fn seek(&self, position: Position) -> Result<()>;
    fn get_duration(&self) -> Result<Duration>;
    fn get_state(&self) -> Result<PipelineState>;
}

pub trait Source: Send + Sync {
    fn new(source_path: &str) -> Result<Self>
    where
        Self: Sized;
    fn set_name(&mut self, name: &str);
    fn set_properties(&mut self, properties: Vec<(&str, &dyn ToValue)>);
    fn get_element(&self) -> &Element;

    fn build(
        name: &str,
        properties: Vec<(&str, &dyn ToValue)>,
        source_path: &str,
    ) -> Result<Element>
    where
        Self: Sized,
    {
        info!("Created pipeline source {}", name);

        let mut source = Self::new(source_path)?;
        source.set_name(name);
        source.set_properties(properties);

        Ok(source.get_element().clone())
    }
}

pub trait StreamBranch: Send + Sync {
    fn new(factory: &(impl ElementFactoryTrait + Debug)) -> Result<Self>
    where
        Self: Sized;

    fn get_entry(&self) -> Element;
    fn get_elements(&self) -> Vec<&Element>;
}

#[derive(Debug)]
pub enum DecodebinSignal {
    ConnectPadAdded,
}

pub trait Decoder: Send + Sync {
    fn new(factory: &(impl ElementFactoryTrait + Debug)) -> Result<Self>
    where
        Self: Sized;

    fn handle_signal(&mut self, signal: DecodebinSignal, video_sink: Element, audio_sink: Element);
    fn get_element(&self) -> &Element;
}

pub trait HlsSink: Send + Sync {
    fn new(hls_state_actor_addr: Addr<HlsStateActor>) -> Result<Self>
    where
        Self: Sized;
    fn get_element(&self) -> &Element;
}
