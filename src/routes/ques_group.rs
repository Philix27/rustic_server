use crate::controller::ques_group;
use actix_web::web;

pub fn routes_handler() -> actix_web::Scope {
    web::scope("/question_group")
        .route("/{ques_group_id}", web::post().to(ques_group::create_question_group))
        .route("/{ques_group_id}", web::get().to(ques_group::get_question_group))
        .route("/{ques_group_id}", web::get().to(ques_group::get_questions_group))
        .route("/{ques_group_id}", web::patch().to(ques_group::update_question_group))
}
