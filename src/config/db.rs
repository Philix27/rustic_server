use dotenvy::dotenv;
use sqlx::{postgres::PgConnectOptions, ConnectOptions, Connection, Error, PgConnection};
use std::env;

pub async fn setup_connection() -> Option<PgConnection> {
    // Get database link
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // Get connection to db

    let mut connection_pool = sqlx::postgres::PgConnection::connect(&database_url).await;

    // Run migrations
    // sqlx::migrate!("./migrations").run(&conn).await;
    let conn = PgConnection::connect(&database_url).await;

    match conn {
        Ok(conn) => Some(conn),
        Err(_conn) => None,
    }
}
