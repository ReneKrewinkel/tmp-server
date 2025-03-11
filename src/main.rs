use actix_web::{get, App, HttpServer};
mod env;
mod firebs;
mod openai;
mod server;

use server::services;

#[get("/")]
async fn index() -> String {
    "server running".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().service(index).configure(services::config))
        .bind(("0.0.0.0", 666))?
        .run()
        .await
}
