use crate::memory::store::InMemoryStore;
use crate::memory::types::*;
use std::sync::Arc;
use std::collections::HashSet;

/// Memory consolidation: decay weak memories, merge similar ones
pub struct MemoryConsolidator {
    store: Arc<InMemoryStore>,
    decay_threshold: f64,
    merge_threshold: f64,
}

impl MemoryConsolidator {
    pub fn new(store: Arc<InMemoryStore>) -> Self {
        Self {
            store,
            decay_threshold: 0.1,
            merge_threshold: 0.9,
        }
    }

    /// Run consolidation: decay + cleanup
    pub async fn consolidate(&self) -> Result<ConsolidationReport, String> {
        let mut report = ConsolidationReport::default();

        // Stage 1: Apply decay
        let decayed = self.apply_decay().await?;
        report.decayed_count = decayed.len();

        // Stage 2: Remove weak memories
        let removed = self.remove_weak().await?;
        report.removed_count = removed.len();

        report.success = true;
        Ok(report)
    }

    /// Apply time-based decay to all memories
    async fn apply_decay(&self) -> Result<Vec<uuid::Uuid>, String> {
        let stats = self.store.stats().await?;
        let memories = self.store.list(None, None, stats.total_memories).await?;
        
        let mut decayed = Vec::new();
        
        for memory in memories {
            let days_since_access = (chrono::Utc::now() - memory.last_accessed).num_days();
            if days_since_access > 0 {
                let decay_amount = memory.decay_rate * days_since_access as f64;
                let new_importance = (memory.importance - decay_amount).max(0.0);
                
                if (new_importance - memory.importance).abs() > 0.001 {
                    let update = UpdateMemoryRequest {
                        content: None,
                        pool: None,
                        type_: None,
                        confidence: None,
                        importance: Some(new_importance),
                        decay_rate: None,
                        tags: None,
                        metadata: None,
                    };
                    self.store.update(&memory.id, update).await?;
                    decayed.push(memory.id);
                }
            }
        }

        Ok(decayed)
    }

    /// Remove memories below threshold
    async fn remove_weak(&self) -> Result<Vec<uuid::Uuid>, String> {
        let stats = self.store.stats().await?;
        let memories = self.store.list(None, None, stats.total_memories).await?;
        
        let mut removed = Vec::new();
        
        for memory in memories {
            let strength = memory.current_strength();
            if strength < self.decay_threshold {
                self.store.delete(&memory.id).await?;
                removed.push(memory.id);
            }
        }

        Ok(removed)
    }

    /// Get consolidation statistics
    pub async fn get_consolidation_stats(&self) -> Result<ConsolidationStats, String> {
        let stats = self.store.stats().await?;
        let memories = self.store.list(None, None, stats.total_memories).await?;

        let mut weak_count = 0;
        let mut strong_count = 0;
        let mut total_strength = 0.0;

        for memory in &memories {
            let strength = memory.current_strength();
            total_strength += strength;
            
            if strength < self.decay_threshold {
                weak_count += 1;
            } else {
                strong_count += 1;
            }
        }

        Ok(ConsolidationStats {
            total_memories: memories.len(),
            weak_memories: weak_count,
            strong_memories: strong_count,
            avg_strength: if !memories.is_empty() {
                total_strength / memories.len() as f64
            } else {
                0.0
            },
        })
    }
}

/// Consolidation report
#[derive(Debug, Default)]
pub struct ConsolidationReport {
    pub success: bool,
    pub decayed_count: usize,
    pub removed_count: usize,
    pub similar_pairs_found: usize,
}

/// Consolidation statistics
#[derive(Debug)]
pub struct ConsolidationStats {
    pub total_memories: usize,
    pub weak_memories: usize,
    pub strong_memories: usize,
    pub avg_strength: f64,
}