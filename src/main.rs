mod agent;
mod cli;
mod config;
mod model;
mod tool;
mod memory;
mod team;
mod workflow;

use agent::Agent;
use clap::Parser;
use cli::{Cli, Commands};
use config::load_agent_config;
use memory::sqlite::SqliteMemory;
use model::openai::OpenAiClient;
use std::sync::Arc;
use tool::load_tools;
use team::{Team, TeamDispatcher};

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
        Commands::Team { config } => {
            println!("🤝 Starting team with config file: {config}");
            
            // Load team configuration
            let team_config = load_team_config(&config)?;
            println!("✅ Loaded Team Config:\n{:#?}", team_config);
            
            // Create team dispatcher
            let mut dispatcher = TeamDispatcher::new(team_config).await?;
            
            // Run team REPL
            run_team_repl(&mut dispatcher).await;
        }
    }

    Ok(())
}

/// Load team configuration from TOML file
fn load_team_config(path: &str) -> Result<Team, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    let team: Team = toml::from_str(&content)?;
    Ok(team)
}

/// Run team REPL loop
async fn run_team_repl(dispatcher: &mut TeamDispatcher) {
    use std::io::{self, Write};

    println!("\n🤝 Team '{}' is ready. Type input or 'exit' to quit.", "Team");
    println!("Commands: /status, /agents, /workflow, /clear");

    loop {
        print!("team> ");
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
            "/status" => {
                println!("📊 Team Status: Active");
            }
            "/agents" => {
                println!("👥 Team Agents: [List agents here]");
            }
            "/workflow" => {
                println!("🔄 Current Workflow: [Show workflow type]");
            }
            "/clear" => {
                println!("🧹 Team memory cleared.");
            }
            _ => {
                match dispatcher.execute(input).await {
                    Ok(result) => println!("✅ Result: {}", result),
                    Err(err) => println!("❌ Error: {}", err),
                }
            }
        }
    }
}
