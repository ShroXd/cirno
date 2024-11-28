use anyhow::*;

use super::model::{Duration, PipelineState};
use crate::domain::stream::ports::StreamType;

pub trait PipelinePort: Send + Sync {
    fn build(&mut self) -> Result<()>;
    fn play(&self) -> Result<()>;
    fn pause(&self) -> Result<()>;
    fn stop(&self) -> Result<()>;
    // TODO: Change to Position
    fn seek(&self, position: u32) -> Result<()>;
    fn get_duration(&self) -> Result<Duration>;
    fn get_state(&self) -> Result<PipelineState>;
}

pub trait Source: Send + Sync {
    async fn get_uri(&self) -> Result<String>;
}

pub trait Decoder: Send + Sync {
    fn get_name(&self) -> &str;
    fn supports_format(&self, format: &str) -> bool;
}

pub trait StreamBranch: Send + Sync {
    fn get_type(&self) -> StreamType;
    fn get_caps(&self) -> Option<&str>;
}
