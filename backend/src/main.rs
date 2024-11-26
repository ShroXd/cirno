use actix_cors::Cors;
use actix_files::Files;
use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use actors::utils::WsConnections;
use init::system_initializer::SystemInitializer;
use std::env;
use tracing::*;

mod actors;
mod handlers;

mod application;
mod domain;
mod infrastructure;
mod init;
mod interfaces;
mod shared;

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    unsafe {
        env::set_var("GST_DEBUG", "3");
        env::set_var("RUST_LOG", "actix_web=debug");
    }

    let mut initializer = match SystemInitializer::new().await {
        Ok(initializer) => initializer,
        Err(e) => {
            panic!("Failed to initialize system: {}", e);
        }
    };

    if let Err(e) = initializer.run().await {
        panic!("Failed to run system: {}", e);
    }

    let pipeline_addr = initializer.get_pipeline_addr();
    let parser_addr = initializer.get_parser_addr();
    let database_addr = initializer.get_database_addr();

    let ws_connections = WsConnections::default();

    info!("Starting backend server");
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        let mut app = App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(pipeline_addr.clone()))
            .app_data(web::Data::new(parser_addr.clone()))
            .app_data(web::Data::new(database_addr.clone()))
            .app_data(web::Data::new(ws_connections.clone()))
            .route("/hello", web::get().to(hello))
            .configure(interfaces::http_api::routes::init_routes)
            .service(Files::new("/hls", "./tmp").show_files_listing())
            .service(handlers::websocket::ws_index);

        if !cfg!(debug_assertions) {
            app = app.service(Files::new("/", "./web/dist").index_file("index.html"));
        }

        app
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
