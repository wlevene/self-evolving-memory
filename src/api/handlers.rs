//! API handlers for memory operations

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use uuid::Uuid;
use std::sync::Arc;

use crate::memory::{MemoryStore, Memory, CreateMemoryRequest, UpdateMemoryRequest, SearchQuery};

/// Application state
#[derive(Clone)]
pub struct AppState {
    pub store: Arc<dyn MemoryStore>,
}

/// Create memory handler
pub async fn create_memory(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateMemoryRequest>,
) -> Result<Json<Memory>, (StatusCode, String)> {
    let memory = req.into_memory()
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;
    
    let created = state.store.create(memory).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;
    
    Ok(Json(created))
}

/// Get memory handler
pub async fn get_memory(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Memory>, (StatusCode, String)> {
    let memory = state.store.get(&id).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;
    
    match memory {
        Some(m) => Ok(Json(m)),
        None => Err((StatusCode::NOT_FOUND, format!("Memory not found: {}", id))),
    }
}

/// List memories handler
pub async fn list_memories(
    State(state): State<Arc<AppState>>,
    Query(params): Query<ListParams>,
) -> Result<Json<Vec<Memory>>, (StatusCode, String)> {
    let limit = params.limit.unwrap_or(100);
    let memories = state.store.list(params.pool, params.type_, limit).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;
    
    Ok(Json(memories))
}

#[derive(Debug, Deserialize)]
pub struct ListParams {
    pub pool: Option<String>,
    pub type_: Option<String>,
    pub limit: Option<usize>,
}

/// Update memory handler
pub async fn update_memory(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateMemoryRequest>,
) -> Result<Json<Memory>, (StatusCode, String)> {
    let memory = state.store.update(&id, req).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;
    
    match memory {
        Some(m) => Ok(Json(m)),
        None => Err((StatusCode::NOT_FOUND, format!("Memory not found: {}", id))),
    }
}

/// Delete memory handler
pub async fn delete_memory(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    let deleted = state.store.delete(&id).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;
    
    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err((StatusCode::NOT_FOUND, format!("Memory not found: {}", id)))
    }
}

/// Search memories handler
pub async fn search_memories(
    State(state): State<Arc<AppState>>,
    Json(query): Json<SearchQuery>,
) -> Result<Json<Vec<Memory>>, (StatusCode, String)> {
    let memories = state.store.search(query).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;
    
    Ok(Json(memories))
}

/// Get stats handler
pub async fn get_stats(
    State(state): State<Arc<AppState>>,
) -> Result<Json<crate::memory::MemoryStats>, (StatusCode, String)> {
    let stats = state.store.stats().await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;
    
    Ok(Json(stats))
}

/// Health check handler
pub async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "service": "self-evolving-memory"
    }))
}