//! API routes

use axum::{
    routing::{get, post, put, delete},
    Router,
};
use std::sync::Arc;

use super::handlers::{AppState, create_memory, get_memory, list_memories, update_memory, delete_memory, search_memories, get_stats, health_check};

/// Build the API router
pub fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(|| async { "Self-Evolving Memory API is running!" }))
        .route("/health", get(health_check))
        .route("/memories", post(create_memory))
        .route("/memories", get(list_memories))
        .route("/memories/search", get(search_memories))
        .route("/memories/:id", get(get_memory))
        .route("/memories/:id", put(update_memory))
        .route("/memories/:id", delete(delete_memory))
        .route("/stats", get(get_stats))
        .with_state(state)
}