use crate::memory::store::InMemoryStore;
use crate::memory::types::*;
use std::sync::Arc;
use serde_json::Value;

/// MCP Server for AI agent integration
pub struct McpServer {
    store: Arc<InMemoryStore>,
}

impl McpServer {
    pub fn new(store: Arc<InMemoryStore>) -> Self {
        Self { store }
    }

    /// Handle MCP tool call
    pub async fn handle_tool_call(&self, tool_name: &str, arguments: Value) -> Result<Value, String> {
        match tool_name {
            "memory_create" => self.memory_create(arguments).await,
            "memory_retrieve" => self.memory_retrieve(arguments).await,
            "memory_link" => self.memory_link(arguments).await,
            "memory_get" => self.memory_get(arguments).await,
            "memory_update" => self.memory_update(arguments).await,
            "memory_delete" => self.memory_delete(arguments).await,
            "memory_stats" => self.memory_stats(arguments).await,
            "memory_consolidate" => self.memory_consolidate(arguments).await,
            _ => Err(format!("Unknown tool: {}", tool_name)),
        }
    }

    async fn memory_create(&self, args: Value) -> Result<Value, String> {
        let content = args["content"].as_str().ok_or("Missing content")?.to_string();
        let pool_str = args["pool"].as_str().unwrap_or("explicit");
        let type_str = args["type"].as_str().unwrap_or("fact");
        
        let pool: MemoryPool = pool_str.parse()?;
        let type_: MemoryType = type_str.parse()?;
        
        let memory = Memory::new(content, pool, type_);
        let created = self.store.create(memory).await?;
        
        Ok(serde_json::to_value(created).map_err(|e| e.to_string())?)
    }

    async fn memory_retrieve(&self, args: Value) -> Result<Value, String> {
        let query = args["query"].as_str().ok_or("Missing query")?.to_string();
        let limit = args["limit"].as_u64().unwrap_or(10) as usize;
        let type_ = args["type"].as_str().map(|s| s.to_string());
        let pool = args["pool"].as_str().map(|s| s.to_string());
        
        let search_query = SearchQuery {
            query,
            pool,
            type_,
            tags: None,
            limit,
            min_confidence: 0.0,
            include_links: false,
        };
        
        let results = self.store.search(search_query).await?;
        Ok(serde_json::to_value(results).map_err(|e| e.to_string())?)
    }

    async fn memory_link(&self, args: Value) -> Result<Value, String> {
        let source_id = args["source_id"].as_str().ok_or("Missing source_id")?;
        let target_id = args["target_id"].as_str().ok_or("Missing target_id")?;
        let link_type_str = args["link_type"].as_str().ok_or("Missing link_type")?;
        
        let source = uuid::Uuid::parse_str(source_id).map_err(|e| e.to_string())?;
        let target = uuid::Uuid::parse_str(target_id).map_err(|e| e.to_string())?;
        let link_type: LinkType = link_type_str.parse()?;
        
        let link = MemoryLink::new(source, target, link_type);
        let created = self.store.create_link(link).await?;
        
        Ok(serde_json::to_value(created).map_err(|e| e.to_string())?)
    }

    async fn memory_get(&self, args: Value) -> Result<Value, String> {
        let id = args["id"].as_str().ok_or("Missing id")?;
        let uuid = uuid::Uuid::parse_str(id).map_err(|e| e.to_string())?;
        
        match self.store.get(&uuid).await? {
            Some(memory) => Ok(serde_json::to_value(memory).map_err(|e| e.to_string())?),
            None => Err("Memory not found".to_string()),
        }
    }

    async fn memory_update(&self, args: Value) -> Result<Value, String> {
        let id = args["id"].as_str().ok_or("Missing id")?;
        let uuid = uuid::Uuid::parse_str(id).map_err(|e| e.to_string())?;
        
        let update = UpdateMemoryRequest {
            content: args["content"].as_str().map(|s| s.to_string()),
            pool: args["pool"].as_str().map(|s| s.to_string()),
            type_: args["type"].as_str().map(|s| s.to_string()),
            confidence: args["confidence"].as_f64(),
            importance: args["importance"].as_f64(),
            decay_rate: args["decay_rate"].as_f64(),
            tags: args["tags"].as_array().map(|a| a.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect()),
            metadata: None,
        };
        
        match self.store.update(&uuid, update).await? {
            Some(memory) => Ok(serde_json::to_value(memory).map_err(|e| e.to_string())?),
            None => Err("Memory not found".to_string()),
        }
    }

    async fn memory_delete(&self, args: Value) -> Result<Value, String> {
        let id = args["id"].as_str().ok_or("Missing id")?;
        let uuid = uuid::Uuid::parse_str(id).map_err(|e| e.to_string())?;
        
        match self.store.delete(&uuid).await? {
            true => Ok(serde_json::json!({ "deleted": true })),
            false => Err("Memory not found".to_string()),
        }
    }

    async fn memory_stats(&self, _args: Value) -> Result<Value, String> {
        let stats = self.store.stats().await?;
        Ok(serde_json::to_value(stats).map_err(|e| e.to_string())?)
    }

    async fn memory_consolidate(&self, _args: Value) -> Result<Value, String> {
        // TODO: Implement memory consolidation (decay + cleanup)
        Ok(serde_json::json!({ "consolidated": true, "removed": 0 }))
    }
}