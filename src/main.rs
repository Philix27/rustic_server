mod routes;
mod controller;

use actix_web::web;
use actix_web::{middleware::Logger, App, HttpResponse, HttpServer, Responder};
use routes::tags;
use routes::tasks::transactions_routes_handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    // env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(index))
            .service(transactions_routes_handler())
            .service(tags::routes_handler())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, Actix web")
}
