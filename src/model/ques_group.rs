use diesel::sql_types::Date;
use crate::model::question::Questions;

pub struct QuestionsGroup {
    title: String,
    subtitle: String,
    questions: Vec<Questions>,
    created_at: Date,
    updated_at: Date,
}
