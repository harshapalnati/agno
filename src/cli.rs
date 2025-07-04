use clap::{Parser, Subcommand};

/// CLI entrypoint for Helixor/AEGNO
#[derive(Parser)]
#[command(name = "helixor", version, about = "Run intelligent agents with memory and tools.")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run an agent using a configuration file
    Run {
        /// Path to the agent config TOML file
        #[arg(short, long, default_value = "agent.toml")]
        config: String,
    },
}

/// Main dispatcher function
pub async fn run_cli() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { config } => {
            println!("🧠 Running agent with config: {}", config);

            // TODO: Integrate actual agent loading here
            // For example:
            // let agent = load_agent(&config).await?;
            // agent.run_loop().await;

            // Placeholder only
        }
    }
}
