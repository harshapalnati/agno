use helixor::{AgentBuilder, OpenAiClient, SqliteMemory, ToolRegistry};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up your OpenAI API key
    let api_key = std::env::var("OPENAI_API_KEY")
        .expect("Please set OPENAI_API_KEY environment variable");

    // Create a tool registry
    let mut registry = ToolRegistry::new();
    
    // Create an agent using the builder pattern
    let mut agent = AgentBuilder::new("MyAssistant")
        .with_instructions("You are a helpful AI assistant. You can use tools when needed.")
        .with_model(OpenAiClient::new(api_key))
        .with_memory(SqliteMemory::new("example_memory.db")?)
        .with_tools(vec![
            registry.get("math").unwrap().clone(),
            registry.get("search").unwrap().clone(),
        ])
        .build();

    // Run the agent with a simple query
    println!("🤖 Agent created successfully!");
    println!("💬 Asking: 'What is 15 * 23?'");
    
    let response = agent.run_once("What is 15 * 23?").await;
    println!("📝 Response: {}", response);

    // Ask another question
    println!("\n💬 Asking: 'What is the current weather in New York?'");
    let response = agent.run_once("What is the current weather in New York?").await;
    println!("📝 Response: {}", response);

    Ok(())
} 