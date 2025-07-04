use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Standard chat message format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,    // "system", "user", or "assistant"
    pub content: String,
}

/// Trait for all LLMs used in the agent
#[async_trait]
pub trait Model: Send + Sync {
    async fn generate(&self, messages: Vec<Message>) -> String;
}
