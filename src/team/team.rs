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

/// Builder for creating Team instances with a fluent API
pub struct TeamBuilder {
    name: String,
    agents: Vec<TeamAgent>,
    workflow: Option<TeamWorkflow>,
    shared_memory: Option<String>,
    fsm: Option<FSMConfig>,
    dag: Option<DAGConfig>,
}

impl TeamBuilder {
    /// Create a new TeamBuilder with the given name
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            agents: Vec::new(),
            workflow: None,
            shared_memory: None,
            fsm: None,
            dag: None,
        }
    }

    /// Add an agent to the team
    pub fn with_agent(mut self, agent: TeamAgent) -> Self {
        self.agents.push(agent);
        self
    }

    /// Set the workflow for the team
    pub fn with_workflow(mut self, workflow: TeamWorkflow) -> Self {
        self.workflow = Some(workflow);
        self
    }

    /// Set shared memory path
    pub fn with_shared_memory(mut self, path: impl Into<String>) -> Self {
        self.shared_memory = Some(path.into());
        self
    }

    /// Set FSM configuration
    pub fn with_fsm(mut self, fsm: FSMConfig) -> Self {
        self.fsm = Some(fsm);
        self
    }

    /// Set DAG configuration
    pub fn with_dag(mut self, dag: DAGConfig) -> Self {
        self.dag = Some(dag);
        self
    }

    /// Build the Team
    pub fn build(self) -> Team {
        let workflow = self.workflow.unwrap_or(TeamWorkflow::RoundRobin);
        
        Team {
            name: self.name,
            agents: self.agents,
            workflow,
            shared_memory: self.shared_memory,
            fsm: self.fsm,
            dag: self.dag,
        }
    }
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