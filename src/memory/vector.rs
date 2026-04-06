use crate::memory::types::*;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

impl super::PostgresStore {
    /// Find similar memories using vector similarity
    pub async fn find_similar(
        &self,
        embedding: &[f32],
        threshold: f32,
        limit: usize,
    ) -> Result<Vec<(Memory, f32)>, String> {
        let embedding_str = format!(
            "[{}]",
            embedding.iter()
                .map(|f| f.to_string())
                .collect::<Vec<_>>()
                .join(",")
        );

        let rows = sqlx::query_as::<_, (Uuid, String, f32)>(
            "SELECT id, content, similarity FROM find_similar_memories($1::vector, $2, $3)"
        )
        .bind(&embedding_str)
        .bind(threshold as f64)
        .bind(limit as i32)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to find similar memories: {}", e))?;

        let mut results = Vec::new();
        for (id, _content, similarity) in rows {
            if let Some(memory) = self.get(&id).await? {
                results.push((memory, similarity));
            }
        }

        Ok(results)
    }

    /// Update memory embedding
    pub async fn update_embedding(
        &self,
        id: &Uuid,
        embedding: &[f32],
    ) -> Result<(), String> {
        let embedding_str = format!(
            "[{}]",
            embedding.iter()
                .map(|f| f.to_string())
                .collect::<Vec<_>>()
                .join(",")
        );

        sqlx::query("SELECT update_memory_embedding($1, $2::vector)")
            .bind(id)
            .bind(&embedding_str)
            .execute(&self.pool)
            .await
            .map_err(|e| format!("Failed to update embedding: {}", e))?;

        Ok(())
    }

    /// Hybrid search: combine keyword and vector search
    pub async fn hybrid_search(
        &self,
        query: &str,
        embedding: Option<&[f32]>,
        limit: usize,
    ) -> Result<Vec<Memory>, String> {
        let pattern = format!("%{}%", query.to_lowercase());

        if let Some(emb) = embedding {
            // Combine keyword and vector search
            let embedding_str = format!(
                "[{}]",
                emb.iter()
                    .map(|f| f.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            );

            let rows = sqlx::query_as::<_, super::postgres::MemoryRow>(
                r#"
                SELECT m.* FROM memories m
                WHERE LOWER(m.content) LIKE $1
                ORDER BY m.embedding <=> $2::vector
                LIMIT $3
                "#
            )
            .bind(&pattern)
            .bind(&embedding_str)
            .bind(limit as i64)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| format!("Hybrid search failed: {}", e))?;

            Ok(rows.into_iter().map(|r| r.into()).collect())
        } else {
            // Keyword only
            self.search(SearchQuery {
                query: query.to_string(),
                pool: None,
                type_: None,
                tags: None,
                limit,
                min_confidence: 0.0,
                include_links: false,
            }).await
        }
    }
}