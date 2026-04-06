use serde_json::json;

/// MCP Resource definitions
pub fn get_resources() -> Vec<serde_json::Value> {
    vec![
        json!({
            "uri": "memory://stats",
            "name": "Memory Statistics",
            "description": "Current memory system statistics",
            "mimeType": "application/json"
        }),
        json!({
            "uri": "memory://explicit",
            "name": "Explicit Memories",
            "description": "All explicit memories",
            "mimeType": "application/json"
        }),
        json!({
            "uri": "memory://implicit",
            "name": "Implicit Memories",
            "description": "All implicit memories",
            "mimeType": "application/json"
        }),
    ]
}