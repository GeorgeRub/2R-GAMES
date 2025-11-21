// use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub async fn get_connection_to_db() -> Pool<Postgres> {
    // dotenv().ok();
    // let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://george:george@192.168.2.58:5432/two-r-games")
        .await
        .expect("Error connecting to db")
}
