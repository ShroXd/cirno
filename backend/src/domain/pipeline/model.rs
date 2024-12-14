use anyhow::*;
use gstreamer::ClockTime;
use serde::Serialize;
use std::time;

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum PipelineState {
    Null,
    Ready,
    Paused,
    Playing,
}

#[derive(Debug, Clone)]
pub struct Position(ClockTime);

impl Position {
    pub fn from_secs(secs: u64) -> Result<Self> {
        let duration = time::Duration::from_secs(secs);
        let clock_time = ClockTime::try_from(duration)?;

        Ok(Position(clock_time))
    }

    pub fn as_nanos(&self) -> u64 {
        self.0.nseconds()
    }
}

#[derive(Debug, Clone)]
pub struct Duration(ClockTime);

impl Duration {
    pub fn from_secs(secs: u64) -> Result<Self> {
        let duration = time::Duration::from_secs(secs);
        let clock_time = ClockTime::try_from(duration)?;

        Ok(Duration(clock_time))
    }

    pub fn as_nanos(&self) -> u64 {
        self.0.nseconds()
    }
}
