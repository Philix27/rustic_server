use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub async fn setup_connection() -> PgConnection {
    // Get database link
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // Get connection to db
    PgConnection::establish(&database_url).expect(&format!("Error, connecting to {}", database_url))
}


