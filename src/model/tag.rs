use diesel::sql_types::Date;

pub struct Tag {
    title: String,
    subtitle: String,
    created_at: Date,
    updated_at: Date,
}
