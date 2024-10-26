use actix_files::Files;
use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use chrono::Local;
use init::system_initializer::SystemInitializer;
use tracing::*;
use tracing_subscriber::{filter::LevelFilter, fmt, prelude::*};

mod actors;
mod database;
mod handlers;
mod init;
mod routes;
mod services;
mod utils;

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    info!("Initializing tracing subscriber for logging");
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

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set tracing subscriber");

    // TODO: move logger initialization above to system initializer
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

    info!("Starting backend server");
    HttpServer::new(move || {
        let mut app = App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(pipeline_addr.clone()))
            .app_data(web::Data::new(parser_addr.clone()))
            .route("/hello", web::get().to(hello))
            .service(routes::websocket_routes::ws_index);

        if !cfg!(debug_assertions) {
            app = app.service(Files::new("/", "./web/dist").index_file("index.html"));
        }

        app
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
