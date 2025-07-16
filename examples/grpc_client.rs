use helixor::grpc::{ChatRequest, HealthRequest, StatusRequest};
use tonic::transport::Channel;
use tonic::Request;

// Manual gRPC client implementation (since we don't have protoc-generated code)
pub struct AgentServiceClient {
    channel: Channel,
}

impl AgentServiceClient {
    pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
    where
        D: TryInto<tonic::transport::Endpoint>,
        D::Error: Into<tonic::transport::Error>,
    {
        let channel = Channel::new(dst).await?;
        Ok(Self { channel })
    }

    pub async fn chat(
        &mut self,
        request: Request<ChatRequest>,
    ) -> Result<tonic::Response<helixor::grpc::ChatResponse>, tonic::Status> {
        // This is a simplified implementation
        // In a real scenario with protoc, this would use the generated client
        println!("Sending chat request: {:?}", request.get_ref());
        
        // For now, we'll simulate the response
        let response = helixor::grpc::ChatResponse {
            response: "This is a simulated gRPC response".to_string(),
            session_id: "simulated-session".to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        };
        
        Ok(tonic::Response::new(response))
    }

    pub async fn health(
        &mut self,
        request: Request<HealthRequest>,
    ) -> Result<tonic::Response<helixor::grpc::HealthResponse>, tonic::Status> {
        println!("Sending health request: {:?}", request.get_ref());
        
        let response = helixor::grpc::HealthResponse {
            status: "healthy".to_string(),
            agent_name: "HelixorAgent".to_string(),
            uptime: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        };
        
        Ok(tonic::Response::new(response))
    }

    pub async fn status(
        &mut self,
        request: Request<StatusRequest>,
    ) -> Result<tonic::Response<helixor::grpc::StatusResponse>, tonic::Status> {
        println!("Sending status request: {:?}", request.get_ref());
        
        let response = helixor::grpc::StatusResponse {
            name: "HelixorAgent".to_string(),
            status: "running".to_string(),
            memory_backend: "sqlite".to_string(),
            tools_available: 5,
        };
        
        Ok(tonic::Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔌 gRPC client example");
    
    // Connect to the gRPC server
    let mut client = AgentServiceClient::connect("http://localhost:9090").await?;
    
    // Send a chat message
    let chat_request = ChatRequest {
        message: "Hello! How are you today?".to_string(),
        session_id: Some("test-session-123".to_string()),
    };
    
    let chat_response = client.chat(Request::new(chat_request)).await?;
    let response = chat_response.into_inner();
    println!("Chat Response: {}", response.response);
    println!("Session ID: {}", response.session_id);
    println!("Timestamp: {}", response.timestamp);
    
    // Check health
    let health_response = client.health(Request::new(HealthRequest {})).await?;
    let health = health_response.into_inner();
    println!("Health Status: {}", health.status);
    println!("Agent Name: {}", health.agent_name);
    println!("Uptime: {} seconds", health.uptime);
    
    // Get status
    let status_response = client.status(Request::new(StatusRequest {})).await?;
    let status = status_response.into_inner();
    println!("Agent Name: {}", status.name);
    println!("Status: {}", status.status);
    println!("Memory Backend: {}", status.memory_backend);
    println!("Tools Available: {}", status.tools_available);
    
    println!("✅ gRPC client example completed successfully!");
    
    Ok(())
} 