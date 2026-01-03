use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;

/// PostgreSQL接続プールを作成
pub async fn create_pool() -> Result<PgPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost:5432/spire_board".to_string());

    tracing::info!("データベースに接続中: {}", database_url);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    tracing::info!("データベース接続プールを確立しました");

    Ok(pool)
}

/// データベースマイグレーションを実行
pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {
    tracing::info!("データベースマイグレーション実行中");
    sqlx::migrate!("./migrations").run(pool).await?;
    tracing::info!("データベースマイグレーション完了");
    Ok(())
}
