use reqwest::Client;
use serde_json::{json, Value};
use std::collections::HashMap;

/// Client for communicating with deployed Helixor agents
pub struct HelixorClient {
    client: Client,
    agents: HashMap<String, String>, // name -> url
}

impl HelixorClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            agents: HashMap::new(),
        }
    }

    /// Add an agent to the client
    pub fn add_agent(&mut self, name: &str, url: &str) {
        self.agents.insert(name.to_string(), url.to_string());
    }

    /// Send a message to a specific agent
    pub async fn send_message(&self, agent_name: &str, message: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let url = self.agents.get(agent_name)
            .ok_or_else(|| format!("Agent '{}' not found", agent_name))?;

        let response = self.client
            .post(&format!("{}/chat", url))
            .json(&json!({
                "message": message,
                "session_id": None::<String>
            }))
            .send()
            .await?;

        let response_data: Value = response.json().await?;
        Ok(response_data["response"].as_str().unwrap_or("No response").to_string())
    }

    /// Send a message to all agents and collect responses
    pub async fn broadcast_message(&self, message: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error + Send + Sync>> {
        let mut responses = HashMap::new();
        
        for (name, url) in &self.agents {
            match self.send_message(name, message).await {
                Ok(response) => {
                    responses.insert(name.clone(), response);
                }
                Err(e) => {
                    eprintln!("Error communicating with agent {}: {}", name, e);
                }
            }
        }
        
        Ok(responses)
    }

    /// Check health of all agents
    pub async fn check_health(&self) -> Result<HashMap<String, bool>, Box<dyn std::error::Error + Send + Sync>> {
        let mut health_status = HashMap::new();
        
        for (name, url) in &self.agents {
            match self.client.get(&format!("{}/health", url)).send().await {
                Ok(response) => {
                    health_status.insert(name.clone(), response.status().is_success());
                }
                Err(_) => {
                    health_status.insert(name.clone(), false);
                }
            }
        }
        
        Ok(health_status)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client for multi-agent communication
    let mut client = HelixorClient::new();
    
    // Add deployed agents
    client.add_agent("researcher", "http://localhost:8080");
    client.add_agent("analyst", "http://localhost:8081");
    client.add_agent("writer", "http://localhost:8082");
    
    // Check health of all agents
    println!("🔍 Checking agent health...");
    let health = client.check_health().await?;
    for (name, is_healthy) in health {
        println!("{}: {}", name, if is_healthy { "✅" } else { "❌" });
    }
    
    // Send a message to a specific agent
    println!("\n💬 Sending message to researcher...");
    let response = client.send_message("researcher", "What is the latest news about AI?").await?;
    println!("Response: {}", response);
    
    // Broadcast a message to all agents
    println!("\n📢 Broadcasting message to all agents...");
    let responses = client.broadcast_message("Analyze the current market trends").await?;
    for (name, response) in responses {
        println!("{}: {}", name, response);
    }
    
    Ok(())
} 