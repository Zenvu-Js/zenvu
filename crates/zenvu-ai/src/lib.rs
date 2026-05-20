//! # Zenvu AI
//!
//! Native AI capabilities for Zenvu.js server routes.
//! Provides unified interfaces for LLMs, streaming, embeddings, and RAG.

pub mod llm;
pub mod embeddings;
pub mod providers;
pub mod rag;


/// Configuration for AI providers.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AiConfig {
    pub openai_api_key: Option<String>,
    pub anthropic_api_key: Option<String>,
    pub gemini_api_key: Option<String>,
    pub ollama_url: Option<String>,
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            openai_api_key: std::env::var("OPENAI_API_KEY").ok(),
            anthropic_api_key: std::env::var("ANTHROPIC_API_KEY").ok(),
            gemini_api_key: std::env::var("GEMINI_API_KEY").ok(),
            ollama_url: std::env::var("OLLAMA_URL").ok().or_else(|| Some("http://localhost:11434".into())),
        }
    }
}
