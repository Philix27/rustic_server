mod schema;
mod controller;
mod routes;
mod model;

use actix_web::web;
use actix_web::{middleware::Logger, App, HttpResponse, HttpServer, Responder};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use routes::articles;
use routes::ques_group;
use routes::question;
use routes::tags;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");


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
