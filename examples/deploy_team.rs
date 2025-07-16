use helixor::{
    TeamBuilder, 
    TeamAgent, 
    TeamWorkflow,
    deploy_team_instance,
    HelixorResult
};

#[tokio::main]
async fn main() -> HelixorResult<()> {
    println!("🚀 Starting Helixor Team Deployment Example");
    
    println!("📋 Setting up team agents...");
    
    // Create specialized team agents
    let researcher = TeamAgent {
        name: "Researcher".to_string(),
        role: "Research Specialist".to_string(),
        instructions: "You are a research specialist. Your job is to gather information, facts, and data from reliable sources. Use search tools to find current information and provide well-researched insights.",
        tools: vec!["search".to_string()],
        model: "openai".to_string(),
    };

    let analyst = TeamAgent {
        name: "Analyst".to_string(),
        role: "Data Analyst".to_string(),
        instructions: "You are a data analyst. Your job is to analyze data, perform calculations, and provide insights. Use math tools for calculations and provide clear analysis of information.",
        tools: vec!["math".to_string(), "search".to_string()],
        model: "openai".to_string(),
    };

    let writer = TeamAgent {
        name: "Writer".to_string(),
        role: "Content Writer".to_string(),
        instructions: "You are a content writer. Your job is to write clear, engaging, and well-structured content based on research and analysis. Create summaries, reports, and explanations that are easy to understand.",
        tools: vec![],
        model: "openai".to_string(),
    };

    let reviewer = TeamAgent {
        name: "Reviewer".to_string(),
        role: "Quality Reviewer".to_string(),
        instructions: "You are a quality reviewer. Your job is to review content for accuracy, clarity, and completeness. Provide constructive feedback and ensure the final output meets high standards.",
        tools: vec!["search".to_string()],
        model: "openai".to_string(),
    };

    println!("🏗️  Building team with Chain-of-Thought workflow...");
    
    // Build the team using the builder pattern
    let team = TeamBuilder::new("ResearchTeam")
        .with_agent(researcher)
        .with_agent(analyst)
        .with_agent(writer)
        .with_agent(reviewer)
        .with_workflow(TeamWorkflow::ChainOfThought)
        .with_shared_memory("team_deployment_memory.db")
        .build();
    
    println!("✅ Team built successfully!");
    println!("👥 Team: {}", team.name);
    println!("👤 Agents: {}", team.agents.len());
    println!("🔄 Workflow: {:?}", team.workflow);
    println!("");
    println!("🌐 Deploying team with HTTP server on port 8081...");
    println!("📡 The team will be available at:");
    println!("   HTTP: http://localhost:8081");
    println!("");
    println!("🔗 You can test it with:");
    println!("   curl http://localhost:8081/health");
    println!("   curl -X POST http://localhost:8081/chat \\");
    println!("     -H 'Content-Type: application/json' \\");
    println!("     -d '{\"message\": \"Research the latest AI trends and write a summary\"}'");
    println!("");
    println!("⏹️  Press Ctrl+C to stop the server");
    println!("");
    
    // Deploy the team (this starts the HTTP server)
    // Note: Team gRPC is not yet implemented, so we pass None for grpc_port
    deploy_team_instance(team, 8081, None).await?;
    
    Ok(())
} 