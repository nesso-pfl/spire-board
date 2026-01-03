use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::env;

mod db;
mod handlers;
mod models;

async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "service": "spire-board-backend"
    }))
}

async fn index() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "message": "Welcome to spire-board API",
        "version": "0.1.0"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    // Create database connection pool
    let pool = db::create_pool()
        .await
        .expect("Failed to create database pool");

    // Run migrations
    db::run_migrations(&pool)
        .await
        .expect("Failed to run migrations");

    let host = env::var("BACKEND_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("BACKEND_PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_address = format!("{}:{}", host, port);

    tracing::info!("Starting server at http://{}", bind_address);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(index))
            .route("/health", web::get().to(health_check))
            // Player card endpoints
            .route("/api/player-cards", web::get().to(handlers::player_card_handler::list_cards))
            .route("/api/player-cards", web::post().to(handlers::player_card_handler::create_card))
            .route("/api/player-cards/{id}", web::get().to(handlers::player_card_handler::get_card))
            .route("/api/player-cards/{id}", web::put().to(handlers::player_card_handler::update_card))
            .route("/api/player-cards/{id}", web::delete().to(handlers::player_card_handler::delete_card))
            .route("/api/player-cards/color/{color}", web::get().to(handlers::player_card_handler::get_cards_by_color))
            .route("/api/player-cards/upgraded/{base_card_id}", web::get().to(handlers::player_card_handler::get_upgraded_card))
    })
    .bind(&bind_address)?
    .run()
    .await
}
