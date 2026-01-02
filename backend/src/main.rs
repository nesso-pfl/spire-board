use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::env;

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

    let host = env::var("BACKEND_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("BACKEND_PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_address = format!("{}:{}", host, port);

    tracing::info!("Starting server at http://{}", bind_address);

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/health", web::get().to(health_check))
    })
    .bind(&bind_address)?
    .run()
    .await
}
