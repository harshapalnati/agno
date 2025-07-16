use tonic::{Request, Response, Status};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::agent::Agent;
use serde::{Deserialize, Serialize};

// Manual gRPC types (fallback when protoc is not available)
#[derive(Debug, Clone, Deserialize)]
pub struct ChatRequest {
    pub message: String,
    pub session_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ChatResponse {
    pub response: String,
    pub session_id: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HealthRequest {}

#[derive(Debug, Clone, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub agent_name: String,
    pub uptime: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StatusRequest {}

#[derive(Debug, Clone, Serialize)]
pub struct StatusResponse {
    pub name: String,
    pub status: String,
    pub memory_backend: String,
    pub tools_available: u32,
}

// Service trait
#[tonic::async_trait]
pub trait AgentService {
    async fn chat(&self, request: Request<ChatRequest>) -> Result<Response<ChatResponse>, Status>;
    async fn health(&self, request: Request<HealthRequest>) -> Result<Response<HealthResponse>, Status>;
    async fn status(&self, request: Request<StatusRequest>) -> Result<Response<StatusResponse>, Status>;
}

/// Service implementation
#[derive(Clone)]
pub struct AgentServiceImpl {
    pub agent: Arc<Mutex<Agent>>,
}

impl AgentServiceImpl {
    pub fn new(agent: Arc<Mutex<Agent>>) -> Self {
        Self { agent }
    }
}

#[tonic::async_trait]
impl AgentService for AgentServiceImpl {
    async fn chat(
        &self,
        request: Request<ChatRequest>,
    ) -> Result<Response<ChatResponse>, Status> {
        let req = request.into_inner();
        let mut agent = self.agent.lock().await;
        
        let response = agent.run_once(&req.message).await;
        let session_id = req.session_id.unwrap_or_else(|| {
            uuid::Uuid::new_v4().to_string()
        });

        Ok(Response::new(ChatResponse {
            response,
            session_id,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }))
    }

    async fn health(
        &self,
        _request: Request<HealthRequest>,
    ) -> Result<Response<HealthResponse>, Status> {
        let agent = self.agent.lock().await;
        
        Ok(Response::new(HealthResponse {
            status: "healthy".to_string(),
            agent_name: agent.name.clone(),
            uptime: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }))
    }

    async fn status(
        &self,
        _request: Request<StatusRequest>,
    ) -> Result<Response<StatusResponse>, Status> {
        let agent = self.agent.lock().await;
        
        Ok(Response::new(StatusResponse {
            name: agent.name.clone(),
            status: "running".to_string(),
            memory_backend: "sqlite".to_string(),
            tools_available: agent.tools.len() as u32,
        }))
    }
}

/// Start gRPC server
pub async fn start_grpc_server(
    agent: Arc<Mutex<Agent>>,
    port: u16,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("🌐 Starting gRPC server on 0.0.0.0:{}", port);
    println!("⚠️ gRPC server requires protoc to be installed for full functionality.");
    println!("   For now, using HTTP endpoints. Install protoc with:");
    println!("   winget install Google.Protobuf");
    
    // For now, we'll just keep the server running but not actually serve gRPC
    // This allows the HTTP server to work while we wait for protoc installation
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await;
    }
} 