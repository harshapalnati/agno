use helixor::{
    AgentBuilder, 
    OpenAiClient, 
    SqliteMemory, 
    ToolRegistry,
    deploy_agent_instance,
    tool::builtin::{EchoTool, MathTool, SearchTool},
    HelixorResult
};

#[tokio::main]
async fn main() -> HelixorResult<()> {
    println!("🚀 Starting Helixor Agent Deployment Example");
    
    // Get API key from environment
    let api_key = std::env::var("OPENAI_API_KEY")
        .expect("Please set OPENAI_API_KEY environment variable");
    
    println!("📋 Setting up agent...");
    
    // Create tool registry with built-in tools
    let mut tools = ToolRegistry::new();
    tools.register(Box::new(EchoTool::new()));
    tools.register(Box::new(MathTool::new()));
    tools.register(Box::new(SearchTool::new()));
    
    // Build the agent using the builder pattern
    let agent = AgentBuilder::new("DeploymentAgent")
        .with_instructions(
            "You are a helpful AI assistant deployed via Helixor. \
             You have access to echo, math, and search tools. \
             Be friendly and helpful in your responses."
        )
        .with_model(Box::new(OpenAiClient::new(api_key)))
        .with_memory(SqliteMemory::new("deployment_agent_memory.db")?)
        .with_tools(tools)
        .build();
    
    println!("✅ Agent built successfully!");
    println!("🌐 Deploying agent with HTTP server on port 8080 and gRPC on port 9090...");
    println!("📡 The agent will be available at:");
    println!("   HTTP: http://localhost:8080");
    println!("   gRPC: localhost:9090");
    println!("");
    println!("🔗 You can test it with:");
    println!("   curl http://localhost:8080/health");
    println!("   curl -X POST http://localhost:8080/chat \\");
    println!("     -H 'Content-Type: application/json' \\");
    println!("     -d '{\"message\": \"Hello! What can you do?\"}'");
    println!("");
    println!("⏹️  Press Ctrl+C to stop the server");
    println!("");
    
    // Deploy the agent (this starts the HTTP/gRPC servers)
    deploy_agent_instance(agent, 8080, Some(9090)).await?;
    
    Ok(())
} 