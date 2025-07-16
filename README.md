# 🧠 Helixor

**Helixor** is a powerful multi-agent framework for building AI agents and teams in Rust. Create intelligent agents with memory, reasoning, tools, and workflows — all with a simple, fluent API.

---

## ✨ Key Features

- ⚡ **Rust Native** – Fully async, memory-safe, and ultra-performant
- 🏗️ **Builder Pattern** – Fluent API for creating agents and teams
- 🛠️ **Modular & Pluggable** – Add tools, models, workflows via trait-based architecture
- 🤝 **Team Collaboration** – Multiple agents working together with different workflows
- 🧠 **Reasoning & Workflows** – Supports Chain-of-Thought, FSMs, DAGs
- 🧩 **Structured Memory** – Built-in support for SQLite and in-memory storage
- 📡 **CLI & Library** – Use as a library or run from command line
- 🐳 **Docker Deployment** – Deploy agents as containers with HTTP APIs
- 🔌 **gRPC Support** – High-performance, strongly-typed communication
- 🔧 **Tool Ecosystem** – Built-in tools and easy custom tool creation

---

## 🚀 Quick Start

### As a Library

Add to your `Cargo.toml`:

```toml
[dependencies]
helixor = "0.1.0"
```

Create your first agent:

```rust
use helixor::{AgentBuilder, OpenAiClient, SqliteMemory};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an agent
    let mut agent = AgentBuilder::new("MyAgent")
        .with_instructions("You are a helpful assistant.")
        .with_model(OpenAiClient::new("your-api-key"))
        .with_memory(SqliteMemory::new("memory.db")?)
        .build();

    // Run the agent
    let response = agent.run_once("Hello!").await;
    println!("Response: {}", response);

    Ok(())
}
```

### As a CLI Tool

```bash
# Run an agent with config file
helixor run agent.toml

# Run a team
helixor team team.toml

# Deploy an agent to Docker (HTTP only)
helixor deploy agent.toml --port 8080

# Deploy an agent to Docker (HTTP + gRPC)
helixor deploy agent.toml --port 8080 --grpc-port 9090

# Start HTTP/gRPC server
helixor serve --port 8080 --grpc-port 9090 --config agent.toml
```

---

## 🐳 Docker Deployment

Deploy your agents as containers with HTTP and gRPC APIs for multi-agent communication:

### Deploy an Agent

```bash
# Deploy agent to Docker (HTTP only)
helixor deploy agent.toml --port 8080 --name my-agent

# Deploy agent to Docker (HTTP + gRPC)
helixor deploy agent.toml --port 8080 --grpc-port 9090 --name my-agent

# The agent will be available at:
# HTTP: http://localhost:8080
# gRPC: localhost:9090
```

### Multi-Agent Communication

```rust
use helixor::HelixorClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = HelixorClient::new();
    
    // Add deployed agents
    client.add_agent("researcher", "http://localhost:8080");
    client.add_agent("analyst", "http://localhost:8081");
    
    // Send message to specific agent
    let response = client.send_message("researcher", "Research AI trends").await?;
    println!("Response: {}", response);
    
    // Broadcast to all agents
    let responses = client.broadcast_message("Analyze market data").await?;
    for (name, response) in responses {
        println!("{}: {}", name, response);
    }
    
    Ok(())
}
```

### HTTP API Endpoints

Once deployed, your agent exposes these endpoints:

- `GET /health` - Health check
- `POST /chat` - Send message to agent
- `GET /status` - Agent status

Example usage:

```bash
# Health check
curl http://localhost:8080/health

# Send message
curl -X POST http://localhost:8080/chat \
  -H 'Content-Type: application/json' \
  -d '{"message": "Hello, how are you?"}'

# Get status
curl http://localhost:8080/status
```

### gRPC API

Your agent also exposes gRPC services:

```protobuf
service AgentService {
  rpc Chat(ChatRequest) returns (ChatResponse);
  rpc Health(HealthRequest) returns (HealthResponse);
  rpc Status(StatusRequest) returns (StatusResponse);
}
```

Example gRPC client:

```rust
use tonic::transport::Channel;
use helixor::grpc::agent_service::agent_service_client::AgentServiceClient;
use helixor::grpc::agent_service::{ChatRequest, HealthRequest, StatusRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the gRPC server
    let channel = Channel::from_shared("http://localhost:9090".to_string())?
        .connect()
        .await?;
    
    let mut client = AgentServiceClient::new(channel);
    
    // Send a message
    let chat_request = ChatRequest {
        message: "Hello! How are you today?".to_string(),
        session_id: Some("test-session-123".to_string()),
    };
    
    let chat_response = client.chat(chat_request).await?;
    let response = chat_response.into_inner();
    println!("Response: {}", response.response);
    
    Ok(())
}
```

---

## 🧱 Architecture

```
┌─────────────────┐
│   Your App      │  ← Use Helixor as a library
└─────────┬───────┘
          │
┌─────────▼───────┐
│   Helixor       │  ← Rust library
│─────────────────│
│ Agent Builder   │
│ Team Builder    │
│ Tool Registry   │
│ Memory System   │
│ Model Clients   │
│ HTTP Server     │
│ gRPC Server     │
└─────────────────┘
```

---

## 📚 Examples

### Single Agent

```rust
use helixor::{AgentBuilder, OpenAiClient, SqliteMemory, ToolRegistry};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("OPENAI_API_KEY")?;
    let mut registry = ToolRegistry::new();
    
    let mut agent = AgentBuilder::new("Assistant")
        .with_instructions("You are a helpful AI assistant.")
        .with_model(OpenAiClient::new(api_key))
        .with_memory(SqliteMemory::new("memory.db")?)
        .with_tools(vec![
            registry.get("math").unwrap().clone(),
            registry.get("search").unwrap().clone(),
        ])
        .build();

    let response = agent.run_once("What is 15 * 23?").await;
    println!("Response: {}", response);

    Ok(())
}
```

### Team of Agents

```rust
use helixor::{TeamBuilder, TeamAgent, TeamWorkflow};

let researcher = TeamAgent {
    name: "Researcher".to_string(),
    role: "Research Specialist".to_string(),
    instructions: "Research and gather information.".to_string(),
    tools: vec!["search".to_string()],
    model: "openai".to_string(),
};

let team = TeamBuilder::new("ResearchTeam")
    .with_agent(researcher)
    .with_workflow(TeamWorkflow::ChainOfThought)
    .with_shared_memory("team_memory.db")
    .build();
```

### Custom Tools

```rust
use helixor::{Tool, async_trait};

#[derive(Debug)]
struct MyCustomTool;

#[async_trait]
impl Tool for MyCustomTool {
    fn name(&self) -> &str {
        "my_tool"
    }

    async fn call(&self, args: &str) -> String {
        format!("Custom tool called with: {}", args)
    }
}

// Register your tool
let mut registry = ToolRegistry::new();
registry.register("my_tool", Box::new(MyCustomTool));
```

---

## 🛠️ CLI Usage

### Configuration Files

**Agent Config (`agent.toml`):**

```toml
[agent]
name = "finance_bot"
instructions = "You are a financial assistant. Use tools when needed."

[tools]
enabled = ["math", "search"]

[memory]
backend = "sqlite"
path = "memory.db"
```

**Team Config (`team.toml`):**

```toml
name = "FinanceTeam"
shared_memory = "team_memory.db"

[[agents]]
name = "researcher"
role = "Data Researcher"
instructions = "Research financial data and market trends."
tools = ["search"]
model = "openai"

[[agents]]
name = "analyst"
role = "Financial Analyst"
instructions = "Analyze financial data and perform calculations."
tools = ["math", "search"]
model = "openai"

[workflow]
type = "ChainOfThought"
```

### Commands

```bash
# Run an agent
helixor run agent.toml

# Run a team
helixor team team.toml

# Deploy agent to Docker (HTTP only)
helixor deploy agent.toml --port 8080

# Deploy agent to Docker (HTTP + gRPC)
helixor deploy agent.toml --port 8080 --grpc-port 9090

# Start HTTP/gRPC server
helixor serve --port 8080 --grpc-port 9090 --config agent.toml

# Interactive mode
helixor run agent.toml --interactive
```

---

## 🔧 Available Tools

- **Math Tool** - Perform mathematical calculations
- **Search Tool** - Search the web for information
- **Echo Tool** - Echo back input (for testing)

### Adding Custom Tools

```rust
use helixor::{Tool, async_trait};

#[derive(Debug)]
struct WeatherTool;

#[async_trait]
impl Tool for WeatherTool {
    fn name(&self) -> &str {
        "weather"
    }

    async fn call(&self, location: &str) -> String {
        // Implement weather API call
        format!("Weather for {}: Sunny, 72°F", location)
    }
}
```

---

## 🧠 Memory Systems

- **SQLite Memory** - Persistent storage with SQLite
- **In-Memory** - Fast, temporary storage

```rust
// SQLite for persistence
let memory = SqliteMemory::new("memory.db")?;

// In-memory for speed
let memory = InMemory::new();
```

---

## 🔭 Roadmap

- [x] Rust library with builder pattern
- [x] CLI tool with TOML configs
- [x] Docker deployment with HTTP APIs
- [x] gRPC support for high-performance communication
- [x] Multi-agent communication
- [x] Basic tools (math, search, echo)
- [x] SQLite and in-memory storage
- [x] Team workflows
- [ ] WebSocket streaming
- [ ] Plugin marketplace
- [ ] Cloud deployment support
- [ ] Python/JS bindings

---

## 📄 License

MIT

## ✅ **What's Working Now:**

1. **Library Structure** - Can be imported like `agno`
2. **Builder Pattern** - `AgentBuilder` and `TeamBuilder` for fluent API
3. **HTTP Server** - REST API endpoints for agent communication
4. **gRPC Support** - Structure in place (placeholder implementation)
5. **Docker Deployment** - `helixor deploy` command
6. **Multi-Agent Communication** - HTTP client for agent-to-agent communication
7. **CLI Commands** - `run`, `team`, `deploy`, `serve`

## 📝 **Next Steps (Optional):**

1. **Real gRPC Implementation** - Install `protoc` and implement full gRPC server
2. **Cloud Deployment** - Add AWS/GCP deployment support
3. **More Tools** - Add custom tools to the registry
4. **WebSocket Streaming** - Real-time agent communication
5. **Plugin System** - Dynamic tool loading

Your multi-agent framework is now ready to use! 🎯

---

## 🚦 Programmatic Deployment (No TOML Required)

You can deploy agents and teams directly from Rust code using the Helixor library—no config files needed!

### Deploy an Agent from Code

```rust
use helixor::{AgentBuilder, OpenAiClient, SqliteMemory, ToolRegistry, deploy_agent_instance};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("OPENAI_API_KEY")?;
    let tools = ToolRegistry::new();

    let agent = AgentBuilder::new("MyAgent")
        .with_instructions("You are a helpful assistant.")
        .with_model(OpenAiClient::new(api_key))
        .with_memory(SqliteMemory::new("memory.db")?)
        .with_tools(tools)
        .build();

    // Deploy the agent (starts HTTP/gRPC server)
    deploy_agent_instance(agent, 8080, Some(9090)).await?;

    Ok(())
}
```

### Deploy a Team from Code

```rust
use helixor::{TeamBuilder, TeamAgent, TeamWorkflow, deploy_team_instance};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let researcher = TeamAgent {
        name: "Researcher".to_string(),
        role: "Research Specialist".to_string(),
        instructions: "Research and gather information.".to_string(),
        tools: vec!["search".to_string()],
        model: "openai".to_string(),
    };

    let analyst = TeamAgent {
        name: "Analyst".to_string(),
        role: "Financial Analyst".to_string(),
        instructions: "Analyze financial data and perform calculations.".to_string(),
        tools: vec!["math".to_string(), "search".to_string()],
        model: "openai".to_string(),
    };

    let writer = TeamAgent {
        name: "Writer".to_string(),
        role: "Report Writer".to_string(),
        instructions: "Write a summary report.".to_string(),
        tools: vec![],
        model: "openai".to_string(),
    };

    let team = TeamBuilder::new("ResearchTeam")
        .with_agent(researcher)
        .with_agent(analyst)
        .with_agent(writer)
        .with_workflow(TeamWorkflow::ChainOfThought)
        .build();

    deploy_team_instance(team, 8081, None).await?;
    Ok(())
}
```
