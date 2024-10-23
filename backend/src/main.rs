use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use chrono::Local;
use tracing::*;
use tracing_subscriber::{filter::LevelFilter, fmt, prelude::*};

mod actors;
mod handlers;
mod routes;
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

    info!("Starting backend server");
    HttpServer::new(|| {
        let mut app = App::new()
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
