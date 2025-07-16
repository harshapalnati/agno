use reqwest::Client;
use serde_json::json;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("🔌 Helixor Client Test");
    println!("Testing deployed agent via HTTP API");
    println!("");
    
    let client = Client::new();
    let base_url = "http://localhost:8080";
    
    // Test health endpoint
    println!("🏥 Testing health endpoint...");
    match client.get(&format!("{}/health", base_url))
        .timeout(Duration::from_secs(5))
        .send()
        .await {
        Ok(response) => {
            if response.status().is_success() {
                let health: serde_json::Value = response.json().await?;
                println!("✅ Health check passed!");
                println!("   Status: {}", health["status"]);
                println!("   Agent: {}", health["agent_name"]);
                println!("   Uptime: {} seconds", health["uptime"]);
            } else {
                println!("❌ Health check failed with status: {}", response.status());
            }
        }
        Err(e) => {
            println!("❌ Health check failed: {}", e);
            println!("   Make sure the agent is running on port 8080");
            return Ok(());
        }
    }
    
    println!("");
    
    // Test status endpoint
    println!("📊 Testing status endpoint...");
    match client.get(&format!("{}/status", base_url))
        .timeout(Duration::from_secs(5))
        .send()
        .await {
        Ok(response) => {
            if response.status().is_success() {
                let status: serde_json::Value = response.json().await?;
                println!("✅ Status check passed!");
                println!("   Name: {}", status["name"]);
                println!("   Status: {}", status["status"]);
                println!("   Memory: {}", status["memory_backend"]);
                println!("   Tools: {}", status["tools_available"]);
            } else {
                println!("❌ Status check failed with status: {}", response.status());
            }
        }
        Err(e) => {
            println!("❌ Status check failed: {}", e);
        }
    }
    
    println!("");
    
    // Test chat endpoint
    println!("💬 Testing chat endpoint...");
    let chat_request = json!({
        "message": "Hello! What tools do you have available? Can you help me with some calculations?",
        "session_id": "test-session-123"
    });
    
    match client.post(&format!("{}/chat", base_url))
        .header("Content-Type", "application/json")
        .json(&chat_request)
        .timeout(Duration::from_secs(30))
        .send()
        .await {
        Ok(response) => {
            if response.status().is_success() {
                let chat_response: serde_json::Value = response.json().await?;
                println!("✅ Chat request successful!");
                println!("   Session ID: {}", chat_response["session_id"]);
                println!("   Timestamp: {}", chat_response["timestamp"]);
                println!("   Response: {}", chat_response["response"]);
            } else {
                println!("❌ Chat request failed with status: {}", response.status());
                if let Ok(error_text) = response.text().await {
                    println!("   Error: {}", error_text);
                }
            }
        }
        Err(e) => {
            println!("❌ Chat request failed: {}", e);
        }
    }
    
    println!("");
    println!("🎉 Client test completed!");
    println!("");
    println!("💡 To run this test:");
    println!("   1. Start the agent: cargo run --example deploy_agent");
    println!("   2. In another terminal: cargo run --example client_test");
    
    Ok(())
} 