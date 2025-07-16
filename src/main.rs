mod agent;
mod cli;
mod config;
mod model;
mod tool;
mod memory;
mod team;
mod workflow;
mod deploy;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    cli::run_cli().await;
}
