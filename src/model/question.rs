use actix_web::http::header::Date;
use diesel::prelude::*;
use crate::schema::questions;
use crate::model::ques_group;

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = questions)]
#[diesel(belongs_to(QuestionsGroup, foreign_key = id))]
pub struct Questions {
    id: String,
    question: String,
    option1: String,
    option2: String,
    option3: String,
    option4: String,
    option5: String,
    option6: String,
    answer_index: i16,
    answer_explain: String,
    created_at: Date,
    updated_at: Date,
}
