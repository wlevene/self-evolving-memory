use serde_json::json;

/// MCP Tool definitions
pub fn get_tools() -> Vec<serde_json::Value> {
    vec![
        json!({
            "name": "memory_create",
            "description": "Create a new memory entry",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "content": { "type": "string", "description": "Memory content" },
                    "pool": { "type": "string", "enum": ["explicit", "implicit"], "default": "explicit" },
                    "type": { "type": "string", "enum": ["fact", "event", "procedure", "concept", "preference", "context"], "default": "fact" }
                },
                "required": ["content"]
            }
        }),
        json!({
            "name": "memory_retrieve",
            "description": "Search and retrieve memories",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "query": { "type": "string", "description": "Search query" },
                    "limit": { "type": "integer", "default": 10 }
                },
                "required": ["query"]
            }
        }),
        json!({
            "name": "memory_link",
            "description": "Create a link between two memories",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "source_id": { "type": "string", "description": "Source memory ID" },
                    "target_id": { "type": "string", "description": "Target memory ID" },
                    "link_type": { "type": "string", "enum": ["causes", "related", "contradicts", "specializes", "derived_from", "similar", "follows", "alternative"] }
                },
                "required": ["source_id", "target_id", "link_type"]
            }
        }),
        json!({
            "name": "memory_get",
            "description": "Get a specific memory by ID",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "id": { "type": "string", "description": "Memory UUID" }
                },
                "required": ["id"]
            }
        }),
        json!({
            "name": "memory_update",
            "description": "Update an existing memory",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "id": { "type": "string", "description": "Memory UUID" },
                    "content": { "type": "string" },
                    "confidence": { "type": "number" },
                    "importance": { "type": "number" }
                },
                "required": ["id"]
            }
        }),
        json!({
            "name": "memory_delete",
            "description": "Delete a memory",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "id": { "type": "string", "description": "Memory UUID" }
                },
                "required": ["id"]
            }
        }),
        json!({
            "name": "memory_stats",
            "description": "Get memory system statistics",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        }),
        json!({
            "name": "memory_consolidate",
            "description": "Run memory consolidation (decay + cleanup)",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        }),
    ]
}