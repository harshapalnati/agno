use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::model::model_trait::{Message, Model};


/// Struct representing the OpenAI client
pub struct OpenAiClient {
    pub api_key: String,
    pub http: Client,
}

impl OpenAiClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            http: Client::new(),
        }
    }
}

/// Structure of a chat request sent to OpenAI
#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
}

/// Response from OpenAI containing choices
#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

/// Each choice contains a message
#[derive(Deserialize)]
struct Choice {
    message: Message,
}

/// Implementing the Model trait for the OpenAI client
#[async_trait]
impl Model for OpenAiClient {
    /// Generates a response by calling OpenAI with provided conversation messages
    async fn generate(&self, mut messages: Vec<Message>) -> String {
        // Inject system prompt at the beginning if not already present
        let system_prompt = Message {
            role: "system".to_string(),
            content: r#"You are an intelligent AI agent.
You may invoke tools when needed by responding with JSON like:
{"tool_call": {"name": "search", "args": "interest rate trends"}}
If a tool is not required, just answer normally."#
                .to_string(),
        };

        if messages.is_empty() || messages.first().unwrap().role != "system" {
            messages.insert(0, system_prompt);
        }

        // Build full chat request
        let request_body = ChatRequest {
            model: "gpt-3.5-turbo".to_string(),
            messages,
        };

        // Send request to OpenAI
        let response = self
            .http
            .post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(&self.api_key)
            .json(&request_body)
            .send()
            .await;

        // Handle response
        match response {
            Ok(resp) => match resp.json::<ChatResponse>().await {
                Ok(parsed) => {
                    parsed
                        .choices
                        .get(0)
                        .map(|c| c.message.content.clone())
                        .unwrap_or("⚠️ OpenAI returned no response.".to_string())
                }
                Err(err) => {
                    eprintln!("❌ Failed to parse OpenAI JSON response: {err}");
                    "❌ Failed to interpret OpenAI response.".to_string()
                }
            },
            Err(err) => {
                eprintln!("❌ HTTP request to OpenAI failed: {err}");
                "❌ Could not reach OpenAI.".to_string()
            }
        }
    }
}
