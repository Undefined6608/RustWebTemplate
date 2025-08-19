use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::time::Duration;

pub type DbPool = Pool<Postgres>;

pub async fn create_pool(database_url: &str) -> anyhow::Result<DbPool> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(30))
        .connect(database_url)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}
