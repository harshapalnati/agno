use crate::memory::memory_trait::Memory;
use crate::model::model_trait::{Message, Model};
use crate::tool::tool_traits::Tool;

use serde::Deserialize;
use std::sync::Arc;

/// Agent handles tool invocation, memory storage, and LLM communication.
pub struct Agent {
    pub name: String,
    pub instructions: String,
    pub model: Box<dyn Model + Send + Sync>,
    pub tools: Vec<Box<dyn Tool + Send + Sync>>,
    pub memory: Arc<dyn Memory + Send + Sync>,
}

impl Agent {
    /// Create a new agent with model, tools, and memory
    pub fn new(
        name: String,
        user_instructions: String,
        model: Box<dyn Model + Send + Sync>,
        tools: Vec<Box<dyn Tool + Send + Sync>>,
        memory: Arc<dyn Memory + Send + Sync>,
    ) -> Self {
        let system_instructions = r#"
You are a helpful assistant.
If you want to use a tool, respond ONLY in JSON format:
{ "tool_call": { "name": "TOOL_NAME", "args": "ARGUMENT_STRING" } }
Otherwise, reply normally as a helpful assistant.
"#;

        let full_instructions = format!("{}\n{}", system_instructions.trim(), user_instructions);

        Self {
            name,
            instructions: full_instructions,
            model,
            tools,
            memory,
        }
    }

    /// Process a single user input: generate response, use tools, store in memory
    pub async fn run(&mut self, input: &str) {
        println!("\n🟦 Input: {input}");

        // Load memory history
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

        // Get model response
        let response = self.model.generate(messages).await;
        println!("🟨 Response: {}", response);

        // Store messages in memory
        let _ = self.memory.store("user", input).await;
        let _ = self.memory.store("assistant", &response).await;

        // Try to parse tool usage
        match Self::parse_tool_call(&response) {
            Some(tool_call) => {
                println!("🛠 Tool Call: {}({})", tool_call.name, tool_call.args);
                self.invoke_tool(tool_call).await;
            }
            None => println!("💬 Agent replied without tool usage."),
        }

        println!("✅ Agent finished.");
    }

    /// Process a single user input and return the assistant's response (with tool use if needed)
    pub async fn run_once(&mut self, input: &str) -> String {
        // Load memory history
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

        // Get model response
        let response = self.model.generate(messages).await;
        
        // Store messages in memory
        let _ = self.memory.store("user", input).await;
        let _ = self.memory.store("assistant", &response).await;
        
        // Try to parse tool usage
        match Self::parse_tool_call(&response) {
            Some(tool_call) => {
                // Actually invoke the tool and return its output
                let tool_output = self.invoke_tool_return(tool_call).await;
                tool_output
            }
            None => response,
        }
    }

    /// REPL loop for continuous interaction
    pub async fn run_loop(&mut self) {
        use std::io::{self, Write};

        println!("\n🤖 Agent '{}' is ready. Type input or 'exit' to quit.", self.name);

        loop {
            print!("> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                println!("❌ Error reading input.");
                continue;
            }

            let input = input.trim();
            match input {
                "exit" => {
                    println!("👋 Goodbye.");
                    break;
                }
                "/memory" => {
                    let history = self.memory.load().await;
                    println!("🧠 Memory:");
                    for msg in history {
                        println!("[{}] {}: {}", "timestamp", msg.role, msg.content);
                    }
                }
                "/clear" => {
                    if let Err(e) = self.memory.clear().await {
                        println!("❌ Could not clear memory: {}", e);
                    } else {
                        println!("🧹 Memory cleared.");
                    }
                }
                _ => {
                    self.run(input).await;
                }
            }
        }
    }

    /// Extract tool call from model response (if any)
    fn parse_tool_call(response: &str) -> Option<ToolCall> {
        serde_json::from_str::<ToolCallWrapper>(response)
            .ok()
            .map(|wrapper| wrapper.tool_call)
    }

    /// Invoke the appropriate tool with arguments and log output to memory
    async fn invoke_tool(&mut self, call: ToolCall) {
        match self.tools.iter().find(|t| t.name() == call.name) {
            Some(tool) => {
                let output = tool.call(&call.args).await;
                println!("🔧 Tool [{}]: {}", tool.name(), output);
                let _ = self
                    .memory
                    .store("tool", &format!("{} → {}", tool.name(), output))
                    .await;
            }
            None => {
                println!("⚠️ Tool '{}' not found.", call.name);
                let _ = self
                    .memory
                    .store("assistant", &format!("⚠️ Unknown tool: {}", call.name))
                    .await;
            }
        }
    }

    /// Invoke the appropriate tool and return its output as a string
    async fn invoke_tool_return(&mut self, call: ToolCall) -> String {
        match self.tools.iter().find(|t| t.name() == call.name) {
            Some(tool) => {
                let output = tool.call(&call.args).await;
                let _ = self.memory.store("tool", &format!("{} → {}", tool.name(), output)).await;
                output
            }
            None => {
                let msg = format!("⚠️ Unknown tool: {}", call.name);
                let _ = self.memory.store("assistant", &msg).await;
                msg
            }
        }
    }
}

/// Wrapper struct for parsing tool calls
#[derive(Debug, Deserialize)]
struct ToolCallWrapper {
    tool_call: ToolCall,
}

/// Struct representing a tool call
#[derive(Debug, Deserialize)]
struct ToolCall {
    name: String,
    args: String,
}
