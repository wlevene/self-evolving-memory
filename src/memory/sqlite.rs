//! SQLite storage for memories

use crate::memory::store::MemoryStore;
use crate::memory::types::*;
use async_trait::async_trait;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions, SqliteRow};
use sqlx::Row;
use uuid::Uuid;
use serde_json;
use std::collections::HashMap;

/// Helper function to convert a row to Memory
fn row_to_memory(row: SqliteRow) -> Memory {
    let metadata_str: String = row.get("metadata");
    let metadata: HashMap<String, serde_json::Value> = 
        serde_json::from_str(&metadata_str).unwrap_or_default();
    
    let tags_str: String = row.get("tags");
    let tags: Vec<String> = serde_json::from_str(&tags_str).unwrap_or_default();

    let id_str: String = row.get("id");
    let id = Uuid::parse_str(&id_str).unwrap_or_default();

    Memory {
        id,
        content: row.get("content"),
        type_: row.get::<String, _>("type").parse().unwrap_or(MemoryType::Fact),
        pool: row.get::<String, _>("pool").parse().unwrap_or(MemoryPool::Implicit),
        confidence: row.get("confidence"),
        importance: row.get("importance"),
        decay_rate: row.get("decay_rate"),
        source: row.get("source"),
        tags,
        metadata,
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
        last_accessed: row.get("last_accessed"),
        access_count: row.get::<i64, _>("access_count") as u64,
    }
}

/// SQLite storage for memories
pub struct SqliteStore {
    pool: SqlitePool,
}

impl SqliteStore {
    pub async fn new(database_url: &str) -> Result<Self, String> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await
            .map_err(|e| format!("Failed to connect to database: {}", e))?;
        
        // Ensure tables exist
        let schema = include_str!("../../migrations/001_sqlite_init.sql");
        sqlx::query(schema)
            .execute(&pool)
            .await
            .map_err(|e| format!("Failed to initialize database schema: {}", e))?;
            
        Ok(Self { pool })
    }
}

#[async_trait]
impl MemoryStore for SqliteStore {
    async fn create(&self, memory: Memory) -> Result<Memory, String> {
        let metadata_json = serde_json::to_string(&memory.metadata)
            .map_err(|e| format!("Failed to serialize metadata: {}", e))?;
        let tags_json = serde_json::to_string(&memory.tags)
            .map_err(|e| format!("Failed to serialize tags: {}", e))?;
        let id_str = memory.id.to_string();
        
        sqlx::query(
            "INSERT INTO memories (id, content, type, pool, confidence, importance, decay_rate, source, tags, metadata, created_at, updated_at, last_accessed, access_count)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&id_str)
        .bind(&memory.content)
        .bind(memory.type_.as_str())
        .bind(memory.pool.as_str())
        .bind(memory.confidence)
        .bind(memory.importance)
        .bind(memory.decay_rate)
        .bind(&memory.source)
        .bind(&tags_json)
        .bind(&metadata_json)
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
        let id_str = id.to_string();
        
        sqlx::query(
            "UPDATE memories SET access_count = access_count + 1, last_accessed = CURRENT_TIMESTAMP WHERE id = ?"
        )
        .bind(&id_str)
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Failed to update memory access: {}", e))?;

        let row = sqlx::query(
            "SELECT * FROM memories WHERE id = ?"
        )
        .bind(&id_str)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Failed to get memory: {}", e))?;
        
        Ok(row.map(row_to_memory))
    }

    async fn update(&self, id: &Uuid, update: UpdateMemoryRequest) -> Result<Option<Memory>, String> {
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
        
        let metadata_json = serde_json::to_string(&memory.metadata)
            .map_err(|e| format!("Failed to serialize metadata: {}", e))?;
        let tags_json = serde_json::to_string(&memory.tags)
            .map_err(|e| format!("Failed to serialize tags: {}", e))?;
        let id_str = memory.id.to_string();
        
        sqlx::query(
            "UPDATE memories SET content = ?, type = ?, pool = ?, confidence = ?, importance = ?, decay_rate = ?, tags = ?, metadata = ?, updated_at = ? WHERE id = ?"
        )
        .bind(&memory.content)
        .bind(memory.type_.as_str())
        .bind(memory.pool.as_str())
        .bind(memory.confidence)
        .bind(memory.importance)
        .bind(memory.decay_rate)
        .bind(&tags_json)
        .bind(&metadata_json)
        .bind(memory.updated_at)
        .bind(&id_str)
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Failed to update memory: {}", e))?;
        
        Ok(Some(memory))
    }

    async fn delete(&self, id: &Uuid) -> Result<bool, String> {
        let id_str = id.to_string();
        let result = sqlx::query("DELETE FROM memories WHERE id = ?")
            .bind(&id_str)
            .execute(&self.pool)
            .await
            .map_err(|e| format!("Failed to delete memory: {}", e))?;
            
        Ok(result.rows_affected() > 0)
    }

    async fn list(&self, pool: Option<String>, type_: Option<String>, limit: usize) -> Result<Vec<Memory>, String> {
        let mut query = String::from("SELECT * FROM memories WHERE 1=1");
        
        if pool.is_some() {
            query.push_str(" AND pool = ?");
        }
        if type_.is_some() {
            query.push_str(" AND type = ?");
        }
        
        query.push_str(" ORDER BY created_at DESC LIMIT ?");
        
        let mut q = sqlx::query(&query);
        
        if let Some(p) = &pool {
            q = q.bind(p);
        }
        if let Some(t) = &type_ {
            q = q.bind(t);
        }
        q = q.bind(limit as i64);
        
        let rows = q.fetch_all(&self.pool).await
            .map_err(|e| format!("Failed to list memories: {}", e))?;
            
        Ok(rows.into_iter().map(row_to_memory).collect())
    }

    async fn search(&self, query: SearchQuery) -> Result<Vec<Memory>, String> {
        let mut sql = String::from("SELECT * FROM memories WHERE content LIKE ?");
        let search_term = format!("%{}%", query.query);
        
        if query.pool.is_some() {
            sql.push_str(" AND pool = ?");
        }
        if query.type_.is_some() {
            sql.push_str(" AND type = ?");
        }
        sql.push_str(" AND confidence >= ?");
        sql.push_str(" ORDER BY importance DESC, created_at DESC LIMIT ?");
        
        let mut q = sqlx::query(&sql).bind(&search_term);
        
        if let Some(p) = &query.pool {
            q = q.bind(p);
        }
        if let Some(t) = &query.type_ {
            q = q.bind(t);
        }
        
        q = q.bind(query.min_confidence)
             .bind(query.limit as i64);
             
        let rows = q.fetch_all(&self.pool).await
            .map_err(|e| format!("Failed to search memories: {}", e))?;
            
        Ok(rows.into_iter().map(row_to_memory).collect())
    }

    async fn create_link(&self, link: MemoryLink) -> Result<MemoryLink, String> {
        let id_str = link.id.to_string();
        let source_id_str = link.source_id.to_string();
        let target_id_str = link.target_id.to_string();
        
        sqlx::query(
            "INSERT INTO memory_links (id, source_id, target_id, link_type, strength, created_at)
             VALUES (?, ?, ?, ?, ?, ?)"
        )
        .bind(&id_str)
        .bind(&source_id_str)
        .bind(&target_id_str)
        .bind(link.link_type.as_str())
        .bind(link.strength)
        .bind(link.created_at)
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Failed to create link: {}", e))?;
        
        Ok(link)
    }

    async fn get_links(&self, memory_id: &Uuid) -> Result<Vec<MemoryLink>, String> {
        let id_str = memory_id.to_string();
        
        let rows = sqlx::query(
            "SELECT * FROM memory_links WHERE source_id = ? OR target_id = ?"
        )
        .bind(&id_str)
        .bind(&id_str)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to get links: {}", e))?;
        
        let mut links = Vec::new();
        for row in rows {
            let id_str: String = row.get("id");
            let source_str: String = row.get("source_id");
            let target_str: String = row.get("target_id");
            
            links.push(MemoryLink {
                id: Uuid::parse_str(&id_str).unwrap_or_default(),
                source_id: Uuid::parse_str(&source_str).unwrap_or_default(),
                target_id: Uuid::parse_str(&target_str).unwrap_or_default(),
                link_type: row.get::<String, _>("link_type").parse().unwrap_or(LinkType::Related),
                strength: row.get("strength"),
                created_at: row.get("created_at"),
            });
        }
        
        Ok(links)
    }

    async fn delete_link(&self, id: &Uuid) -> Result<bool, String> {
        let id_str = id.to_string();
        let result = sqlx::query("DELETE FROM memory_links WHERE id = ?")
            .bind(&id_str)
            .execute(&self.pool)
            .await
            .map_err(|e| format!("Failed to delete link: {}", e))?;
            
        Ok(result.rows_affected() > 0)
    }

    async fn stats(&self) -> Result<MemoryStats, String> {
        let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM memories")
            .fetch_one(&self.pool).await.unwrap_or(0);
            
        let explicit: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM memories WHERE pool = 'explicit'")
            .fetch_one(&self.pool).await.unwrap_or(0);
            
        let implicit: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM memories WHERE pool = 'implicit'")
            .fetch_one(&self.pool).await.unwrap_or(0);
            
        let links: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM memory_links")
            .fetch_one(&self.pool).await.unwrap_or(0);
            
        let avg_conf: f64 = sqlx::query_scalar("SELECT AVG(confidence) FROM memories")
            .fetch_one(&self.pool).await.unwrap_or(0.0);
            
        let avg_imp: f64 = sqlx::query_scalar("SELECT AVG(importance) FROM memories")
            .fetch_one(&self.pool).await.unwrap_or(0.0);

        let type_rows = sqlx::query("SELECT type, COUNT(*) as count FROM memories GROUP BY type")
            .fetch_all(&self.pool).await.unwrap_or_default();
            
        let mut memory_types = HashMap::new();
        for row in type_rows {
            let t: String = row.get("type");
            let c: i64 = row.get("count");
            memory_types.insert(t, c as usize);
        }

        Ok(MemoryStats {
            total_memories: total as usize,
            explicit_count: explicit as usize,
            implicit_count: implicit as usize,
            total_links: links as usize,
            memory_types,
            avg_confidence: avg_conf,
            avg_importance: avg_imp,
        })
    }
}