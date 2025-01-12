use actix::Addr;
use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Mutex;

use crate::infrastructure::pipeline::pipeline::Pipeline;

// TODO: all of these functionalities should be moved to the hls module

static PIPELINE_ADDR: Lazy<Mutex<Option<Addr<Pipeline>>>> = Lazy::new(|| Mutex::new(None));

pub fn get_pipeline_addr() -> Addr<Pipeline> {
    match PIPELINE_ADDR.lock() {
        Ok(pipeline_addr) => pipeline_addr.clone().unwrap(),
        Err(e) => panic!("Failed to lock pipeline address: {}", e),
    }
}

static GLOBAL_SEGMENT_INDEX: Lazy<AtomicU32> = Lazy::new(|| AtomicU32::new(0));

pub fn set_segment_index(value: u32) {
    GLOBAL_SEGMENT_INDEX.store(value, Ordering::Relaxed);
}

// TODO: optimize this, we should query the duration from the pipeline
static PIPELINE_DURATION: Lazy<Mutex<Option<u64>>> = Lazy::new(|| Mutex::new(None));

pub fn set_pipeline_duration(duration: u64) {
    let mut pipeline_duration = match PIPELINE_DURATION.lock() {
        Ok(pipeline_duration) => pipeline_duration,
        Err(e) => panic!("Failed to lock pipeline duration: {}", e),
    };
    *pipeline_duration = Some(duration);
}

static PIPELINE_SEGMENT_DURATION: Lazy<Mutex<Option<u64>>> = Lazy::new(|| Mutex::new(None));

pub fn set_pipeline_segment_duration(duration: u64) {
    let mut pipeline_segment_duration = match PIPELINE_SEGMENT_DURATION.lock() {
        Ok(pipeline_segment_duration) => pipeline_segment_duration,
        Err(e) => panic!("Failed to lock pipeline segment duration: {}", e),
    };
    *pipeline_segment_duration = Some(duration);
}

pub fn get_pipeline_segment_duration() -> u64 {
    match PIPELINE_SEGMENT_DURATION.lock() {
        Ok(pipeline_segment_duration) => pipeline_segment_duration.unwrap(),
        Err(e) => panic!("Failed to lock pipeline segment duration: {}", e),
    }
}
