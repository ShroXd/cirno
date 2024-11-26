use actix::Addr;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Mutex;

use crate::infrastructure::{hls::playlist::PlaylistStream, pipeline::pipeline::Pipeline};

// Global state management for the application
// TODO: This is a temporary solution for MVP. These global variables will be refactored
// once we have a better understanding of:
// 1. The final requirements for state management across modules/threads
// 2. The optimal patterns for state sharing (e.g. actor system vs web app state)
// 3. Performance and scalability needs
//
// Potential refactoring approaches:
// 1. Consolidate state into a dedicated actor that manages all application state (Preferred)
// 2. Move state into actix-web's managed state and share via request handlers
//
// Current global variables:
// PLAYLIST_STREAMS: Manages HLS playlist streams by path
// PIPELINE_ADDR: Stores address of the main GStreamer pipeline actor
// GLOBAL_SEGMENT_INDEX: Tracks the current HLS segment number
// PIPELINE_DURATION: Caches the duration of the current media pipeline

static PLAYLIST_STREAMS: Lazy<Mutex<HashMap<String, PlaylistStream>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub fn get_playlist_stream(path: String) -> PlaylistStream {
    let mut streams = match PLAYLIST_STREAMS.lock() {
        Ok(streams) => streams,
        Err(e) => panic!("Failed to lock playlist streams: {}", e),
    };
    streams
        .entry(path.clone())
        .or_insert_with(|| PlaylistStream::new(path))
        .clone()
}

static PIPELINE_ADDR: Lazy<Mutex<Option<Addr<Pipeline>>>> = Lazy::new(|| Mutex::new(None));

pub fn set_pipeline_addr(addr: Addr<Pipeline>) {
    let mut pipeline_addr = match PIPELINE_ADDR.lock() {
        Ok(pipeline_addr) => pipeline_addr,
        Err(e) => panic!("Failed to lock pipeline address: {}", e),
    };
    *pipeline_addr = Some(addr);
}

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

pub fn get_segment_index() -> u32 {
    GLOBAL_SEGMENT_INDEX.load(Ordering::Relaxed)
}

pub fn increment_segment_index() {
    GLOBAL_SEGMENT_INDEX.fetch_add(1, Ordering::Relaxed);
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

pub fn get_pipeline_duration() -> u64 {
    match PIPELINE_DURATION.lock() {
        Ok(pipeline_duration) => pipeline_duration.unwrap(),
        Err(e) => panic!("Failed to lock pipeline duration: {}", e),
    }
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
