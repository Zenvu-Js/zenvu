//! Language Model (LLM) orchestration

use crate::providers::{GenerationOptions, LlmProvider, Message};
use anyhow::Result;
use std::sync::Arc;

/// The main entry point for LLM interactions.
pub struct LlmClient {
    provider: Arc<dyn LlmProvider>,
    default_options: GenerationOptions,
}

impl LlmClient {
    /// Create a new LLM client with a specific provider
    pub fn new(provider: Arc<dyn LlmProvider>) -> Self {
        Self {
            provider,
            default_options: GenerationOptions::default(),
        }
    }

    /// Set default generation options
    pub fn with_options(mut self, options: GenerationOptions) -> Self {
        self.default_options = options;
        self
    }

    /// Generate a complete response
    pub async fn chat(&self, messages: &[Message]) -> Result<String> {
        self.provider.generate_text(messages, &self.default_options).await
    }

    /// Generate a streaming response
    pub async fn stream_chat(&self, messages: &[Message]) -> Result<futures_util::stream::BoxStream<'static, Result<String>>> {
        let mut options = self.default_options.clone();
        options.stream = true;
        self.provider.stream_text(messages, &options).await
    }
}

// In a real implementation, we would include the actual API clients here:
// - OpenAiClient
// - AnthropicClient
// - GeminiClient
// - OllamaClient

/// Dummy mock provider for now
pub struct MockProvider;

#[async_trait::async_trait]
impl LlmProvider for MockProvider {
    async fn generate_text(&self, _messages: &[Message], _options: &GenerationOptions) -> Result<String> {
        Ok("This is a mock AI response from Zenvu.js".into())
    }

    async fn stream_text(&self, _messages: &[Message], _options: &GenerationOptions) -> Result<futures_util::stream::BoxStream<'static, Result<String>>> {
        let stream = futures_util::stream::iter(vec![
            Ok("This ".into()),
            Ok("is ".into()),
            Ok("a ".into()),
            Ok("mock ".into()),
            Ok("stream ".into()),
        ]);
        Ok(Box::pin(stream))
    }
}
