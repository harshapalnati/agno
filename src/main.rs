mod agent;
mod cli;
mod config;
mod model;
mod tool;
mod memory;

use agent::Agent;
use clap::Parser;
use cli::{Cli, Commands};
use config::load_agent_config;
use memory::sqlite::SqliteMemory;
use model::openai::OpenAiClient;
use std::sync::Arc;
use tool::load_tools;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    if let Err(err) = run_cli().await {
        eprintln!("❌ Application error: {err}");
    }
}

/// Entrypoint for CLI-based agent execution
async fn run_cli() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { config } => {
            println!("🧠 Starting agent with config file: {config}");

            // Load and parse agent configuration from file
            let agent_config = load_agent_config(&config)?;
            println!("✅ Loaded Agent Config:\n{:#?}", agent_config);

            // Load tools dynamically via registry
            let tools = load_tools(&agent_config.tools);
            let tool_names: Vec<_> = tools.iter().map(|t| t.name()).collect();
            println!("🛠️ Active tools: {:?}", tool_names);

            // Get OpenAI API key from environment
            let api_key = std::env::var("OPENAI_API_KEY")
                .expect("❌ Missing OPENAI_API_KEY in .env or environment");

            // Initialize the LLM model client
            let model = Box::new(OpenAiClient::new(api_key));

            // Initialize persistent memory with SQLite (no .await here!)
            let memory = Arc::new(SqliteMemory::new("memory.db")?);

            // Create and run the agent REPL loop
            let mut agent = Agent::new(
                agent_config.name,
                agent_config.instructions,
                model,
                tools,
                memory,
            );

            agent.run_loop().await;
        }
    }

    Ok(())
}
