//! RAG (Retrieval-Augmented Generation) primitives

use crate::embeddings::EmbeddingProvider;
use anyhow::Result;
use std::sync::Arc;

pub struct Document {
    pub id: String,
    pub content: String,
    pub metadata: serde_json::Value,
    pub embedding: Option<Vec<f32>>,
}

/// A simple vector store trait
#[async_trait::async_trait]
pub trait VectorStore: Send + Sync {
    async fn add_documents(&self, docs: &[Document]) -> Result<()>;
    async fn similarity_search(&self, query_embedding: &[f32], limit: usize) -> Result<Vec<Document>>;
}

pub struct RagPipeline {
    embedding_provider: Arc<dyn EmbeddingProvider>,
    vector_store: Arc<dyn VectorStore>,
}

impl RagPipeline {
    pub fn new(embedding_provider: Arc<dyn EmbeddingProvider>, vector_store: Arc<dyn VectorStore>) -> Self {
        Self {
            embedding_provider,
            vector_store,
        }
    }

    /// Retrieve context for a given query
    pub async fn retrieve_context(&self, query: &str, limit: usize) -> Result<String> {
        let options = Default::default();
        let query_embedding = self.embedding_provider.embed_text(query, &options).await?;
        
        let docs = self.vector_store.similarity_search(&query_embedding, limit).await?;
        
        let mut context = String::new();
        for (i, doc) in docs.iter().enumerate() {
            context.push_str(&format!("Document {}:\n{}\n\n", i + 1, doc.content));
        }
        
        Ok(context)
    }
}
