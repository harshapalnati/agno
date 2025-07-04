use crate::model::model_trait::Message;
use async_trait::async_trait;

#[async_trait]
pub trait Memory: Send + Sync {
    async fn recall(&self, key: &str) -> Option<String>;
    async fn store(&self, role: &str, value: &str);
    async fn load(&self) -> Vec<Message>;
    async fn clear(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}
