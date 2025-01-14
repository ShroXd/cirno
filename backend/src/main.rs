use actix_cors::Cors;
use actix_files::Files;
use actix_web::{middleware::Logger, web, App, HttpServer};
use application::{file_service::FileService, pipeline_service::PipelineService};
use std::sync::Arc;
use tracing::*;

use infrastructure::{
    async_task_pool::task_pool::TaskPool, file::repository_impl::FileRepositoryImpl,
};
use init::{
    app_state::{
        AppState, CommunicationContext, InfrastructureContext, MediaProcessingContext,
        StorageContext,
    },
    system_initializer::SystemInitializer,
};
use interfaces::ws::utils::WsConnections;

mod application;
mod domain;
mod infrastructure;
mod init;
mod interfaces;
mod shared;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // unsafe {
    //     env::set_var("GST_DEBUG", "3");
    //     env::set_var(
    //         "RUST_LOG",
    //         "actix_web=info,actix_server=info,actix_rt=info,gstreamer=info",
    //     );
    // }

    let _guard = SystemInitializer::init_logger();

    let mut initializer = match SystemInitializer::new().await {
        Ok(initializer) => initializer,
        Err(e) => {
            panic!("Failed to initialize system: {}", e);
        }
    };

    if let Err(e) = initializer.run().await {
        panic!("Failed to run system: {}", e);
    }

    // let pipeline_addr = initializer.get_pipeline_addr();
    let parser_addr = initializer.get_parser_addr();
    let database_addr = initializer.get_database_addr();
    let hls_state_actor_addr = initializer.get_hls_state_actor_addr();
    let event_bus = initializer.get_event_bus();
    event_bus.start();

    let repositories = initializer.get_repositories();
    let task_pool = TaskPool::new(100, event_bus.clone());
    let ws_connections = WsConnections::default();

    // TODO: move this to system initializer
    let pipeline_service = match PipelineService::new(
        event_bus.clone(),
        // pipeline_addr.clone(),
        hls_state_actor_addr.clone(),
    ) {
        Ok(service) => service,
        Err(e) => {
            panic!("Failed to initialize pipeline service: {}", e);
        }
    };

    let file_repository = FileRepositoryImpl {};
    let file_service = FileService::new(Arc::new(file_repository));

    info!("Init app state");
    let media_context =
        MediaProcessingContext::new(parser_addr.clone(), hls_state_actor_addr.clone());
    let storage_context = StorageContext::new(
        database_addr.clone(),
        file_service.clone(),
        repositories.clone(),
    );
    let communication_context = CommunicationContext::new(ws_connections.clone());
    let infrastructure_context = InfrastructureContext::new(task_pool.clone(), event_bus.clone());

    let app_state = AppState::new(
        media_context,
        storage_context,
        communication_context,
        infrastructure_context,
    );

    info!("Starting backend server");
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        let mut app = App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(parser_addr.clone()))
            .app_data(web::Data::new(database_addr.clone()))
            .app_data(web::Data::new(ws_connections.clone()))
            .app_data(web::Data::new(task_pool.clone()))
            .app_data(web::Data::new(event_bus.clone()))
            .app_data(web::Data::new(pipeline_service.clone()))
            .app_data(web::Data::new(hls_state_actor_addr.clone()))
            .app_data(web::Data::new(file_service.clone()))
            .app_data(web::Data::new(repositories.clone()))
            .app_data(web::Data::new(app_state.clone()))
            .configure(interfaces::http_api::routes::init_library_routes)
            .configure(interfaces::http_api::routes::init_video_player_routes)
            .service(Files::new("/hls", "./tmp").show_files_listing())
            .service(interfaces::ws::routes::ws_index);

        if !cfg!(debug_assertions) {
            app = app.service(Files::new("/", "./web/dist").index_file("index.html"));
        }

        app
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
