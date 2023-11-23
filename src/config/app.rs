use sqlx::PgConnection;

pub struct AppState {
    pub db_conn: PgConnection,
}