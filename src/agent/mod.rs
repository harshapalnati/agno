use crate::model::{model_trait::Message, Model};
use crate::memory::memory_trait::Memory;
use crate::tool::tool_traits::Tool;
use serde::Deserialize;
use std::sync::Arc;

pub struct Agent {
    pub name: String,
    pub instructions: String,
    pub model: Box<dyn Model + Send + Sync>,
    pub tools: Vec<Box<dyn Tool + Send + Sync>>,
    pub memory: Arc<dyn Memory + Send + Sync>,
}

impl Agent {
    pub fn new(
        name: String,
        instructions: String,
        model: Box<dyn Model + Send + Sync>,
        tools: Vec<Box<dyn Tool + Send + Sync>>,
        memory: Arc<dyn Memory + Send + Sync>,
    ) -> Self {
        Self {
            name,
            instructions,
            model,
            tools,
            memory,
        }
    }

    pub async fn run(&self, input: &str) {
        println!("🤖 {} received input: {input}", self.name);

        // Load history from memory
        let mut messages = self.memory.load().await;

        // Add system instructions
        messages.insert(0, Message {
            role: "system".to_string(),
            content: self.instructions.clone(),
        });

        // Add user input to messages
        messages.push(Message {
            role: "user".to_string(),
            content: input.to_string(),
        });

        // Store user message
        self.memory.store("user", input).await;

        // Generate response from model
        let response = self.model.generate(messages.clone()).await;
        println!("🧠 Model says: {response}");

        // Store assistant response
        self.memory.store("assistant", &response).await;

        // Parse and invoke tool if applicable
        if let Some(tool_call) = Self::parse_tool_call(&response) {
            println!("🛠 Tool call detected: {}({})", tool_call.name, tool_call.args);
            self.invoke_tool(tool_call).await;
        } else {
            println!("💬 Normal assistant reply.");
        }

        println!("✅ Agent finished.");
    }

    pub async fn run_loop(&self) {
        use std::io::{self, Write};

        println!(
            "🤖 Agent '{}' is ready. Type your message or 'exit' to quit.",
            self.name
        );

        loop {
            print!("> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                println!("❌ Failed to read input.");
                continue;
            }

            let trimmed = input.trim();
            if trimmed.eq_ignore_ascii_case("exit") {
                println!("👋 Goodbye!");
                break;
            }

            if trimmed == "/memory" {
                let history = self.memory.load().await;
                println!("🧠 Memory:");
                for msg in history {
                    println!("[{}] {}", msg.role, msg.content);
                }
                continue;
            }

            if trimmed == "/clear" {
                match self.memory.clear().await {
                    Ok(_) => println!("🧹 Memory cleared."),
                    Err(e) => println!("❌ Could not clear memory: {}", e),
                }
                continue;
            }

            self.run(trimmed).await;
        }
    }

    async fn invoke_tool(&self, call: ToolCall) {
        let tool = self.tools.iter().find(|t| t.name() == call.name);

        match tool {
            Some(tool) => {
                let output = tool.call(&call.args).await;
                println!("🔧 Tool [{}] says: {output}", tool.name());

                self.memory
                    .store("tool", &format!("{} → {}", tool.name(), output))
                    .await;
            }
            None => {
                println!("⚠️ Tool '{}' not found.", call.name);
                self.memory
                    .store("assistant", &format!("⚠️ Unknown tool: {}", call.name))
                    .await;
            }
        }
    }

    fn parse_tool_call(response: &str) -> Option<ToolCall> {
        serde_json::from_str::<ToolCallWrapper>(response)
            .ok()
            .map(|wrapper| wrapper.tool_call)
    }
}

#[derive(Debug, Deserialize)]
struct ToolCallWrapper {
    tool_call: ToolCall,
}

#[derive(Debug, Deserialize)]
struct ToolCall {
    name: String,
    args: String,
}
