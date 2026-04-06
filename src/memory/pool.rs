use crate::memory::store::InMemoryStore;
use crate::memory::types::*;
use std::sync::Arc;

/// Manager for explicit and implicit memory pools
pub struct PoolManager {
    store: Arc<InMemoryStore>,
}

impl PoolManager {
    pub fn new(store: Arc<InMemoryStore>) -> Self {
        Self { store }
    }

    /// Create an explicit memory
    pub async fn create_explicit(
        &self,
        content: String,
        type_: MemoryType,
    ) -> Result<Memory, String> {
        let memory = Memory::new(content, MemoryPool::Explicit, type_);
        self.store.create(memory).await
    }

    /// Create an implicit memory
    pub async fn create_implicit(
        &self,
        content: String,
        type_: MemoryType,
    ) -> Result<Memory, String> {
        let memory = Memory::new(content, MemoryPool::Implicit, type_);
        self.store.create(memory).await
    }

    /// Get all explicit memories
    pub async fn get_explicit(&self, limit: usize) -> Result<Vec<Memory>, String> {
        self.store.list(Some("explicit".to_string()), None, limit).await
    }

    /// Get all implicit memories
    pub async fn get_implicit(&self, limit: usize) -> Result<Vec<Memory>, String> {
        self.store.list(Some("implicit".to_string()), None, limit).await
    }

    /// Search explicit memories
    pub async fn search_explicit(&self, query: String, limit: usize) -> Result<Vec<Memory>, String> {
        let search_query = SearchQuery {
            query,
            pool: Some("explicit".to_string()),
            type_: None,
            tags: None,
            limit,
            min_confidence: 0.0,
            include_links: false,
        };
        self.store.search(search_query).await
    }

    /// Search implicit memories
    pub async fn search_implicit(&self, query: String, limit: usize) -> Result<Vec<Memory>, String> {
        let search_query = SearchQuery {
            query,
            pool: Some("implicit".to_string()),
            type_: None,
            tags: None,
            limit,
            min_confidence: 0.0,
            include_links: false,
        };
        self.store.search(search_query).await
    }
}