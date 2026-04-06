-- Add pgvector extension for semantic search
CREATE EXTENSION IF NOT EXISTS vector;

-- Add embedding column to memories table
ALTER TABLE memories ADD COLUMN IF NOT EXISTS embedding vector(1536);

-- Create vector index for similarity search
CREATE INDEX IF NOT EXISTS idx_memories_embedding ON memories USING ivfflat (embedding vector_cosine_ops) WITH (lists = 100);

-- Function to find similar memories
CREATE OR REPLACE FUNCTION find_similar_memories(
    query_embedding vector(1536),
    match_threshold float DEFAULT 0.7,
    match_count int DEFAULT 10
)
RETURNS TABLE (
    id uuid,
    content text,
    similarity float
)
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN QUERY
    SELECT 
        m.id,
        m.content,
        1 - (m.embedding <=> query_embedding) as similarity
    FROM memories m
    WHERE m.embedding IS NOT NULL
    AND 1 - (m.embedding <=> query_embedding) > match_threshold
    ORDER BY m.embedding <=> query_embedding
    LIMIT match_count;
END;
$$;

-- Function to update embedding
CREATE OR REPLACE FUNCTION update_memory_embedding(
    memory_id uuid,
    new_embedding vector(1536)
)
RETURNS void
LANGUAGE plpgsql
AS $$
BEGIN
    UPDATE memories SET embedding = new_embedding WHERE id = memory_id;
END;
$$;