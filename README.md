#  AEGNO

**AEGNO** is a blazing-fast, developer-friendly, and language-agnostic framework for building multi-agent systems. Built entirely in **Rust**, AEGNO allows developers to create intelligent agents with memory, reasoning, tools, and workflows — accessible from **Java, Rust**, and (soon) **Python, JS, Go**, and more.

---

## ✨ Key Features

- ⚡ **Rust Core** – Fully async, memory-safe, and ultra-performant
- ⚙️ **Modular & Pluggable** – Add tools, models, workflows via trait-based architecture
- 🌍 **Language Agnostic** – Exposes agents via gRPC/HTTP for cross-language use
- 🛠️ **CLI-First Developer Experience** – `aegno run agent.toml` in seconds
- 🧠 **Reasoning & Workflows** – Supports Chain-of-Thought, FSMs, DAGs
- 🧩 **Structured Memory** – Built-in support for SQLite, Redis, Qdrant
- 📡 **API Ready** – Axum-powered REST & gRPC endpoints
- ☕ **Java SDK Available** – Simple Java client to run agents

---

##  Architecture

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

##  File Structure (Rust Core)

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

##  Example: `agent.toml`

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

---

##  Usage

###  Run an Agent

```bash
aegno run agent.toml
```

###  Create a New Agent

```bash
aegno new summarizer
```

###  Add a Tool

```bash
aegno add-tool yfinance
```

###  Serve as API

```bash
aegno serve
```

---

##  Java SDK (Preview)

```java
AgentClient client = new AgentClient("http://localhost:8080");
String result = client.ask("Summarize this 10-K");
System.out.println(result);
```

---

##  REST / gRPC API

| Endpoint | Description |
|----------|-------------|
| `POST /agent/run` | Run a single agent |
| `POST /workflow/execute` | Run FSM or DAG |
| `POST /team/dispatch` | Run team of agents |
| `GET /monitor` | View session logs, tool usage, memory |

---

