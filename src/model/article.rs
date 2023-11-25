use diesel::sql_types::Date;
use crate::model::tag;

pub struct Article<'a> {
    title: String,
    subtitle: String,
    content: String,
    tags: Vec<tag::Tag<'a>>,
    created_at: Date,
    updated_at: Date,
}
