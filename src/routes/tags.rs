use crate::repo::tags;
use actix_web::web;

pub fn routes_handler(conn: &sqlx::PgPool) -> actix_web::Scope {
    web::scope("/tags")
        .route("/", web::get().to(tags::get_tags(conn)))
        .route("/{tags_id}", web::post().to(tags::create_tag(conn)))
        .route("/{tags_id}", web::get().to(tags::get_tag(conn)))
        .route("/{tags_id}", web::patch().to(tags::update_tag(conn)))
}
