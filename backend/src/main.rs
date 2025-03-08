use actix_cors::Cors;
use actix_files::Files;
use actix_web::{middleware::Logger, web, App, HttpServer};
use std::path::Path;
use tracing::*;

use init::system_initializer::{SystemConfig, SystemInitializer};

mod application;
mod domain;
mod infrastructure;
mod init;
mod interfaces;
mod shared;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    info!("Initializing gstreamer");
    gstreamer::init().expect("Failed to initialize gstreamer");

    let _guard = SystemInitializer::init_logger(Path::new("logs"));

    info!("Initializing system");
    let initializer = SystemInitializer::new(SystemConfig::default());
    let app_state = match initializer.initialize().await {
        Ok(app_state) => app_state,
        Err(e) => panic!("Failed to initialize system: {}", e),
    };

    info!("Starting backend server");
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        let mut app = App::new()
            .wrap(Logger::default())
            .wrap(cors)
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
