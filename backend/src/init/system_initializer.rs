use actix::prelude::*;
use anyhow::Result;
use chrono::Local;
use gstreamer::prelude::*;
use once_cell::sync::Lazy;
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc, Mutex,
    },
};
use tracing::*;
use tracing_subscriber::{filter::LevelFilter, fmt, prelude::*};

use crate::{
    actors::parser_actor::ParserActor,
    database::database::Database,
    services::{
        gstreamer_pipeline::{
            elements::{
                branch::{AudioBranch, StreamBranch, VideoBranch},
                decode::{Decodebin, Decoder},
                hlssink::{HlsSink, HlsSinkImpl},
                source::{FileSource, Source},
            },
            pipeline::Pipeline,
        },
        stream::playlist_stream::PlaylistStream,
    },
    utils::gst::ElementFactory,
};

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

pub struct SystemInitializer {
    database: Database,
    element_factory: Arc<ElementFactory>,
    hls_sink: Arc<HlsSinkImpl>,

    // Actor addresses
    pipeline_addr: Option<Addr<Pipeline>>,
    parser_addr: Option<Addr<ParserActor>>,
}

impl SystemInitializer {
    #[instrument]
    pub async fn new() -> Result<Self> {
        match gstreamer::init() {
            Ok(_) => info!("Gstreamer initialized"),
            Err(e) => return Err(anyhow::anyhow!("Failed to initialize gstreamer: {}", e)),
        }

        // TODO: remove this
        let element_factory = Arc::new(ElementFactory);

        let hls_sink = match HlsSinkImpl::new() {
            Ok(hls_sink) => hls_sink,
            Err(e) => return Err(anyhow::anyhow!("Failed to initialize hls sink: {}", e)),
        };

        let database = Database::new("media_library.db").await?;

        Ok(Self {
            database,
            element_factory,
            hls_sink: Arc::new(hls_sink),
            pipeline_addr: None,
            parser_addr: None,
        })
    }

    #[instrument(skip(self))]
    pub fn get_pipeline_addr(&self) -> Addr<Pipeline> {
        get_pipeline_addr()
    }

    #[instrument(skip(self))]
    pub fn get_parser_addr(&self) -> Addr<ParserActor> {
        match self.parser_addr.clone() {
            Some(addr) => addr,
            None => panic!("Parser actor not started"),
        }
    }

    #[instrument(skip(self))]
    pub async fn run(&mut self) -> Result<()> {
        self.init_logger().await?;
        self.init_database().await?;
        self.init_parser().await?;
        self.init_pipeline().await?;

        Ok(())
    }

    #[instrument(skip(self))]
    async fn init_pipeline(&mut self) -> Result<()> {
        info!("Initializing pipeline");

        let source = match FileSource::new() {
            Ok(source) => source,
            Err(e) => return Err(anyhow::anyhow!("Failed to create file source: {}", e)),
        };
        debug!("File source created");

        // TODO: temporary hardcode the video path
        let video_path = "/Users/atriiy/Animes_test/一拳超人 (2015)/S1/S01E01.mp4";
        source.element.set_property("location", &video_path);
        debug!("File source location set to {}", video_path);

        let decoder = match Decodebin::new(&*self.element_factory) {
            Ok(decoder) => decoder,
            Err(e) => return Err(anyhow::anyhow!("Failed to create decoder: {}", e)),
        };
        debug!("Decoder created");

        let video_branch = match VideoBranch::new(&*self.element_factory) {
            Ok(video_branch) => video_branch,
            Err(e) => return Err(anyhow::anyhow!("Failed to create video branch: {}", e)),
        };
        debug!("Video branch created");

        let audio_branch = match AudioBranch::new(&*self.element_factory) {
            Ok(audio_branch) => audio_branch,
            Err(e) => return Err(anyhow::anyhow!("Failed to create audio branch: {}", e)),
        };
        debug!("Audio branch created");

        let mut pipeline = Pipeline::new(
            Box::new(source),
            Box::new(decoder),
            Box::new(video_branch),
            Box::new(audio_branch),
            Box::new(self.hls_sink.as_ref().clone()),
        );
        debug!("Pipeline created");

        info!("Building pipeline");
        match pipeline.build() {
            Ok(_) => info!("Pipeline built"),
            Err(e) => return Err(anyhow::anyhow!("Failed to build pipeline: {}", e)),
        }

        // TODO: consider if this is the best way to start the pipeline
        info!("Starting pipeline actor");
        let addr = pipeline.start();
        set_pipeline_addr(addr.clone());

        Ok(())
    }

    #[instrument(skip(self))]
    async fn init_logger(&self) -> Result<()> {
        let file_name = format!("cirno_{}", Local::now().format("%Y-%m-%d"));
        let file_appender = tracing_appender::rolling::daily("logs", &file_name);
        let (non_blocking_writer, _guard) = tracing_appender::non_blocking(file_appender);

        let subscriber = tracing_subscriber::registry()
            .with(fmt::layer().with_writer(non_blocking_writer))
            .with(
                fmt::layer()
                    .with_writer(std::io::stdout)
                    .with_filter(LevelFilter::DEBUG),
            );

        tracing::subscriber::set_global_default(subscriber)
            .expect("Failed to set tracing subscriber");

        info!("Tracing subscriber initialized");

        Ok(())
    }

    #[instrument(skip(self))]
    async fn init_database(&self) -> Result<()> {
        info!("Initializing database");
        self.database.initialize_db().await?;

        Ok(())
    }

    #[instrument(skip(self))]
    async fn init_parser(&mut self) -> Result<()> {
        info!("Initializing parser");

        let parser_actor = ParserActor::default();
        let addr = parser_actor.start();
        self.parser_addr = Some(addr);

        Ok(())
    }
}
