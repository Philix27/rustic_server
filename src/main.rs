mod controller;
mod routes;

use actix_web::web;
use actix_web::{middleware::Logger, App, HttpResponse, HttpServer, Responder};
use routes::tags;
use routes::question;
use routes::ques_group;
use routes::articles;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    // env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(index))
            .service(tags::routes_handler())
            .service(question::routes_handler())
            .service(ques_group::routes_handler())
            .service(articles::routes_handler())
    })
    .bind(("127.0.0.1", 3080))?
    .run()
    .await
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, Actix web")
}
