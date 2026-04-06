//! Self-Evolving Memory Library
//!
//! Core memory management

pub mod memory;

pub mod api;

// Re-export main types
pub use memory::{Memory, MemoryType, MemoryPool, InMemoryStore, PostgresStore, MemoryStore};