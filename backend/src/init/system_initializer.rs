use actix::prelude::*;
use anyhow::{Context, Result};
use chrono::Local;
use gstreamer::{log::add_log_function, DebugLevel};
use std::{
    env,
    path::{Path, PathBuf},
    sync::Arc,
};
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

#[derive(Debug)]
pub struct SystemConfig {
    pub database_url: String,
    pub task_pool_size: usize,
    pub event_bus_capacity: usize,
}

impl Default for SystemConfig {
    fn default() -> Self {
        // TODO: move all configs to env vars
        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            task_pool_size: 100,
            event_bus_capacity: 100,
        }
    }
}

pub struct DatabaseBuilder {
    url: String,
    sql_dir: PathBuf,
}

impl DatabaseBuilder {
    pub fn new(url: String) -> Self {
        let sql_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("sql");
        Self { url, sql_dir }
    }

    pub async fn build(&self) -> Result<Addr<Database>> {
        let query_manager = Arc::new(FileQueryManager::new(&self.sql_dir).await?);
        query_manager.reload().await?;

        let database = Database::new(&self.url, query_manager).await?;
        Ok(database.start())
    }
}

pub struct EventBusBuilder {
    capacity: usize,
}

impl EventBusBuilder {
    pub fn new(capacity: usize) -> Self {
        Self { capacity }
    }

    pub fn build(&self) -> Arc<EventBus> {
        Arc::new(EventBus::new(self.capacity))
    }
}

pub struct SystemInitializer {
    config: SystemConfig,
}

impl SystemInitializer {
    pub fn new(config: SystemConfig) -> Self {
        Self { config }
    }

    #[instrument]
    pub fn init_logger(log_dir: &Path) -> WorkerGuard {
        let log_file_name = format!("cirno_{}", Local::now().format("%Y-%m-%d"));

        let file_appender = tracing_appender::rolling::daily(log_dir, &log_file_name);
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

        add_log_function(
            |category, level, file, function, line, object, message| match level {
                DebugLevel::Error => {
                    event!(
                        target: "gstreamer",
                        Level::ERROR,
                        category = category.name(),
                        file = file.to_string(),
                        function = function.to_string(),
                        line = line,
                        object = object.map(|o| o.to_string()),
                        "{:?}",
                        message,
                    );
                }
                DebugLevel::Warning => {
                    event!(
                        target: "gstreamer",
                        Level::WARN,
                        "{:?}",
                        message,
                    );
                }
                DebugLevel::Info => {
                    event!(
                        target: "gstreamer",
                        Level::INFO,
                        "{:?}",
                        message,
                    );
                }
                DebugLevel::Debug => {
                    event!(
                        target: "gstreamer",
                        Level::DEBUG,
                        "{:?}",
                        message,
                    );
                }
                DebugLevel::Log | DebugLevel::Trace => {
                    event!(
                        target: "gstreamer",
                        Level::TRACE,
                        "{:?}",
                        message,
                    );
                }
                _ => {
                    event!(
                        target: "gstreamer",
                        Level::INFO,
                        "{:?}",
                        message,
                    );
                }
            },
        );

        _guard
    }

    #[instrument(skip(self))]
    pub async fn initialize(self) -> Result<AppState> {
        info!("Initializing database");
        let database_addr = DatabaseBuilder::new(self.config.database_url)
            .build()
            .await
            .context("Failed to initialize database")?;

        info!("Initializing event bus");
        let event_bus = EventBusBuilder::new(self.config.event_bus_capacity).build();
        event_bus.start();

        info!("Initializing parser");
        let parser_addr = ParserActor.start();

        info!("Initializing hls state actor");
        let hls_state_actor = HlsStateActor::new(event_bus.clone());
        let hls_state_actor_addr = hls_state_actor.start();

        info!("Initializing repository manager");
        let repository_manager = RepositoryManager::new(database_addr.clone());
        let repositories = repository_manager
            .init_repositories()
            .context("Failed to initialize repositories")?;

        info!("Initializing task pool");
        let task_pool = TaskPool::new(self.config.task_pool_size, event_bus.clone());

        info!("Initializing websocket connections");
        let ws_connections = WsConnections::default();

        info!("Initializing pipeline service");
        let pipeline_service =
            PipelineService::new(event_bus.clone(), hls_state_actor_addr.clone())
                .context("Failed to initialize pipeline service")?;

        info!("Initializing file service");
        let file_service = FileService::new(Arc::new(FileRepositoryImpl {}));

        info!("Assembling application state");
        let app_state = AppState::new(
            MediaProcessingContext::new(pipeline_service, parser_addr, hls_state_actor_addr),
            StorageContext::new(database_addr, file_service, repositories),
            CommunicationContext::new(ws_connections.clone()),
            InfrastructureContext::new(task_pool.clone(), event_bus.clone()),
        );

        Ok(app_state)
    }
}
