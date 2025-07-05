use serde::{Deserialize, Serialize};

/// Represents a team of agents that can work together
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    pub name: String,
    pub agents: Vec<TeamAgent>,
    pub workflow: TeamWorkflow,
    pub shared_memory: Option<String>, // Path to shared memory DB
    pub fsm: Option<FSMConfig>, // FSM-specific configuration
    pub dag: Option<DAGConfig>, // DAG-specific configuration
}

/// FSM-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FSMConfig {
    pub states: Vec<String>,
    pub initial_state: String,
    pub transitions: Vec<StateTransition>,
}

/// DAG-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DAGConfig {
    pub nodes: Vec<DAGNode>,
    pub edges: Vec<DAGEdge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamAgent {
    pub name: String,
    pub role: String,
    pub instructions: String,
    pub tools: Vec<String>,
    pub model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(from = "String")]
pub enum TeamWorkflow {
    /// Simple round-robin where each agent gets a turn
    RoundRobin,
    /// Chain-of-Thought: agents pass results to next agent
    ChainOfThought,
    /// Parallel: all agents work simultaneously on the same task
    Parallel,
    /// Finite State Machine with defined transitions
    FSM {
        states: Vec<String>,
        transitions: Vec<StateTransition>,
        initial_state: String,
    },
    /// Directed Acyclic Graph workflow
    DAG {
        nodes: Vec<DAGNode>,
        edges: Vec<DAGEdge>,
    },
}

impl From<String> for TeamWorkflow {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "roundrobin" | "round_robin" => TeamWorkflow::RoundRobin,
            "chainofthought" | "chain_of_thought" | "cot" => TeamWorkflow::ChainOfThought,
            "parallel" => TeamWorkflow::Parallel,
            "fsm" => TeamWorkflow::FSM {
                states: vec![],
                transitions: vec![],
                initial_state: "start".to_string(),
            },
            "dag" => TeamWorkflow::DAG {
                nodes: vec![],
                edges: vec![],
            },
            _ => TeamWorkflow::RoundRobin, // default
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateTransition {
    pub from: String,
    pub to: String,
    pub condition: String, // Simple condition like "task_complete" or "error"
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DAGNode {
    pub id: String,
    pub agent: String,
    pub task: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DAGEdge {
    pub from: String,
    pub to: String,
    pub condition: Option<String>,
}

impl Team {
    pub fn new(name: String) -> Self {
        Self {
            name,
            agents: Vec::new(),
            workflow: TeamWorkflow::RoundRobin,
            shared_memory: None,
            fsm: None,
            dag: None,
        }
    }

    pub fn add_agent(&mut self, agent: TeamAgent) {
        self.agents.push(agent);
    }

    pub fn set_workflow(&mut self, workflow: TeamWorkflow) {
        self.workflow = workflow;
    }

    pub fn set_shared_memory(&mut self, path: String) {
        self.shared_memory = Some(path);
    }
} 