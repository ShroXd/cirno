use actix::prelude::*;
use anyhow::Result;
use chrono::Local;
use gstreamer::prelude::*;
use std::sync::Arc;
use tracing::*;
use tracing_subscriber::{filter::LevelFilter, fmt, prelude::*};

use crate::{
    infrastructure::{
        database::database::Database,
        organizer::organizer::ParserActor,
        pipeline::{
            elements::{
                branch::{AudioBranch, StreamBranch, VideoBranch},
                decode::{Decodebin, Decoder},
                hlssink::{HlsSink, HlsSinkImpl},
                source::{FileSource, Source},
            },
            pipeline::Pipeline,
        },
    },
    init::app_state,
    shared::utils::ElementFactory,
};

pub struct SystemInitializer {
    element_factory: Arc<ElementFactory>,
    hls_sink: Arc<HlsSinkImpl>,

    // Actor addresses
    pipeline_addr: Option<Addr<Pipeline>>,
    parser_addr: Option<Addr<ParserActor>>,
    database_addr: Option<Addr<Database>>,
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

        Ok(Self {
            element_factory,
            hls_sink: Arc::new(hls_sink),
            pipeline_addr: None,
            parser_addr: None,
            database_addr: None,
        })
    }

    #[instrument(skip(self))]
    pub fn get_pipeline_addr(&self) -> Addr<Pipeline> {
        app_state::get_pipeline_addr()
    }

    #[instrument(skip(self))]
    pub fn get_parser_addr(&self) -> Addr<ParserActor> {
        match self.parser_addr.clone() {
            Some(addr) => addr,
            None => panic!("Parser actor not started"),
        }
    }

    #[instrument(skip(self))]
    pub fn get_database_addr(&self) -> Addr<Database> {
        match self.database_addr.clone() {
            Some(addr) => addr,
            None => panic!("Database actor not started"),
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
        app_state::set_pipeline_addr(addr.clone());

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
    async fn init_database(&mut self) -> Result<()> {
        info!("Initializing database");

        let database = Database::new("media_library.db").await?;
        self.database_addr = Some(database.start());

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
