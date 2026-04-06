//! Self-Evolving Memory Service

use self_evolving_memory::api::{handlers::AppState, routes::create_routes};
use self_evolving_memory::{InMemoryStore, PostgresStore, MemoryStore};
use std::sync::Arc;
use std::net::SocketAddr;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    // Load environment variables
    let _ = dotenv();
    
    // Initialize logging
    env_logger::init();
    
    // Create store based on DATABASE_URL
    let store: Arc<dyn MemoryStore> = if let Ok(db_url) = std::env::var("DATABASE_URL") {
        println!("Connecting to PostgreSQL: {}", db_url);
        let pg_store = PostgresStore::new(&db_url)
            .await
            .expect("Failed to connect to database");
        Arc::new(pg_store)
    } else {
        println!("Using in-memory storage (no DATABASE_URL set)");
        Arc::new(InMemoryStore::new())
    };
    
    let state = Arc::new(AppState { store });
    
    // Add CORS middleware
    use tower_http::cors::{CorsLayer, Any};
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    
    // Build router
    let app = create_routes(state).layer(cors);
    
    // Get port from environment or default
    let port = std::env::var("MEM_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);
    
    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Server listening on {}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Server failed to start");
}