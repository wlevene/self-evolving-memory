use crate::memory::store::InMemoryStore;
use crate::memory::types::*;
use std::collections::{HashMap, HashSet, BinaryHeap};
use std::cmp::Ordering;
use std::sync::Arc;

/// Spreading Activation for progressive memory retrieval
pub struct SpreadingActivation {
    store: Arc<InMemoryStore>,
    decay_factor: f64,
    min_strength: f64,
    max_depth: usize,
}

impl SpreadingActivation {
    pub fn new(store: Arc<InMemoryStore>) -> Self {
        Self {
            store,
            decay_factor: 0.7,
            min_strength: 0.1,
            max_depth: 3,
        }
    }

    /// Perform spreading activation from a set of seed memories
    pub async fn spread(&self, seed_ids: &[uuid::Uuid]) -> Result<Vec<(Memory, f64)>, String> {
        let mut visited: HashSet<uuid::Uuid> = HashSet::new();
        let mut results: HashMap<uuid::Uuid, f64> = HashMap::new();
        
        // Simple BFS with decay
        let mut queue: Vec<(uuid::Uuid, f64, usize)> = seed_ids.iter()
            .map(|&id| (id, 1.0, 0))
            .collect();
        
        while let Some((id, strength, depth)) = queue.pop() {
            if visited.contains(&id) || depth > self.max_depth {
                continue;
            }
            
            if strength < self.min_strength {
                continue;
            }
            
            visited.insert(id);
            results.insert(id, strength);
            
            // Get links and spread
            let links = self.store.get_links(&id).await?;
            for link in links {
                let other_id = if link.source_id == id {
                    link.target_id
                } else {
                    link.source_id
                };
                
                let new_strength = strength * self.decay_factor * link.strength;
                
                if new_strength >= self.min_strength && !visited.contains(&other_id) {
                    queue.push((other_id, new_strength, depth + 1));
                }
            }
        }
        
        // Fetch all memories
        let mut result_vec = Vec::new();
        for (id, strength) in results {
            if let Some(memory) = self.store.get(&id).await? {
                result_vec.push((memory, strength));
            }
        }
        
        result_vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));
        Ok(result_vec)
    }

    /// Progressive retrieval: start with search, then spread
    pub async fn progressive_search(
        &self,
        query: &str,
        initial_limit: usize,
    ) -> Result<Vec<(Memory, f64)>, String> {
        // Stage 1: Initial search
        let search_query = SearchQuery {
            query: query.to_string(),
            pool: None,
            type_: None,
            tags: None,
            limit: initial_limit,
            min_confidence: 0.0,
            include_links: false,
        };
        
        let direct_results = self.store.search(search_query).await?;
        
        // Stage 2: Spread activation
        let seed_ids: Vec<uuid::Uuid> = direct_results.iter().map(|m| m.id).collect();
        let spread_results = if !seed_ids.is_empty() {
            self.spread(&seed_ids).await?
        } else {
            Vec::new()
        };
        
        // Merge results
        let mut seen_ids: HashSet<uuid::Uuid> = HashSet::new();
        let mut all_memories: Vec<(Memory, f64)> = Vec::new();
        
        for memory in direct_results {
            seen_ids.insert(memory.id);
            all_memories.push((memory, 1.0));
        }
        
        for (memory, strength) in spread_results {
            if !seen_ids.contains(&memory.id) {
                seen_ids.insert(memory.id);
                all_memories.push((memory, strength));
            }
        }
        
        all_memories.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));
        
        Ok(all_memories)
    }
}