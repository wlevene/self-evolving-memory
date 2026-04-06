//! Memory storage trait and in-memory implementation

use crate::memory::types::*;
use async_trait::async_trait;
use uuid::Uuid;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Trait for memory storage implementations
#[async_trait]
pub trait MemoryStore: Send + Sync {
    async fn create(&self, memory: Memory) -> Result<Memory, String>;
    async fn get(&self, id: &Uuid) -> Result<Option<Memory>, String>;
    async fn update(&self, id: &Uuid, update: UpdateMemoryRequest) -> Result<Option<Memory>, String>;
    async fn delete(&self, id: &Uuid) -> Result<bool, String>;
    async fn list(&self, pool: Option<String>, type_: Option<String>, limit: usize) -> Result<Vec<Memory>, String>;
    async fn search(&self, query: SearchQuery) -> Result<Vec<Memory>, String>;
    async fn create_link(&self, link: MemoryLink) -> Result<MemoryLink, String>;
    async fn get_links(&self, memory_id: &Uuid) -> Result<Vec<MemoryLink>, String>;
    async fn delete_link(&self, id: &Uuid) -> Result<bool, String>;
    async fn stats(&self) -> Result<MemoryStats, String>;
}

/// In-memory storage for memories
pub struct InMemoryStore {
    memories: Arc<RwLock<HashMap<Uuid, Memory>>>,
    links: Arc<RwLock<HashMap<Uuid, MemoryLink>>>,
}

impl InMemoryStore {
    pub fn new() -> Self {
        Self {
            memories: Arc::new(RwLock::new(HashMap::new())),
            links: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryStore {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl MemoryStore for InMemoryStore {
    async fn create(&self, memory: Memory) -> Result<Memory, String> {
        let mut memories = self.memories.write().await;
        memories.insert(memory.id, memory.clone());
        Ok(memory)
    }

    async fn get(&self, id: &Uuid) -> Result<Option<Memory>, String> {
        let mut memories = self.memories.write().await;
        if let Some(memory) = memories.get_mut(id) {
            memory.access_count += 1;
            memory.last_accessed = chrono::Utc::now();
            Ok(Some(memory.clone()))
        } else {
            Ok(None)
        }
    }

    async fn update(&self, id: &Uuid, update: UpdateMemoryRequest) -> Result<Option<Memory>, String> {
        let mut memories = self.memories.write().await;
        if let Some(memory) = memories.get_mut(id) {
            if let Some(content) = update.content {
                memory.content = content;
            }
            if let Some(pool_str) = update.pool {
                memory.pool = pool_str.parse()?;
            }
            if let Some(type_str) = update.type_ {
                memory.type_ = type_str.parse()?;
            }
            if let Some(confidence) = update.confidence {
                memory.confidence = confidence;
            }
            if let Some(importance) = update.importance {
                memory.importance = importance;
            }
            if let Some(decay_rate) = update.decay_rate {
                memory.decay_rate = decay_rate;
            }
            if let Some(tags) = update.tags {
                memory.tags = tags;
            }
            if let Some(metadata) = update.metadata {
                memory.metadata = metadata;
            }
            memory.updated_at = chrono::Utc::now();
            Ok(Some(memory.clone()))
        } else {
            Ok(None)
        }
    }

    async fn delete(&self, id: &Uuid) -> Result<bool, String> {
        let mut memories = self.memories.write().await;
        Ok(memories.remove(id).is_some())
    }

    async fn list(&self, pool: Option<String>, type_: Option<String>, limit: usize) -> Result<Vec<Memory>, String> {
        let memories = self.memories.read().await;
        let filtered: Vec<Memory> = memories
            .values()
            .filter(|m| {
                if let Some(p) = &pool {
                    if m.pool.as_str() != p {
                        return false;
                    }
                }
                if let Some(t) = &type_ {
                    if m.type_.as_str() != t {
                        return false;
                    }
                }
                true
            })
            .cloned()
            .collect();
        
        Ok(filtered.into_iter().take(limit).collect())
    }

    async fn search(&self, query: SearchQuery) -> Result<Vec<Memory>, String> {
        let memories = self.memories.read().await;
        let pattern = query.query.to_lowercase();
        
        let filtered: Vec<Memory> = memories
            .values()
            .filter(|m| {
                if !m.content.to_lowercase().contains(&pattern) {
                    return false;
                }
                if let Some(p) = &query.pool {
                    if m.pool.as_str() != p {
                        return false;
                    }
                }
                if let Some(t) = &query.type_ {
                    if m.type_.as_str() != t {
                        return false;
                    }
                }
                if m.confidence < query.min_confidence {
                    return false;
                }
                true
            })
            .cloned()
            .collect();
        
        // Sort by confidence + importance
        let mut sorted = filtered;
        sorted.sort_by(|a, b| {
            (b.confidence + b.importance).partial_cmp(&(a.confidence + a.importance)).unwrap()
        });
        
        Ok(sorted.into_iter().take(query.limit).collect())
    }

    async fn create_link(&self, link: MemoryLink) -> Result<MemoryLink, String> {
        let mut links = self.links.write().await;
        links.insert(link.id, link.clone());
        Ok(link)
    }

    async fn get_links(&self, memory_id: &Uuid) -> Result<Vec<MemoryLink>, String> {
        let links = self.links.read().await;
        Ok(links
            .values()
            .filter(|l| l.source_id == *memory_id || l.target_id == *memory_id)
            .cloned()
            .collect())
    }

    async fn delete_link(&self, id: &Uuid) -> Result<bool, String> {
        let mut links = self.links.write().await;
        Ok(links.remove(id).is_some())
    }

    async fn stats(&self) -> Result<MemoryStats, String> {
        let memories = self.memories.read().await;
        let links = self.links.read().await;
        
        let mut memory_types = HashMap::new();
        let mut explicit_count = 0;
        let mut implicit_count = 0;
        let mut total_confidence = 0.0;
        let mut total_importance = 0.0;
        
        for memory in memories.values() {
            *memory_types.entry(memory.type_.as_str().to_string()).or_insert(0) += 1;
            if memory.pool == MemoryPool::Explicit {
                explicit_count += 1;
            } else {
                implicit_count += 1;
            }
            total_confidence += memory.confidence;
            total_importance += memory.importance;
        }
        
        let count = memories.len();
        
        Ok(MemoryStats {
            total_memories: count,
            explicit_count,
            implicit_count,
            total_links: links.len(),
            memory_types,
            avg_confidence: if count > 0 { total_confidence / count as f64 } else { 0.0 },
            avg_importance: if count > 0 { total_importance / count as f64 } else { 0.0 },
        })
    }
}