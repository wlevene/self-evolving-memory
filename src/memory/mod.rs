//! Memory management module
//!
//! Provides storage and retrieval for typed memories

mod store;
mod types;
mod postgres;

// Optional modules (disabled until dependencies are ready)
// mod embedding;
// mod vector;

// Re-export main types
pub use types::{Memory, MemoryType, MemoryPool, MemoryLink, LinkType, CreateMemoryRequest, UpdateMemoryRequest, SearchQuery, MemoryStats};
pub use store::{InMemoryStore, MemoryStore};
pub use postgres::PostgresStore;