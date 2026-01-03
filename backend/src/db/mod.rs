use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;

/// Creates a PostgreSQL connection pool
pub async fn create_pool() -> Result<PgPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost:5432/spire_board".to_string());

    tracing::info!("Connecting to database: {}", database_url);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    tracing::info!("Database connection pool established");

    Ok(pool)
}

/// Runs database migrations
pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {
    tracing::info!("Running database migrations");
    sqlx::migrate!("./migrations").run(pool).await?;
    tracing::info!("Database migrations completed");
    Ok(())
}
