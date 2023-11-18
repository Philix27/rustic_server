use crate::controller::question;
use actix_web::web;

pub fn routes_handler() -> actix_web::Scope {
    web::scope("/questions")
        .route("/{question_id}", web::post().to(question::create_question))
        .route("/{question_id}", web::get().to(question::get_question))
        .route("/{question_id}", web::get().to(question::get_questions))
        .route("/{question_id}", web::patch().to(question::update_question))
}
