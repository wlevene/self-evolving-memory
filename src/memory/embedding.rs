use serde::{Deserialize, Serialize};

/// Embedding provider trait
#[async_trait::async_trait]
pub trait EmbeddingProvider: Send + Sync {
    async fn embed(&self, text: &str) -> Result<Vec<f32>, String>;
    fn dimension(&self) -> usize;
}

/// OpenAI embedding provider
pub struct OpenAIEmbedding {
    api_key: String,
    model: String,
    client: reqwest::Client,
}

impl OpenAIEmbedding {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            model: "text-embedding-3-small".to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub fn with_model(mut self, model: &str) -> Self {
        self.model = model.to_string();
        self
    }
}

#[derive(Serialize)]
struct EmbeddingRequest {
    input: String,
    model: String,
}

#[derive(Deserialize)]
struct EmbeddingResponse {
    data: Vec<EmbeddingData>,
}

#[derive(Deserialize)]
struct EmbeddingData {
    embedding: Vec<f32>,
}

#[async_trait::async_trait]
impl EmbeddingProvider for OpenAIEmbedding {
    async fn embed(&self, text: &str) -> Result<Vec<f32>, String> {
        let request = EmbeddingRequest {
            input: text.to_string(),
            model: self.model.clone(),
        };

        let response = self.client
            .post("https://api.openai.com/v1/embeddings")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let embedding: EmbeddingResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        embedding
            .data
            .first()
            .map(|d| d.embedding.clone())
            .ok_or_else(|| "No embedding returned".to_string())
    }

    fn dimension(&self) -> usize {
        if self.model.contains("large") || self.model.contains("3-large") {
            3072
        } else if self.model.contains("3-small") {
            1536
        } else {
            1536
        }
    }
}

/// Local embedding provider (using a simple hash-based approach for demo)
pub struct LocalEmbedding {
    dimension: usize,
}

impl LocalEmbedding {
    pub fn new(dimension: usize) -> Self {
        Self { dimension }
    }
}

#[async_trait::async_trait]
impl EmbeddingProvider for LocalEmbedding {
    async fn embed(&self, text: &str) -> Result<Vec<f32>, String> {
        // Simple hash-based embedding (for demo/testing)
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        text.hash(&mut hasher);
        let hash = hasher.finish();
        
        // Generate pseudo-random embedding from hash
        let mut embedding = Vec::with_capacity(self.dimension);
        let mut state = hash;
        
        for _ in 0..self.dimension {
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let value = (state >> 32) as i32;
            embedding.push(value as f32 / i32::MAX as f32);
        }
        
        // Normalize
        let norm: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm > 0.0 {
            for x in &mut embedding {
                *x /= norm;
            }
        }
        
        Ok(embedding)
    }

    fn dimension(&self) -> usize {
        self.dimension
    }
}

/// Embedding service
pub struct EmbeddingService {
    provider: Box<dyn EmbeddingProvider>,
}

impl EmbeddingService {
    pub fn new(provider: Box<dyn EmbeddingProvider>) -> Self {
        Self { provider }
    }

    pub fn openai(api_key: String) -> Self {
        Self::new(Box::new(OpenAIEmbedding::new(api_key)))
    }

    pub fn local(dimension: usize) -> Self {
        Self::new(Box::new(LocalEmbedding::new(dimension)))
    }

    pub async fn embed(&self, text: &str) -> Result<Vec<f32>, String> {
        self.provider.embed(text).await
    }

    pub fn dimension(&self) -> usize {
        self.provider.dimension()
    }

    pub async fn embed_batch(&self, texts: &[&str]) -> Result<Vec<Vec<f32>>, String> {
        let mut embeddings = Vec::new();
        for text in texts {
            embeddings.push(self.embed(text).await?);
        }
        Ok(embeddings)
    }
}