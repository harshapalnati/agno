use clap::{Parser, Subcommand};
use crate::deploy;
use crate::deploy::server;

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
    /// Run a team of agents using a configuration file
    Team {
        /// Path to the team config TOML file
        #[arg(short, long, default_value = "team.toml")]
        config: String,
    },
    /// Deploy an agent or team to Docker and get a URL
    Deploy {
        /// Path to the config file (agent.toml or team.toml)
        #[arg(short, long)]
        config: String,
        /// HTTP port to expose the agent on (default: 8080)
        #[arg(short, long, default_value = "8080")]
        port: u16,
        /// gRPC port to expose the agent on (optional)
        #[arg(long)]
        grpc_port: Option<u16>,
        /// Container name (default: auto-generated)
        #[arg(long)]
        name: Option<String>,
        /// Docker image tag (default: latest)
        #[arg(long, default_value = "latest")]
        tag: String,
    },
    /// Start HTTP/gRPC server for deployed agent
    Serve {
        /// HTTP port to serve on (default: 8080)
        #[arg(short, long, default_value = "8080")]
        port: u16,
        /// gRPC port to serve on (optional)
        #[arg(long)]
        grpc_port: Option<u16>,
        /// Path to agent config file
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
        Commands::Team { config } => {
            println!("🤝 Running team with config: {}", config);

            // TODO: Integrate actual team loading here
            // Placeholder only
        }
        Commands::Deploy { config, port, grpc_port, name, tag } => {
            println!("🚀 Deploying with config: {}", config);
            println!("🌐 HTTP port: {}", port);
            if let Some(grpc_port) = grpc_port {
                println!("🔌 gRPC port: {}", grpc_port);
            }
            
            // Deploy the agent/team
            if let Err(e) = deploy::deploy_agent(&config, port, name, tag).await {
                eprintln!("❌ Deployment failed: {}", e);
            }
        }
        Commands::Serve { port, grpc_port, config } => {
            println!("🌐 Starting server with config: {}", config);
            println!("🌐 HTTP port: {}", port);
            if let Some(grpc_port) = grpc_port {
                println!("🔌 gRPC port: {}", grpc_port);
            }
            
            // Start the HTTP/gRPC server
            if let Err(e) = server::start_server_from_config(&config, port, grpc_port).await {
                eprintln!("❌ Server failed to start: {}", e);
            }
        }
    }
}
