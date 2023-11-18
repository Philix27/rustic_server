use crate::controller::articles;
use actix_web::web;

pub fn routes_handler() -> actix_web::Scope {
    web::scope("/articles")
        .route("/{article_id}", web::post().to(articles::create_articles))
        .route("/{article_id}", web::get().to(articles::get_article))
        .route("/{article_id}", web::get().to(articles::get_articles))
        .route("/{article_id}", web::patch().to(articles::update_articles))
}
