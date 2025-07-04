use async_trait::async_trait;

/// Trait that all tools must implement
#[async_trait]
pub trait Tool: Send + Sync {
    /// Unique tool name (used in tool_call JSON)
    fn name(&self) -> &str;

    /// The logic to execute the tool
    async fn call(&self, input: &str) -> String;
}
