# 🦾 AEGNO

**AEGNO** is a blazing-fast, developer-friendly, and language-agnostic framework for building multi-agent systems. Built entirely in **Rust**, AEGNO allows developers to create intelligent agents with memory, reasoning, tools, and workflows — accessible from **Java, Rust**, and (soon) **Python, JS, Go**, and more.

---

## ✨ Key Features

- ⚡ **Rust Core** – Fully async, memory-safe, and ultra-performant
- ⚙️ **Modular & Pluggable** – Add tools, models, workflows via trait-based architecture
- 🌍 **Language Agnostic** – Exposes agents via gRPC/HTTP for cross-language use
- 🛠️ **CLI-First Developer Experience** – `helixor run agent.toml` in seconds
- 🤝 **Team Collaboration** – Multiple agents working together with different workflows
- 🧠 **Reasoning & Workflows** – Supports Chain-of-Thought, FSMs, DAGs
- 🧩 **Structured Memory** – Built-in support for SQLite, Redis, Qdrant
- 📡 **API Ready** – Axum-powered REST & gRPC endpoints
- ☕ **Java SDK Available** – Simple Java client to run agents

---

## 🧱 Architecture

```
           +----------------+
           |    Java App    |
           +--------+-------+
                    |
             (HTTP / gRPC)
                    ↓
           +--------+--------+
           |   AEGNO Server   |  ← Rust core
           |------------------|
           | Agent Runtime    |
           | Tool Engine      |
           | Reasoning Engine |
           | Memory / RAG     |
           | Workflow Engine  |
           +------------------+
```

---

## 🗂 File Structure (Rust Core)

```
aegno/
├── Cargo.toml
└── src/
    ├── agent/        # Agent struct and execution
    ├── tool/         # Tool trait and plugins
    ├── model/        # LLM support
    ├── memory/       # SQLite, Redis, Vector DB
    ├── workflow/     # CoT, FSM, DAG planner
    ├── server/       # Axum REST/gRPC APIs
    ├── config/       # TOML/YAML loaders
    ├── cli/          # CLI commands
    └── tracing/      # Observability
```

---

## ⚙️ Examples

### Single Agent: `agent.toml`

```toml
[agent]
name = "finance_bot"
model = "openai:gpt-4-turbo"
instructions = "Use markdown tables. Be concise."

[tools.yfinance]
enabled = true

[memory]
backend = "sqlite"
path = "data/memory.db"
```

### Team of Agents: `team.toml`

```toml
name = "FinanceTeam"
shared_memory = "team_memory.db"

[[agents]]
name = "researcher"
role = "Data Researcher"
instructions = "Research financial data and market trends."
tools = ["search"]
model = "openai:gpt-4-turbo"

[[agents]]
name = "analyst"
role = "Financial Analyst"
instructions = "Analyze financial data and perform calculations."
tools = ["math", "search"]
model = "openai:gpt-4-turbo"

[[agents]]
name = "reporter"
role = "Report Writer"
instructions = "Write clear, concise reports with actionable insights."
tools = ["search"]
model = "openai:gpt-4-turbo"

[workflow]
type = "ChainOfThought"
```

---

## 🛠 Usage

### 🚀 Run an Agent

```bash
helixor run agent.toml
```

### 🤝 Run a Team

```bash
helixor team team.toml
```

### 🧰 Create a New Agent

```bash
helixor new summarizer
```

### 🧠 Add a Tool

```bash
helixor add-tool yfinance
```

### 📡 Serve as API

```bash
helixor serve
```

---

## ☕ Java SDK (Preview)

```java
AgentClient client = new AgentClient("http://localhost:8080");
String result = client.ask("Summarize this 10-K");
System.out.println(result);
```

---

## 📡 REST / gRPC API

| Endpoint | Description |
|----------|-------------|
| `POST /agent/run` | Run a single agent |
| `POST /workflow/execute` | Run FSM or DAG |
| `POST /team/dispatch` | Run team of agents |
| `GET /monitor` | View session logs, tool usage, memory |

---

## 🔭 Roadmap

- [x] Rust agent runtime
- [x] CLI (`new`, `run`, `serve`)
- [x] Java SDK
- [ ] WebSocket streaming
- [ ] Plugin marketplace
- [ ] Python/JS SDKs
- [ ] Cloud deployment support

---

## 🏁 Mission

> AEGNO is the **fastest**, most composable and language-friendly agent framework — built in Rust, designed for teams, and optimized for production-scale reasoning systems.

---

## 🔗 License

MIT