# Helixor Examples

This directory contains examples demonstrating how to use Helixor programmatically without TOML files.

## 🚀 Quick Start

### Prerequisites

1. Set your OpenAI API key:
   ```bash
   export OPENAI_API_KEY="your-api-key-here"
   ```

2. Make sure you have the required dependencies installed.

## 📚 Examples

### 1. Deploy Agent (`deploy_agent.rs`)

Deploy a single agent with HTTP and gRPC servers.

```bash
cargo run --example deploy_agent
```

This will:
- Create an agent with echo, math, and search tools
- Start HTTP server on port 8080
- Start gRPC server on port 9090
- Display test commands you can use

### 2. Deploy Team (`deploy_team.rs`)

Deploy a team of agents with Chain-of-Thought workflow.

```bash
cargo run --example deploy_team
```

This will:
- Create a team with 4 specialized agents (Researcher, Analyst, Writer, Reviewer)
- Use Chain-of-Thought workflow
- Start HTTP server on port 8081
- Display test commands you can use

### 3. Client Test (`client_test.rs`)

Test the deployed agent via HTTP API.

```bash
# First, start the agent in one terminal:
cargo run --example deploy_agent

# Then, in another terminal, test it:
cargo run --example client_test
```

This will:
- Test health endpoint
- Test status endpoint  
- Test chat endpoint
- Show detailed responses

## 🔧 Testing the Deployed Agents

### HTTP API Testing

Once an agent is deployed, you can test it with curl:

```bash
# Health check
curl http://localhost:8080/health

# Get status
curl http://localhost:8080/status

# Send a message
curl -X POST http://localhost:8080/chat \
  -H 'Content-Type: application/json' \
  -d '{"message": "Hello! What can you do?", "session_id": "test-123"}'
```

### gRPC Testing (when protoc is installed)

```bash
# Use the gRPC client example
cargo run --example grpc_client
```

## 🎯 Key Features Demonstrated

- **Programmatic Agent Creation**: No TOML files needed
- **Builder Pattern**: Fluent API for constructing agents and teams
- **Tool Integration**: Built-in tools (echo, math, search)
- **HTTP/gRPC Servers**: Automatic server deployment
- **Team Workflows**: Multi-agent collaboration
- **Memory Persistence**: SQLite-based memory storage

## 🐛 Troubleshooting

### Common Issues

1. **"OPENAI_API_KEY not set"**
   - Set the environment variable: `export OPENAI_API_KEY="your-key"`

2. **"Port already in use"**
   - Change the port in the example or stop other services using that port

3. **"Connection refused"**
   - Make sure the agent is running before running the client test

4. **"protoc not found" warnings**
   - This is normal if protoc isn't installed. HTTP will still work.

### Getting Help

- Check the main README.md for more details
- Look at the source code in `src/` for implementation details
- The examples are self-contained and well-commented

## 🚀 Next Steps

After running these examples, you can:

1. **Modify the agents**: Change instructions, tools, or models
2. **Create custom tools**: Implement your own tools using the Tool trait
3. **Build teams**: Create more complex multi-agent workflows
4. **Deploy to production**: Use Docker or cloud platforms
5. **Add real gRPC**: Install protoc for full gRPC functionality 