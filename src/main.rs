mod config;
mod model;
mod repo;
mod routes;
mod schema;

use actix_web::web;
use actix_web::{middleware::Logger, App, HttpResponse, HttpServer, Responder};

use config::db;
use routes::articles;
use routes::ques_group;
use routes::question;
use routes::tags;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_conn = db::setup_connection().await.unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(index))
            .service(tags::routes_handler(&db_conn))
            .service(question::routes_handler())
            .service(ques_group::routes_handler())
            .service(articles::routes_handler())
    })
    .bind(("127.0.0.1", 3080))?
    .run()
    .await
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the Rustic Guide Server")
}
