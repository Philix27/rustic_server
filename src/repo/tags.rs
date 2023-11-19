use crate::model::tag;

pub async fn create_tag( conn: &sqlx::PgPool) -> &'static str {
    "Create tags"
}

pub async fn update_tag(conn: &sqlx::PgPool) -> &'static str {
    "Update Tags"
}

pub async fn get_tags(conn: &sqlx::PgPool) -> &'static str {
    "Get Tags"
}

pub async fn get_tag(conn: &sqlx::PgPool) -> &'static str {
    "Update Tags"
}
