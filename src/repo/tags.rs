use crate::model::tag;
use actix_web::Handler;
use sqlx::PgPool;

trait Handlers {
    
}
struct RepoTag {}

pub async fn create_tag() -> &'static str {
    "Create tags"
}

pub async fn update_tag() -> &'static str {
    "Update Tags"
}

pub async fn get_tags() -> &'static str  {
    "Get Tags"
}

pub async fn get_tag() -> &'static str {
    "Update Tags"
}
