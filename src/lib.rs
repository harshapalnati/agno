//! Helixor - A powerful multi-agent framework for building AI agents and teams
//! 
//! # Quick Start
//! 
//! ```rust
//! use helixor::{Agent, AgentBuilder, OpenAiClient, SqliteMemory};
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create an agent
//!     let mut agent = AgentBuilder::new("MyAgent")
//!         .with_instructions("You are a helpful assistant.")
//!         .with_model(OpenAiClient::new("your-api-key"))
//!         .with_memory(SqliteMemory::new("memory.db")?)
//!         .build();
//! 
//!     // Run the agent
//!     let response = agent.run_once("Hello!").await;
//!     println!("Response: {}", response);
//! 
//!     Ok(())
//! }
//! ```

pub mod agent;
pub mod config;
pub mod deploy;
pub mod grpc;
pub mod memory;
pub mod model;
pub mod team;
pub mod tool;
pub mod workflow;

// Re-export main types for easy importing
pub use agent::{Agent, AgentBuilder};
pub use memory::{Memory, SqliteMemory, InMemory};
pub use model::{Model, OpenAiClient};
pub use team::{Team, TeamBuilder, TeamAgent, TeamWorkflow, FSMConfig, DAGConfig};
pub use tool::{ToolRegistry};
pub use tool::tool_traits::Tool;
pub use deploy::{deploy_agent_instance, deploy_team_instance};

// Re-export common traits
pub use async_trait::async_trait;

/// Result type for Helixor operations
pub type HelixorResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>; 