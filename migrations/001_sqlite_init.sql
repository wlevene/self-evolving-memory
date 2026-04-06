-- Self-Evolving Memory SQLite Database Schema

-- Memories table
CREATE TABLE IF NOT EXISTS memories (
    id TEXT PRIMARY KEY,
    content TEXT NOT NULL,
    pool TEXT NOT NULL CHECK (pool IN ('explicit', 'implicit')),
    type TEXT NOT NULL CHECK (type IN ('fact', 'event', 'procedure', 'concept', 'preference', 'context')),
    confidence REAL NOT NULL DEFAULT 0.8,
    importance REAL NOT NULL DEFAULT 0.5,
    decay_rate REAL NOT NULL DEFAULT 0.01,
    source TEXT,
    tags TEXT DEFAULT '[]',
    metadata TEXT DEFAULT '{}',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_accessed DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    access_count INTEGER NOT NULL DEFAULT 0
);

-- Memory links table
CREATE TABLE IF NOT EXISTS memory_links (
    id TEXT PRIMARY KEY,
    source_id TEXT NOT NULL REFERENCES memories(id) ON DELETE CASCADE,
    target_id TEXT NOT NULL REFERENCES memories(id) ON DELETE CASCADE,
    link_type TEXT NOT NULL CHECK (link_type IN ('causes', 'related', 'contradicts', 'specializes', 'derived_from', 'similar', 'follows', 'alternative')),
    strength REAL NOT NULL DEFAULT 1.0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(source_id, target_id, link_type)
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_memories_pool ON memories(pool);
CREATE INDEX IF NOT EXISTS idx_memories_type ON memories(type);
CREATE INDEX IF NOT EXISTS idx_memories_created_at ON memories(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_memory_links_source ON memory_links(source_id);
CREATE INDEX IF NOT EXISTS idx_memory_links_target ON memory_links(target_id);

-- Update trigger for updated_at
CREATE TRIGGER IF NOT EXISTS memories_updated_at
AFTER UPDATE ON memories
FOR EACH ROW
BEGIN
    UPDATE memories SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;
