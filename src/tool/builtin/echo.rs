use crate::tool::Tool;

pub struct EchoTool;

impl EchoTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl Tool for EchoTool {
    fn name(&self) -> &str {
        "echo"
    }

    async fn call(&self, input: &str) -> String {
        format!("Echo: {}", input)
    }
}
