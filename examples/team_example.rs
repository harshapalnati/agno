use helixor::{TeamBuilder, TeamAgent, TeamWorkflow, OpenAiClient, SqliteMemory};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up your OpenAI API key
    let api_key = std::env::var("OPENAI_API_KEY")
        .expect("Please set OPENAI_API_KEY environment variable");

    // Create team agents
    let researcher = TeamAgent {
        name: "Researcher".to_string(),
        role: "Research Specialist".to_string(),
        instructions: "You are a research specialist. Your job is to gather information and facts.".to_string(),
        tools: vec!["search".to_string()],
        model: "openai".to_string(),
    };

    let analyst = TeamAgent {
        name: "Analyst".to_string(),
        role: "Data Analyst".to_string(),
        instructions: "You are a data analyst. Your job is to analyze data and provide insights.".to_string(),
        tools: vec!["math".to_string()],
        model: "openai".to_string(),
    };

    let writer = TeamAgent {
        name: "Writer".to_string(),
        role: "Content Writer".to_string(),
        instructions: "You are a content writer. Your job is to write clear, engaging content based on research and analysis.".to_string(),
        tools: vec![],
        model: "openai".to_string(),
    };

    // Create a team using the builder pattern
    let team = TeamBuilder::new("ResearchTeam")
        .with_agent(researcher)
        .with_agent(analyst)
        .with_agent(writer)
        .with_workflow(TeamWorkflow::ChainOfThought)
        .with_shared_memory("team_memory.db")
        .build();

    println!("🤝 Team created successfully!");
    println!("📋 Team: {}", team.name);
    println!("👥 Agents: {}", team.agents.len());
    println!("🔄 Workflow: {:?}", team.workflow);

    // You can now use this team with a dispatcher
    // (The dispatcher implementation would need to be adapted for the library)

    Ok(())
} 