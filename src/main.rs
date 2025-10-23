mod db;
mod handlers;
mod models;
mod middleware;

use axum::{
    routing::{get, post, put, delete},
    Router,
};
use std::net::SocketAddr;
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv::dotenv().ok();
    
    // Setup logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize database
    let db_pool = db::init_db().await?;
    
    // Build application routes
    let app = Router::new()
        // Health check
        .route("/health", get(handlers::health_check))
        
        // File operations
        .route("/files", post(handlers::files::upload_file))
        .route("/files/:id", get(handlers::files::get_file))
        .route("/files/:id", delete(handlers::files::delete_file))
        .route("/files", get(handlers::files::list_files))
        
        // Project management
        .route("/projects", post(handlers::projects::create_project))
        .route("/projects", get(handlers::projects::list_projects))
        .route("/projects/:id", get(handlers::projects::get_project))
        .route("/projects/:id", put(handlers::projects::update_project))
        .route("/projects/:id", delete(handlers::projects::delete_project))
        
        // Code snippets
        .route("/snippets", post(handlers::snippets::create_snippet))
        .route("/snippets", get(handlers::snippets::list_snippets))
        .route("/snippets/:id", get(handlers::snippets::get_snippet))
        .route("/snippets/:id", delete(handlers::snippets::delete_snippet))
        
        // Pass database pool to all routes
        .with_state(db_pool)
        
        // Add middleware
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Server running on http://localhost:3000");
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}
