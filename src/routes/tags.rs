use crate::{config::app::AppState, repo::tags};
use actix_web::web;
use sqlx::PgPool;

pub fn routes_handler() -> actix_web::Scope {
    // pub fn routes_handler(conn: &AppState) -> actix_web::Scope {
    web::scope("/tags")
        .route("/{tags_id}", web::post().to(tags::create_tag))
        .route("/", web::get().to(tags::get_tags))
        .route("/{tags_id}", web::get().to(tags::get_tag))
        .route("/{tags_id}", web::patch().to(tags::update_tag))
}
