use crate::tool::tool_traits::Tool;
use async_trait::async_trait;
use meval;

pub struct MathTool;

impl MathTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Tool for MathTool {
    fn name(&self) -> &'static str {
        "math"
    }

    async fn call(&self, input: &str) -> String {
        match meval::eval_str(input) {
            Ok(result) => format!("🧮 Result: {}", result),
            Err(e) => format!("❌ Math error: {}", e),
        }
    }
}
