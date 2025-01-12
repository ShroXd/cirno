use actix::prelude::*;
use anyhow::Result;
use chrono::Local;
use std::{env, path::PathBuf, sync::Arc};
use tracing::*;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{filter::LevelFilter, fmt, prelude::*};

use crate::{
    domain::pipeline::ports::{Decoder, HlsSink, Source, StreamBranch},
    infrastructure::{
        database::{
            database::Database,
            query_manager::{FileQueryManager, QueryManager},
        },
        event_bus::event_bus::EventBus,
        hls::hls_state_actor::{HlsStateActor, SetPipelineAddr},
        organizer::organizer::ParserActor,
        pipeline::{
            elements::{
                branch::{AudioBranch, VideoBranch},
                decode::Decodebin,
                hlssink::HlsSinkImpl,
                source::FileSource,
            },
            pipeline::Pipeline,
        },
    },
    init::repository_manager::RepositoryManager,
    shared::utils::ElementFactory,
};

use super::repository_manager::Repositories;

pub struct SystemInitializer {
    _element_factory: Arc<ElementFactory>,

    event_bus: Option<Arc<EventBus>>,
    repositories: Option<Repositories>,

    // Actor addresses
    _pipeline_addr: Option<Addr<Pipeline>>,
    parser_addr: Option<Addr<ParserActor>>,
    database_addr: Option<Addr<Database>>,
    hls_state_actor_addr: Option<Addr<HlsStateActor>>,
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

        Ok(Self {
            _element_factory: element_factory,
            event_bus: None,
            repositories: None,
            _pipeline_addr: None,
            parser_addr: None,
            database_addr: None,
            hls_state_actor_addr: None,
        })
    }

    #[instrument(skip(self))]
    pub fn get_pipeline_addr(&self) -> Addr<Pipeline> {
        match self._pipeline_addr.clone() {
            Some(addr) => addr,
            None => panic!("Pipeline actor not started"),
        }
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
    pub fn get_event_bus(&self) -> Arc<EventBus> {
        match self.event_bus.clone() {
            Some(event_bus) => event_bus,
            None => panic!("Event bus not started"),
        }
    }

    #[instrument(skip(self))]
    pub fn get_hls_state_actor_addr(&self) -> Addr<HlsStateActor> {
        match self.hls_state_actor_addr.clone() {
            Some(addr) => addr,
            None => panic!("Hls state actor not started"),
        }
    }

    #[instrument(skip(self))]
    pub fn get_repositories(&self) -> Repositories {
        match self.repositories.clone() {
            Some(repositories) => repositories,
            None => panic!("Repositories not initialized"),
        }
    }

    #[instrument(skip(self))]
    pub async fn run(&mut self) -> Result<()> {
        self.init_database().await?;
        self.init_parser().await?;
        self.init_event_bus().await?;
        self.init_hls_state_actor().await?;
        self.init_repositories().await?;
        // self.init_pipeline().await?;

        Ok(())
    }

    /// TODO;
    /// Initialize a pipeline for auxiliary tasks like taking screenshots or generating preview WebRTC streams.
    /// The main playback pipeline will be built dynamically when a user plays a video.
    /// This pipeline serves as a utility pipeline for background tasks.
    #[instrument(skip(self))]
    async fn init_pipeline(&mut self) -> Result<()> {
        info!("Initializing pipeline");

        let source =
            match FileSource::new("/Users/atriiy/Animes_test/一拳超人 (2015)/S1/S01E01.mp4") {
                Ok(source) => source,
                Err(e) => return Err(anyhow::anyhow!("Failed to create file source: {}", e)),
            };
        debug!("File source created");

        let decoder = match Decodebin::new(&*self._element_factory) {
            Ok(decoder) => decoder,
            Err(e) => return Err(anyhow::anyhow!("Failed to create decoder: {}", e)),
        };
        debug!("Decoder created");

        let video_branch = match VideoBranch::new(&*self._element_factory) {
            Ok(video_branch) => video_branch,
            Err(e) => return Err(anyhow::anyhow!("Failed to create video branch: {}", e)),
        };
        debug!("Video branch created");

        let audio_branch = match AudioBranch::new(&*self._element_factory) {
            Ok(audio_branch) => audio_branch,
            Err(e) => return Err(anyhow::anyhow!("Failed to create audio branch: {}", e)),
        };
        debug!("Audio branch created");

        let event_bus = Arc::new(EventBus::new(16));

        let hls_sink = match HlsSinkImpl::new(self.get_hls_state_actor_addr().into()) {
            Ok(hls_sink) => hls_sink,
            Err(e) => return Err(anyhow::anyhow!("Failed to initialize hls sink: {}", e)),
        };

        let pipeline = Pipeline::new(
            Arc::new(source),
            Arc::new(decoder),
            Arc::new(video_branch),
            Arc::new(audio_branch),
            Arc::new(hls_sink),
            event_bus,
        );
        debug!("Pipeline created");

        // info!("Building pipeline");
        // match pipeline.build() {
        //     Ok(_) => info!("Pipeline built"),
        //     Err(e) => return Err(anyhow::anyhow!("Failed to build pipeline: {}", e)),
        // }

        // TODO: consider if this is the best way to start the pipeline
        info!("Starting pipeline actor");
        let addr = pipeline.start();
        let addr_clone = addr.clone();
        // app_state::set_pipeline_addr(addr.clone());
        self._pipeline_addr = Some(addr);

        let hls_state_actor = self.get_hls_state_actor_addr();
        match hls_state_actor.send(SetPipelineAddr(addr_clone)).await {
            Ok(_) => info!("Hls state actor set pipeline address"),
            Err(e) => {
                return Err(anyhow::anyhow!(
                    "Failed to set hls state actor pipeline address: {}",
                    e
                ))
            }
        }

        Ok(())
    }

    #[instrument]
    pub fn init_logger() -> WorkerGuard {
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

        _guard
    }

    #[instrument(skip(self))]
    async fn init_database(&mut self) -> Result<()> {
        info!("Initializing database");

        // TODO: move this to env vars
        let cargo_project_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let sql_dir = cargo_project_dir.join("sql");

        debug!("SQL directory: {:?}", sql_dir);

        let query_manager = Arc::new(FileQueryManager::new(sql_dir).await?);
        query_manager.reload().await?;

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");

        debug!("Dabase url: {:?}", database_url);

        let database = Database::new(&database_url, query_manager).await?;
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

    #[instrument(skip(self))]
    async fn init_event_bus(&mut self) -> Result<()> {
        info!("Initializing event bus");

        let event_bus = Arc::new(EventBus::new(100));
        self.event_bus = Some(event_bus);

        Ok(())
    }

    #[instrument(skip(self))]
    async fn init_hls_state_actor(&mut self) -> Result<()> {
        info!("Initializing hls state actor");

        let hls_state_actor = HlsStateActor::new(self.get_event_bus());
        let addr = hls_state_actor.start();
        self.hls_state_actor_addr = Some(addr);

        Ok(())
    }

    #[instrument(skip(self))]
    async fn init_repositories(&mut self) -> Result<()> {
        info!("Initializing repositories");

        let repository_manager = RepositoryManager::new(self.get_database_addr());
        let repositories = repository_manager.init_repositories()?;
        self.repositories = Some(repositories);

        Ok(())
    }
}
