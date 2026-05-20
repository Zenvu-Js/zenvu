//! Text embeddings generation

use anyhow::Result;

pub struct EmbeddingOptions {
    pub model: String,
    pub dimensions: Option<usize>,
}

impl Default for EmbeddingOptions {
    fn default() -> Self {
        Self {
            model: "text-embedding-3-small".into(),
            dimensions: None,
        }
    }
}

#[async_trait::async_trait]
pub trait EmbeddingProvider: Send + Sync {
    async fn embed_text(&self, text: &str, options: &EmbeddingOptions) -> Result<Vec<f32>>;
    async fn embed_batch(&self, texts: &[&str], options: &EmbeddingOptions) -> Result<Vec<Vec<f32>>>;
}

pub struct MockEmbeddingProvider;

#[async_trait::async_trait]
impl EmbeddingProvider for MockEmbeddingProvider {
    async fn embed_text(&self, _text: &str, _options: &EmbeddingOptions) -> Result<Vec<f32>> {
        // Return a dummy 1536-dimensional vector
        Ok(vec![0.1; 1536])
    }

    async fn embed_batch(&self, texts: &[&str], options: &EmbeddingOptions) -> Result<Vec<Vec<f32>>> {
        let mut results = Vec::with_capacity(texts.len());
        for text in texts {
            results.push(self.embed_text(text, options).await?);
        }
        Ok(results)
    }
}
