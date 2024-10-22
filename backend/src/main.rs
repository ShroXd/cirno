use actix_web::{web, App, HttpServer};

mod actors;
mod handlers;
mod routes;
mod utils;

async fn hello() -> &'static str {
    "Hello, world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(hello)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
