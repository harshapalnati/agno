#!/bin/bash

# Example deployment script for Helixor agents

echo "🚀 Deploying Helixor agent..."

# Set your OpenAI API key
export OPENAI_API_KEY="your-openai-api-key-here"

# Deploy the agent
helixor deploy agent.toml --port 8080 --name my-agent

echo "✅ Agent deployed! You can now:"
echo "🌐 Access the agent at: http://localhost:8080"
echo "📊 Check health at: http://localhost:8080/health"
echo "💬 Chat with agent at: http://localhost:8080/chat"

# Example curl commands
echo ""
echo "📝 Example usage:"
echo "curl -X POST http://localhost:8080/chat \\"
echo "  -H 'Content-Type: application/json' \\"
echo "  -d '{\"message\": \"Hello, how are you?\"}'" 