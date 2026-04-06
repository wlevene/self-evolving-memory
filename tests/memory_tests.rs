//! Basic tests for memory store functionality

use self_evolving_memory::memory::{InMemoryStore, Memory, MemoryPool, MemoryType, UpdateMemoryRequest, SearchQuery, MemoryLink, LinkType};
use self_evolving_memory::memory::MemoryStore;
use std::sync::Arc;

#[tokio::test]
async fn test_create_memory() {
    let store = Arc::new(InMemoryStore::new());
    let memory = Memory::new(
        "Test content".to_string(),
        MemoryPool::Explicit,
        MemoryType::Fact,
    );

    let id = memory.id;
    let created = store.create(memory.clone()).await.unwrap();
    assert_eq!(created.content, "Test content");

    let retrieved = store.get(&id).await.unwrap().unwrap();
    assert_eq!(retrieved.content, "Test content");
    assert_eq!(retrieved.pool, MemoryPool::Explicit);
    assert_eq!(retrieved.type_, MemoryType::Fact);
}

#[tokio::test]
async fn test_update_memory() {
    let store = Arc::new(InMemoryStore::new());
    let memory = Memory::new(
        "Original content".to_string(),
        MemoryPool::Explicit,
        MemoryType::Fact,
    );

    let id = memory.id;
    store.create(memory.clone()).await.unwrap();

    let update = UpdateMemoryRequest {
        content: Some("Updated content".to_string()),
        pool: Some("Implicit".to_string()),
        type_: Some("Concept".to_string()),
        confidence: None,
        importance: Some(0.9),
        decay_rate: None,
        tags: None,
        metadata: None,
    };

    store.update(&id, update).await.unwrap();

    let retrieved = store.get(&id).await.unwrap().unwrap();
    assert_eq!(retrieved.content, "Updated content");
    assert_eq!(retrieved.importance, 0.9);
}

#[tokio::test]
async fn test_delete_memory() {
    let store = Arc::new(InMemoryStore::new());
    let memory = Memory::new(
        "To be deleted".to_string(),
        MemoryPool::Explicit,
        MemoryType::Fact,
    );

    let id = memory.id;
    store.create(memory.clone()).await.unwrap();

    // Verify it exists
    assert!(store.get(&id).await.unwrap().is_some());

    // Delete it
    let deleted = store.delete(&id).await.unwrap();
    assert!(deleted);

    // Verify it's gone
    assert!(store.get(&id).await.unwrap().is_none());
}

#[tokio::test]
async fn test_list_memories() {
    let store = Arc::new(InMemoryStore::new());

    // Create multiple memories
    for i in 0..5 {
        let memory = Memory::new(
            format!("Memory {}", i),
            MemoryPool::Explicit,
            MemoryType::Fact,
        );
        store.create(memory).await.unwrap();
    }

    let all = store.list(None, None, 10).await.unwrap();
    assert_eq!(all.len(), 5);
}

#[tokio::test]
async fn test_search_by_content() {
    let store = Arc::new(InMemoryStore::new());

    // Create memories with different content
    let memory1 = Memory::new(
        "Rust programming language".to_string(),
        MemoryPool::Explicit,
        MemoryType::Fact,
    );
    let memory2 = Memory::new(
        "Python programming language".to_string(),
        MemoryPool::Explicit,
        MemoryType::Fact,
    );

    store.create(memory1).await.unwrap();
    store.create(memory2).await.unwrap();

    // Search for "Rust"
    let query = SearchQuery {
        query: "Rust".to_string(),
        pool: None,
        type_: None,
        tags: None,
        limit: 10,
        min_confidence: 0.0,
        include_links: false,
    };

    let results = store.search(query).await.unwrap();
    assert_eq!(results.len(), 1);
    assert!(results[0].content.contains("Rust"));
}

#[tokio::test]
async fn test_memory_links() {
    let store = Arc::new(InMemoryStore::new());

    let memory1 = Memory::new(
        "First memory".to_string(),
        MemoryPool::Explicit,
        MemoryType::Fact,
    );
    let memory2 = Memory::new(
        "Second memory".to_string(),
        MemoryPool::Explicit,
        MemoryType::Fact,
    );

    let id1 = memory1.id;
    let id2 = memory2.id;

    store.create(memory1).await.unwrap();
    store.create(memory2).await.unwrap();

    // Create link
    let link = MemoryLink::new(id1, id2, LinkType::Related);
    store.create_link(link).await.unwrap();

    // Get links
    let links = store.get_links(&id1).await.unwrap();
    assert_eq!(links.len(), 1);
    assert_eq!(links[0].target_id, id2);
}

#[tokio::test]
async fn test_access_count_increment() {
    let store = Arc::new(InMemoryStore::new());
    let memory = Memory::new(
        "Test access count".to_string(),
        MemoryPool::Explicit,
        MemoryType::Fact,
    );

    let id = memory.id;
    store.create(memory).await.unwrap();

    // Access multiple times
    for _ in 0..3 {
        store.get(&id).await.unwrap();
    }

    let retrieved = store.get(&id).await.unwrap().unwrap();
    assert_eq!(retrieved.access_count, 4); // 1 create + 3 get calls + 1 final get
}

#[test]
fn test_memory_pool_parsing() {
    use std::str::FromStr;
    assert!(MemoryPool::from_str("Explicit").is_ok());
    assert!(MemoryPool::from_str("Implicit").is_ok());
    assert!(MemoryPool::from_str("Invalid").is_err());
}

#[test]
fn test_memory_type_parsing() {
    use std::str::FromStr;
    assert!(MemoryType::from_str("Fact").is_ok());
    assert!(MemoryType::from_str("Concept").is_ok());
    assert!(MemoryType::from_str("Procedure").is_ok());
    assert!(MemoryType::from_str("Invalid").is_err());
}