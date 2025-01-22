use actix::prelude::*;
use anyhow::Result;
use chrono::Local;
use std::{env, path::PathBuf, sync::Arc};
use tracing::*;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{filter::LevelFilter, fmt, prelude::*};

use crate::{
    application::{file_service::FileService, pipeline_service::PipelineService},
    infrastructure::{
        async_task_pool::task_pool::TaskPool,
        event_dispatcher::event_bus::EventBus,
        file::repository_impl::FileRepositoryImpl,
        hls::hls_state_actor::HlsStateActor,
        library_organizer::organizer::ParserActor,
        media_db::{
            database::Database,
            query_manager::{FileQueryManager, QueryManager},
        },
    },
    init::{
        app_state::{
            AppState, CommunicationContext, InfrastructureContext, MediaProcessingContext,
            StorageContext,
        },
        repository_manager::RepositoryManager,
    },
    interfaces::ws::utils::WsConnections,
};

pub struct SystemInitializer {}

impl SystemInitializer {
    #[instrument]
    pub async fn new() -> Result<Self> {
        match gstreamer::init() {
            Ok(_) => info!("Gstreamer initialized"),
            Err(e) => return Err(anyhow::anyhow!("Failed to initialize gstreamer: {}", e)),
        }

        Ok(Self {})
    }

    #[instrument(skip(self))]
    pub async fn run(&mut self) -> Result<AppState> {
        let database_addr = self.init_database().await?;
        let parser_addr = self.init_parser().await?;

        let event_bus = self.init_event_bus().await?;
        event_bus.start();

        let hls_state_actor = HlsStateActor::new(event_bus.clone());
        let hls_state_actor_addr = hls_state_actor.start();

        let repository_manager = RepositoryManager::new(database_addr.clone());
        let repositories = repository_manager.init_repositories()?;

        // TODO: move this to env vars
        let task_pool = TaskPool::new(100, event_bus.clone());
        let ws_connections = WsConnections::default();

        let pipeline_service =
            match PipelineService::new(event_bus.clone(), hls_state_actor_addr.clone()) {
                Ok(pipeline_service) => pipeline_service,
                Err(e) => panic!("Failed to initialize pipeline service: {}", e),
            };

        let file_repository = FileRepositoryImpl {};
        let file_service = FileService::new(Arc::new(file_repository));

        info!("Initializing app state");
        let media_context = MediaProcessingContext::new(
            pipeline_service.clone(),
            parser_addr.clone(),
            hls_state_actor_addr.clone(),
        );
        let storage_context = StorageContext::new(
            database_addr.clone(),
            file_service.clone(),
            repositories.clone(),
        );
        let communication_context = CommunicationContext::new(ws_connections.clone());
        let infrastructure_context =
            InfrastructureContext::new(task_pool.clone(), event_bus.clone());

        let app_state = AppState::new(
            media_context,
            storage_context,
            communication_context,
            infrastructure_context,
        );

        Ok(app_state)
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
    async fn init_database(&mut self) -> Result<Addr<Database>> {
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
        let addr = database.start();

        Ok(addr)
    }

    #[instrument(skip(self))]
    async fn init_parser(&mut self) -> Result<Addr<ParserActor>> {
        info!("Initializing parser");

        let parser_actor = ParserActor;
        let addr = parser_actor.start();

        Ok(addr)
    }

    #[instrument(skip(self))]
    async fn init_event_bus(&mut self) -> Result<Arc<EventBus>> {
        info!("Initializing event bus");
        let event_bus = Arc::new(EventBus::new(100));

        Ok(event_bus)
    }
}
