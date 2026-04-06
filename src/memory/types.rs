use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Memory pool type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MemoryPool {
    Explicit,
    Implicit,
}

impl MemoryPool {
    pub fn as_str(&self) -> &'static str {
        match self {
            MemoryPool::Explicit => "explicit",
            MemoryPool::Implicit => "implicit",
        }
    }
}

impl std::fmt::Display for MemoryPool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for MemoryPool {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "explicit" => Ok(MemoryPool::Explicit),
            "implicit" => Ok(MemoryPool::Implicit),
            _ => Err(format!("Invalid pool: {}", s)),
        }
    }
}

/// Memory type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MemoryType {
    Fact,
    Event,
    Procedure,
    Concept,
    Preference,
    Context,
    Document,
    Image,
}

impl MemoryType {
    pub fn as_str(&self) -> &'static str {
        match self {
            MemoryType::Fact => "fact",
            MemoryType::Event => "event",
            MemoryType::Procedure => "procedure",
            MemoryType::Concept => "concept",
            MemoryType::Preference => "preference",
            MemoryType::Context => "context",
            MemoryType::Document => "document",
            MemoryType::Image => "image",
        }
    }
}

impl std::fmt::Display for MemoryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for MemoryType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "fact" => Ok(MemoryType::Fact),
            "event" => Ok(MemoryType::Event),
            "procedure" => Ok(MemoryType::Procedure),
            "concept" => Ok(MemoryType::Concept),
            "preference" => Ok(MemoryType::Preference),
            "context" => Ok(MemoryType::Context),
            "document" => Ok(MemoryType::Document),
            "image" => Ok(MemoryType::Image),
            _ => Err(format!("Invalid type: {}", s)),
        }
    }
}

/// Link type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LinkType {
    Causes,
    Related,
    Contradicts,
    Specializes,
    DerivedFrom,
    Similar,
    Follows,
    Alternative,
}

impl LinkType {
    pub fn as_str(&self) -> &'static str {
        match self {
            LinkType::Causes => "causes",
            LinkType::Related => "related",
            LinkType::Contradicts => "contradicts",
            LinkType::Specializes => "specializes",
            LinkType::DerivedFrom => "derived_from",
            LinkType::Similar => "similar",
            LinkType::Follows => "follows",
            LinkType::Alternative => "alternative",
        }
    }
}

impl std::fmt::Display for LinkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for LinkType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "causes" => Ok(LinkType::Causes),
            "related" => Ok(LinkType::Related),
            "contradicts" => Ok(LinkType::Contradicts),
            "specializes" => Ok(LinkType::Specializes),
            "derived_from" => Ok(LinkType::DerivedFrom),
            "similar" => Ok(LinkType::Similar),
            "follows" => Ok(LinkType::Follows),
            "alternative" => Ok(LinkType::Alternative),
            _ => Err(format!("Invalid link type: {}", s)),
        }
    }
}

/// Memory entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    pub id: Uuid,
    pub content: String,
    pub pool: MemoryPool,
    #[serde(rename = "type")]
    pub type_: MemoryType,
    pub confidence: f64,
    pub importance: f64,
    pub decay_rate: f64,
    pub source: Option<String>,
    pub tags: Vec<String>,
    pub metadata: HashMap<String, serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_accessed: DateTime<Utc>,
    pub access_count: u64,
}

impl Memory {
    pub fn new(content: String, pool: MemoryPool, type_: MemoryType) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            content,
            pool,
            type_,
            confidence: 0.8,
            importance: 0.5,
            decay_rate: 0.01,
            source: None,
            tags: Vec::new(),
            metadata: HashMap::new(),
            created_at: now,
            updated_at: now,
            last_accessed: now,
            access_count: 0,
        }
    }

    #[allow(dead_code)]
    pub fn current_strength(&self) -> f64 {
        self.importance * (1.0 + self.access_count as f64 * 0.1)
    }

    pub fn touch(&mut self) {
        self.last_accessed = Utc::now();
        self.access_count += 1;
    }
}

/// Memory link
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryLink {
    pub id: Uuid,
    pub source_id: Uuid,
    pub target_id: Uuid,
    pub link_type: LinkType,
    pub strength: f64,
    pub created_at: DateTime<Utc>,
}

impl MemoryLink {
    pub fn new(source_id: Uuid, target_id: Uuid, link_type: LinkType) -> Self {
        Self {
            id: Uuid::new_v4(),
            source_id,
            target_id,
            link_type,
            strength: 1.0,
            created_at: Utc::now(),
        }
    }
}

/// Create memory request
#[derive(Debug, Clone, Deserialize)]
pub struct CreateMemoryRequest {
    pub content: String,
    pub pool: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(default = "default_confidence")]
    pub confidence: f64,
    #[serde(default = "default_importance")]
    pub importance: f64,
    pub source: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
}

fn default_confidence() -> f64 { 0.8 }
fn default_importance() -> f64 { 0.5 }

impl CreateMemoryRequest {
    pub fn into_memory(self) -> Result<Memory, String> {
        let pool = self.pool
            .map(|p| p.parse())
            .transpose()?
            .unwrap_or(MemoryPool::Explicit);
        
        let type_ = self.type_
            .map(|t| t.parse())
            .transpose()?
            .unwrap_or(MemoryType::Fact);

        let mut memory = Memory::new(self.content, pool, type_);
        memory.confidence = self.confidence;
        memory.importance = self.importance;
        memory.source = self.source;
        memory.tags = self.tags;
        memory.metadata = self.metadata;

        Ok(memory)
    }
}

/// Update memory request
#[derive(Debug, Clone, Deserialize, Default)]
pub struct UpdateMemoryRequest {
    pub content: Option<String>,
    pub pool: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub confidence: Option<f64>,
    pub importance: Option<f64>,
    pub decay_rate: Option<f64>,
    pub tags: Option<Vec<String>>,
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

/// Search query
#[derive(Debug, Clone, Deserialize)]
pub struct SearchQuery {
    pub query: String,
    pub pool: Option<String>,
    #[serde(rename = "type")]
    #[allow(dead_code)]
    pub type_: Option<String>,
    #[allow(dead_code)]
    pub tags: Option<Vec<String>>,
    #[serde(default = "default_limit")]
    pub limit: usize,
    #[serde(default)]
    pub min_confidence: f64,
    #[allow(dead_code)]
    #[serde(default)]
    pub include_links: bool,
}

fn default_limit() -> usize { 10 }

/// Memory statistics
#[derive(Debug, Clone, Serialize)]
pub struct MemoryStats {
    pub total_memories: usize,
    pub explicit_count: usize,
    pub implicit_count: usize,
    pub total_links: usize,
    pub memory_types: HashMap<String, usize>,
    pub avg_confidence: f64,
    pub avg_importance: f64,
}