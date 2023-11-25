use diesel::prelude::*;
use crate::schema::question_group;
use diesel::sql_types::Date;
use crate::model::question::Questions;


#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = question_group)]
pub struct QuestionsGroup {
    id: String,
    title: String,
    subtitle: String,
    created_at: String,
    updated_at: String,
}
