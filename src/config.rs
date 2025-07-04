use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct AgentConfig {
    pub name: String,
    pub model: String,
    pub tools: Vec<String>,
    pub instructions: String,
}

pub fn load_agent_config(path: &str) -> Result<AgentConfig, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let config: AgentConfig = toml::from_str(&content)?;
    Ok(config)
}
