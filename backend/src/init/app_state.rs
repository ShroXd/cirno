use actix::Addr;
use getset::Getters;
use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};

use crate::application::file_service::FileService;
use crate::infrastructure::media_db::database::Database;
use crate::infrastructure::dispatcher::event_bus::EventBus;
use crate::infrastructure::hls::hls_state_actor::HlsStateActor;
use crate::infrastructure::library_organizer::organizer::ParserActor;
use crate::infrastructure::video_pipeline::pipeline::Pipeline;
use crate::infrastructure::async_task_pool::task_pool::TaskPool;
use crate::interfaces::ws::utils::WsConnections;

use super::repository_manager::Repositories;

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

#[derive(Clone, Getters)]
#[getset(get = "pub")]
pub struct MediaProcessingContext {
    parser_addr: Addr<ParserActor>,
    #[allow(unused)]
    hls_state_actor_addr: Addr<HlsStateActor>,
}

#[derive(Clone, Getters)]
#[getset(get = "pub")]
pub struct StorageContext {
    database_addr: Addr<Database>,
    #[allow(unused)]
    file_service: FileService,
    repositories: Repositories,
}

#[derive(Clone, Getters)]
#[getset(get = "pub")]
pub struct CommunicationContext {
    ws_connections: WsConnections,
}

#[derive(Clone, Getters)]
#[getset(get = "pub")]
pub struct InfrastructureContext {
    task_pool: TaskPool,
    event_bus: Arc<EventBus>,
}

#[derive(Clone, Getters)]
#[getset(get = "pub")]
pub struct AppState {
    media: MediaProcessingContext,
    storage: StorageContext,
    communication: CommunicationContext,
    infrastructure: InfrastructureContext,
}

impl AppState {
    pub fn new(
        parser_addr: Addr<ParserActor>,
        hls_state_actor_addr: Addr<HlsStateActor>,
        database_addr: Addr<Database>,
        file_service: FileService,
        repositories: Repositories,
        ws_connections: WsConnections,
        task_pool: TaskPool,
        event_bus: Arc<EventBus>,
    ) -> Self {
        Self {
            media: MediaProcessingContext {
                parser_addr,
                hls_state_actor_addr,
            },
            storage: StorageContext {
                database_addr,
                file_service,
                repositories,
            },
            communication: CommunicationContext { ws_connections },
            infrastructure: InfrastructureContext {
                task_pool,
                event_bus,
            },
        }
    }
}
