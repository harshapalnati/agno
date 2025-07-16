use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::agent::Agent;
use crate::agent::AgentBuilder;
use crate::model::openai::OpenAiClient;
use crate::memory::sqlite::SqliteMemory;
use crate::tool::ToolRegistry;
use crate::grpc;

/// Server state containing the agent
#[derive(Clone)]
pub struct AppState {
    pub agent: Arc<Mutex<Agent>>,
}

/// Request for agent interaction
#[derive(Deserialize)]
pub struct AgentRequest {
    pub message: String,
    pub session_id: Option<String>,
}

/// Response from agent
#[derive(Serialize)]
pub struct AgentResponse {
    pub response: String,
    pub session_id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Health check response
#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub agent_name: String,
    pub uptime: u64,
}

/// Start server from config file
pub async fn start_server_from_config(
    config_path: &str,
    http_port: u16,
    grpc_port: Option<u16>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Load agent from config
    let agent = Arc::new(Mutex::new(load_agent_from_config(config_path).await?));
    
    // Start both HTTP and gRPC servers
    if let Some(grpc_port) = grpc_port {
        let http_agent = agent.clone();
        let grpc_agent = agent.clone();
        
        let http_future = start_server(http_agent, http_port);
        let grpc_future = grpc::start_grpc_server(grpc_agent, grpc_port);
        
        tokio::try_join!(http_future, grpc_future)?;
    } else {
        // Start only HTTP server
        start_server(agent, http_port).await?;
    }
    
    Ok(())
}

/// Load agent from config file
async fn load_agent_from_config(config_path: &str) -> Result<Agent, Box<dyn std::error::Error + Send + Sync>> {
    // Load config
    let config_content = std::fs::read_to_string(config_path)?;
    let config: crate::config::AgentConfig = toml::from_str(&config_content)?;
    
    // Get API key from environment
    let api_key = std::env::var("OPENAI_API_KEY")
        .expect("OPENAI_API_KEY environment variable must be set");
    
    // Create tool registry
    let _registry = ToolRegistry::new();
    
    // Build agent
    let agent = AgentBuilder::new(&config.name)
        .with_instructions(&config.instructions)
        .with_model(Box::new(OpenAiClient::new(api_key)))
        .with_memory(Arc::new(SqliteMemory::new("memory.db")?))
        .with_tools(crate::tool::load_tools(&config.tools))
        .build();
    
    Ok(agent)
}

/// Create and start the HTTP server
pub async fn start_server(agent: Arc<Mutex<Agent>>, port: u16) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let state = AppState {
        agent,
    };

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/chat", post(chat_with_agent))
        .route("/status", get(get_status))
        .with_state(state);

    let addr = format!("0.0.0.0:{}", port);
    println!("🌐 Starting HTTP server on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Health check endpoint
async fn health_check(
    State(state): State<AppState>,
) -> Json<HealthResponse> {
    let agent = state.agent.lock().await;
    
    Json(HealthResponse {
        status: "healthy".to_string(),
        agent_name: agent.name.clone(),
        uptime: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
    })
}

/// Chat with the agent
async fn chat_with_agent(
    State(state): State<AppState>,
    Json(request): Json<AgentRequest>,
) -> Result<Json<AgentResponse>, StatusCode> {
    let mut agent = state.agent.lock().await;
    
    let response = agent.run_once(&request.message).await;
    let session_id = request.session_id.unwrap_or_else(|| {
        uuid::Uuid::new_v4().to_string()
    });

    Ok(Json(AgentResponse {
        response,
        session_id,
        timestamp: chrono::Utc::now(),
    }))
}

/// Get agent status
async fn get_status(
    State(state): State<AppState>,
) -> Json<serde_json::Value> {
    let agent = state.agent.lock().await;
    
    Json(serde_json::json!({
        "name": agent.name,
        "status": "running",
        "memory_backend": "sqlite",
        "tools_available": agent.tools.len()
    }))
} 

/// Start server for a programmatically constructed Agent
pub async fn start_agent_server(
    agent: Agent,
    http_port: u16,
    grpc_port: Option<u16>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let agent = Arc::new(Mutex::new(agent));
    if let Some(grpc_port) = grpc_port {
        let http_agent = agent.clone();
        let grpc_agent = agent.clone();
        let http_future = start_server(http_agent, http_port);
        let grpc_future = crate::grpc::start_grpc_server(grpc_agent, grpc_port);
        tokio::try_join!(http_future, grpc_future)?;
    } else {
        start_server(agent, http_port).await?;
    }
    Ok(())
}

// Placeholder for Team server (to be implemented as needed)
use crate::team::Team;

pub async fn start_team_server(
    _team: Team,
    _http_port: u16,
    _grpc_port: Option<u16>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // TODO: Implement team server logic
    println!("[helixor] Team server deployment is not yet implemented.");
    Ok(())
} 