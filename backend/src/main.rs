use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

mod actors;
mod handlers;
mod routes;
mod utils;

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting backend server...");
    HttpServer::new(|| {
        println!("Configuring routes...");
        let mut app = App::new().route("/hello", web::get().to(hello));

        if !cfg!(debug_assertions) {
            println!("Configuring static files...");
            app = app.service(Files::new("/", "./web/dist").index_file("index.html"));
        }

        app
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
