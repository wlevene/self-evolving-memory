//! PostgreSQL storage for memories

use crate::memory::store::MemoryStore;
use crate::memory::types::*;
use async_trait::async_trait;
use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::Row;
use uuid::Uuid;
use serde_json;
use std::collections::HashMap;

/// Helper function to convert a row to Memory
fn row_to_memory(row: sqlx::postgres::PgRow) -> Memory {
    let metadata_value: serde_json::Value = row.get("metadata");
    let metadata: HashMap<String, serde_json::Value> = 
        serde_json::from_value(metadata_value).unwrap_or_default();
    
    Memory {
        id: row.get("id"),
        content: row.get("content"),
        type_: row.get::<String, _>("type").parse().unwrap_or(MemoryType::Fact),
        pool: row.get::<String, _>("pool").parse().unwrap_or(MemoryPool::Implicit),
        confidence: row.get("confidence"),
        importance: row.get("importance"),
        decay_rate: row.get("decay_rate"),
        source: row.get("source"),
        tags: row.get("tags"),
        metadata,
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
        last_accessed: row.get("last_accessed"),
        access_count: row.get::<i64, _>("access_count") as u64,
    }
}

/// PostgreSQL storage for memories
pub struct PostgresStore {
    pool: PgPool,
}

impl PostgresStore {
    pub async fn new(database_url: &str) -> Result<Self, String> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await
            .map_err(|e| format!("Failed to connect to database: {}", e))?;
        
        Ok(Self { pool })
    }
}

#[async_trait]
impl MemoryStore for PostgresStore {
    async fn create(&self, memory: Memory) -> Result<Memory, String> {
        let metadata_json = serde_json::to_value(&memory.metadata)
            .map_err(|e| format!("Failed to serialize metadata: {}", e))?;
        
        sqlx::query(
            "INSERT INTO memories (id, content, type, pool, confidence, importance, decay_rate, source, tags, metadata, created_at, updated_at, last_accessed, access_count)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)"
        )
        .bind(memory.id)
        .bind(&memory.content)
        .bind(memory.type_.as_str())
        .bind(memory.pool.as_str())
        .bind(memory.confidence)
        .bind(memory.importance)
        .bind(memory.decay_rate)
        .bind(&memory.source)
        .bind(&memory.tags)
        .bind(metadata_json)
        .bind(memory.created_at)
        .bind(memory.updated_at)
        .bind(memory.last_accessed)
        .bind(memory.access_count as i64)
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Failed to create memory: {}", e))?;
        
        Ok(memory)
    }

    async fn get(&self, id: &Uuid) -> Result<Option<Memory>, String> {
        let row = sqlx::query(
            "UPDATE memories SET access_count = access_count + 1, last_accessed = NOW() WHERE id = $1 RETURNING *"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Failed to get memory: {}", e))?;
        
        Ok(row.map(row_to_memory))
    }

    async fn update(&self, id: &Uuid, update: UpdateMemoryRequest) -> Result<Option<Memory>, String> {
        // First check if memory exists
        let existing = self.get(id).await?;
        if existing.is_none() {
            return Ok(None);
        }
        
        let mut memory = existing.unwrap();
        
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
        
        let metadata_json = serde_json::to_value(&memory.metadata)
            .map_err(|e| format!("Failed to serialize metadata: {}", e))?;
        
        sqlx::query(
            "UPDATE memories SET content = $2, type = $3, pool = $4, confidence = $5, importance = $6, decay_rate = $7, tags = $8, metadata = $9, updated_at = $10 WHERE id = $1"
        )
        .bind(id)
        .bind(&memory.content)
        .bind(memory.type_.as_str())
        .bind(memory.pool.as_str())
        .bind(memory.confidence)
        .bind(memory.importance)
        .bind(memory.decay_rate)
        .bind(&memory.tags)
        .bind(metadata_json)
        .bind(memory.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Failed to update memory: {}", e))?;
        
        Ok(Some(memory))
    }

    async fn delete(&self, id: &Uuid) -> Result<bool, String> {
        let result = sqlx::query("DELETE FROM memories WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| format!("Failed to delete memory: {}", e))?;
        
        Ok(result.rows_affected() > 0)
    }

    async fn list(&self, pool: Option<String>, type_: Option<String>, limit: usize) -> Result<Vec<Memory>, String> {
        let mut query_str = "SELECT * FROM memories".to_string();
        let mut conditions = Vec::new();
        
        if pool.is_some() {
            conditions.push("pool = $1");
        }
        if type_.is_some() {
            conditions.push("type = $2");
        }
        
        if !conditions.is_empty() {
            query_str.push_str(" WHERE ");
            query_str.push_str(&conditions.join(" AND "));
        }
        
        query_str.push_str(&format!(" ORDER BY created_at DESC LIMIT {}", limit));
        
        let mut query = sqlx::query(&query_str);
        if let Some(p) = pool {
            query = query.bind(p);
        }
        if let Some(t) = type_ {
            query = query.bind(t);
        }
        
        let rows = query
            .fetch_all(&self.pool)
            .await
            .map_err(|e| format!("Failed to list memories: {}", e))?;
        
        Ok(rows
            .into_iter()
            .map(|row| row_to_memory(row))
            .collect())
    }

    async fn search(&self, query: SearchQuery) -> Result<Vec<Memory>, String> {
        let pattern = query.query.to_lowercase();
        
        let mut query_str = "SELECT * FROM memories WHERE LOWER(content) LIKE $1".to_string();
        let mut param_count = 2;
        
        if query.pool.is_some() {
            query_str.push_str(&format!(" AND pool = ${}", param_count));
            param_count += 1;
        }
        if query.type_.is_some() {
            query_str.push_str(&format!(" AND type = ${}", param_count));
            param_count += 1;
        }
        query_str.push_str(&format!(" AND confidence >= ${}", param_count));
        query_str.push_str(&format!(" ORDER BY confidence + importance DESC LIMIT {}", query.limit));
        
        let mut sql_query = sqlx::query(&query_str)
            .bind(format!("%{}%", pattern));
        
        if let Some(p) = query.pool {
            sql_query = sql_query.bind(p);
        }
        if let Some(t) = query.type_ {
            sql_query = sql_query.bind(t);
        }
        sql_query = sql_query.bind(query.min_confidence);
        
        let rows = sql_query
            .fetch_all(&self.pool)
            .await
            .map_err(|e| format!("Failed to search memories: {}", e))?;
        
        Ok(rows
            .into_iter()
            .map(|row| row_to_memory(row))
            .collect())
    }

    async fn create_link(&self, link: MemoryLink) -> Result<MemoryLink, String> {
        sqlx::query(
            "INSERT INTO memory_links (id, source_id, target_id, link_type, strength, created_at)
             VALUES ($1, $2, $3, $4, $5, $6)"
        )
        .bind(link.id)
        .bind(link.source_id)
        .bind(link.target_id)
        .bind(link.link_type.as_str())
        .bind(link.strength)
        .bind(link.created_at)
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Failed to create link: {}", e))?;
        
        Ok(link)
    }

    async fn get_links(&self, memory_id: &Uuid) -> Result<Vec<MemoryLink>, String> {
        let rows = sqlx::query(
            "SELECT * FROM memory_links WHERE source_id = $1 OR target_id = $1"
        )
        .bind(memory_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to get links: {}", e))?;
        
        Ok(rows
            .into_iter()
            .map(|row| MemoryLink {
                id: row.get("id"),
                source_id: row.get("source_id"),
                target_id: row.get("target_id"),
                link_type: row.get::<String, _>("link_type").parse().unwrap_or(LinkType::Related),
                strength: row.get("strength"),
                created_at: row.get("created_at"),
            })
            .collect())
    }

    async fn delete_link(&self, id: &Uuid) -> Result<bool, String> {
        let result = sqlx::query("DELETE FROM memory_links WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| format!("Failed to delete link: {}", e))?;
        
        Ok(result.rows_affected() > 0)
    }

    async fn stats(&self) -> Result<MemoryStats, String> {
        let row = sqlx::query(
            "SELECT 
                COUNT(*) as total_memories,
                COUNT(*) FILTER (WHERE pool = 'explicit') as explicit_count,
                COUNT(*) FILTER (WHERE pool = 'implicit') as implicit_count,
                AVG(confidence) as avg_confidence,
                AVG(importance) as avg_importance
             FROM memories"
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| format!("Failed to get stats: {}", e))?;
        
        let total_memories: i64 = row.get("total_memories");
        let explicit_count: i64 = row.get("explicit_count");
        let implicit_count: i64 = row.get("implicit_count");
        
        // Get type counts
        let type_rows = sqlx::query("SELECT type, COUNT(*) as count FROM memories GROUP BY type")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| format!("Failed to get type stats: {}", e))?;
        
        let mut memory_types = HashMap::new();
        for type_row in type_rows {
            let type_str: String = type_row.get("type");
            let count: i64 = type_row.get("count");
            memory_types.insert(type_str, count as usize);
        }
        
        // Get link count
        let link_row = sqlx::query("SELECT COUNT(*) as count FROM memory_links")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| format!("Failed to get link count: {}", e))?;
        
        let total_links: i64 = link_row.get("count");
        
        Ok(MemoryStats {
            total_memories: total_memories as usize,
            explicit_count: explicit_count as usize,
            implicit_count: implicit_count as usize,
            total_links: total_links as usize,
            memory_types,
            avg_confidence: row.get::<Option<f64>, _>("avg_confidence").unwrap_or(0.0),
            avg_importance: row.get::<Option<f64>, _>("avg_importance").unwrap_or(0.0),
        })
    }
}