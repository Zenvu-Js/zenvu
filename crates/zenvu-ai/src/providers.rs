//! AI Provider implementations (OpenAI, Anthropic, Gemini, Ollama)

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Role {
    System,
    User,
    Assistant,
    Tool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct GenerationOptions {
    pub model: String,
    pub temperature: f32,
    pub max_tokens: Option<u32>,
    pub top_p: Option<f32>,
    pub stream: bool,
}

impl Default for GenerationOptions {
    fn default() -> Self {
        Self {
            model: "gpt-4o".into(),
            temperature: 0.7,
            max_tokens: None,
            top_p: None,
            stream: false,
        }
    }
}

/// A unified trait for LLM providers
#[async_trait::async_trait]
pub trait LlmProvider: Send + Sync {
    /// Generate a complete text response
    async fn generate_text(&self, messages: &[Message], options: &GenerationOptions) -> anyhow::Result<String>;
    
    /// Generate a streaming text response (returns a stream of tokens)
    async fn stream_text(&self, messages: &[Message], options: &GenerationOptions) -> anyhow::Result<futures_util::stream::BoxStream<'static, anyhow::Result<String>>>;
}
