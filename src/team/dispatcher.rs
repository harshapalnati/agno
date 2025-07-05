use crate::agent::Agent;
use crate::memory::sqlite::SqliteMemory;
use crate::model::openai::OpenAiClient;
use crate::team::{Team, TeamWorkflow, StateTransition, DAGNode, DAGEdge};
use crate::tool::load_tools;
use crate::workflow::runner::WorkflowRunner;
use std::collections::HashMap;
use std::sync::Arc;

/// Dispatches tasks to team members and manages workflow execution
pub struct TeamDispatcher {
    team: Team,
    agents: HashMap<String, Agent>,
    runner: WorkflowRunner,
}

impl TeamDispatcher {
    pub async fn new(team: Team) -> Result<Self, Box<dyn std::error::Error>> {
        let mut agents = HashMap::new();
        
        // Initialize each agent in the team
        for team_agent in &team.agents {
            let api_key = std::env::var("OPENAI_API_KEY")
                .expect("❌ Missing OPENAI_API_KEY in environment");
            
            let model = Box::new(OpenAiClient::new(api_key));
            let tools = load_tools(&team_agent.tools);
            
            // Use shared memory if specified, otherwise individual memory
            let memory_path = team.shared_memory.clone()
                .unwrap_or_else(|| format!("memory_{}.db", team_agent.name));
            let memory = Arc::new(SqliteMemory::new(&memory_path)?);
            
            let agent = Agent::new(
                team_agent.name.clone(),
                team_agent.instructions.clone(),
                model,
                tools,
                memory,
            );
            
            agents.insert(team_agent.name.clone(), agent);
        }
        
        let runner = WorkflowRunner::new();
        
        Ok(Self {
            team,
            agents,
            runner,
        })
    }
    
    /// Execute a task using the team's workflow
    pub async fn execute(&mut self, task: &str) -> Result<String, Box<dyn std::error::Error>> {
        println!("🤝 Team '{}' executing task: {}", self.team.name, task);
        
        // Clone the workflow to avoid borrowing issues
        let workflow = self.team.workflow.clone();
        
        match workflow {
            TeamWorkflow::RoundRobin => {
                self.execute_round_robin(task).await
            }
            TeamWorkflow::ChainOfThought => {
                self.execute_chain_of_thought(task).await
            }
            TeamWorkflow::Parallel => {
                self.execute_parallel(task).await
            }
            TeamWorkflow::FSM { states, transitions, initial_state } => {
                // Use FSM config if available, otherwise use the workflow's built-in config
                if let Some(fsm_config) = &self.team.fsm {
                    let states = fsm_config.states.clone();
                    let transitions = fsm_config.transitions.clone();
                    let initial_state = fsm_config.initial_state.clone();
                    self.execute_fsm(task, &states, &transitions, &initial_state).await
                } else {
                    self.execute_fsm(task, &states, &transitions, &initial_state).await
                }
            }
            TeamWorkflow::DAG { nodes, edges } => {
                // Use DAG config if available, otherwise use the workflow's built-in config
                if let Some(dag_config) = &self.team.dag {
                    let nodes = dag_config.nodes.clone();
                    let edges = dag_config.edges.clone();
                    self.execute_dag(task, &nodes, &edges).await
                } else {
                    self.execute_dag(task, &nodes, &edges).await
                }
            }
        }
    }
    
    /// Round-robin: each agent gets a turn
    async fn execute_round_robin(&mut self, task: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut results = Vec::new();
        
        for (agent_name, agent) in &mut self.agents {
            println!("🔄 {} taking turn...", agent_name);
            
            // Run the agent with the task directly
            let output = run_agent_silently(agent, task).await;
            results.push(format!("{}: {}", agent_name, output));
        }
        
        Ok(results.join("\n\n"))
    }
    
    /// Chain-of-Thought: agents pass results to next agent
    async fn execute_chain_of_thought(&mut self, task: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut current_input = task.to_string();
        let mut results = Vec::new();
        
        let agent_names: Vec<String> = self.agents.keys().cloned().collect();
        
        for (i, agent_name) in agent_names.iter().enumerate() {
            println!("🔗 {} in chain (step {})...", agent_name, i + 1);
            
            // Get the actual agent and run it
            if let Some(agent) = self.agents.get_mut(agent_name) {
                // Run the agent with the current input directly
                let output = run_agent_silently(agent, &current_input).await;
                results.push(format!("Step {} ({}): {}", i + 1, agent_name, output));
                current_input = output; // Pass output to next agent
            } else {
                results.push(format!("Step {} ({}): Agent not found", i + 1, agent_name));
            }
        }
        
        Ok(results.join("\n\n"))
    }
    
    /// Parallel: all agents work simultaneously on the same task
    async fn execute_parallel(&mut self, task: &str) -> Result<String, Box<dyn std::error::Error>> {
        println!("⚡ Executing parallel workflow...");
        
        // For now, we'll run agents sequentially to avoid borrowing issues
        // In a real implementation, you'd want to use Arc<Mutex<Agent>> or similar
        let mut results = Vec::new();
        
        for (agent_name, agent) in &mut self.agents {
            println!("⚡ {} working in parallel...", agent_name);
            
            // Run the agent with the task
            let output = run_agent_silently(agent, task).await;
            results.push(format!("{}: {}", agent_name, output));
        }
        
        Ok(results.join("\n\n"))
    }
    
    /// FSM: Finite State Machine workflow
    async fn execute_fsm(
        &mut self,
        task: &str,
        states: &[String],
        transitions: &[StateTransition],
        initial_state: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut current_state = initial_state.to_string();
        let mut results = Vec::new();
        let mut visited_states = std::collections::HashSet::new();
        
        // Prevent infinite loops
        let max_iterations = states.len() * 2;
        let mut iteration = 0;
        
        while let Some(state) = states.iter().find(|s| **s == current_state) {
            iteration += 1;
            if iteration > max_iterations {
                results.push("⚠️ FSM: Maximum iterations reached, stopping to prevent infinite loop".to_string());
                break;
            }
            
            if visited_states.contains(&current_state) {
                results.push(format!("⚠️ FSM: State '{}' already visited, stopping loop", current_state));
                break;
            }
            
            visited_states.insert(current_state.clone());
            println!("🏭 FSM State: {} (iteration {})", state, iteration);
            
            // Find agent responsible for this state
            if let Some((agent_name, agent)) = self.agents.iter_mut().find(|(name, _)| {
                name.as_str().contains(state) || state.contains(name.as_str())
            }) {
                // Run the agent for this state
                let state_task = format!("State: {}. Task: {}", state, task);
                let output = run_agent_silently(agent, &state_task).await;
                results.push(format!("State {} ({}): {}", state, agent_name, output));
                
                // Determine next state based on transitions and agent output
                let next_state = self.determine_next_state(&current_state, transitions, &output).await;
                
                if let Some(next) = next_state {
                    current_state = next;
                } else {
                    results.push(format!("✅ FSM: No more transitions from state '{}', workflow complete", current_state));
                    break;
                }
            } else {
                results.push(format!("❌ FSM: No agent found for state: {}", state));
                break;
            }
        }
        
        Ok(results.join("\n\n"))
    }
    
    /// Determine the next state based on transitions and agent output
    async fn determine_next_state(
        &self,
        current_state: &str,
        transitions: &[StateTransition],
        agent_output: &str,
    ) -> Option<String> {
        // Find all possible transitions from current state
        let possible_transitions: Vec<_> = transitions
            .iter()
            .filter(|t| t.from == current_state)
            .collect();
        
        if possible_transitions.is_empty() {
            return None; // No transitions available
        }
        
        // For now, use simple logic based on agent output
        // In a real implementation, you might use LLM to determine the condition
        for transition in &possible_transitions {
            match transition.condition.as_str() {
                "issue_received" | "analysis_complete" | "resolution_attempted" => {
                    // These are automatic transitions
                    return Some(transition.to.clone());
                }
                "customer_satisfied" => {
                    // Check if output suggests satisfaction
                    if agent_output.to_lowercase().contains("satisfied") 
                        || agent_output.to_lowercase().contains("resolved")
                        || agent_output.to_lowercase().contains("happy") {
                        return Some(transition.to.clone());
                    }
                }
                "customer_unsatisfied" => {
                    // Check if output suggests dissatisfaction
                    if agent_output.to_lowercase().contains("unsatisfied")
                        || agent_output.to_lowercase().contains("not resolved")
                        || agent_output.to_lowercase().contains("still has issue") {
                        return Some(transition.to.clone());
                    }
                }
                _ => {
                    // Default: take the first transition
                    return Some(transition.to.clone());
                }
            }
        }
        
        // If no specific condition matched, take the first transition
        possible_transitions.first().map(|t| t.to.clone())
    }
    
    /// DAG: Directed Acyclic Graph workflow
    async fn execute_dag(
        &mut self,
        task: &str,
        nodes: &[DAGNode],
        edges: &[DAGEdge],
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut results = Vec::new();
        let mut completed_nodes = std::collections::HashSet::new();
        let mut node_results = std::collections::HashMap::new();
        
        // Find starting nodes (nodes with no incoming edges)
        let mut ready_nodes: Vec<_> = nodes.iter()
            .filter(|node| {
                !edges.iter().any(|edge| edge.to == node.id)
            })
            .collect();
        
        println!("📊 DAG: Starting with {} ready nodes", ready_nodes.len());
        
        while !ready_nodes.is_empty() {
            let node = ready_nodes.remove(0);
            println!("📊 DAG Node: {} (Agent: {}) - {}", node.id, node.agent, node.task);
            
            // Build context before mutable borrow
            let context = self.build_dag_context(&node.id, edges, &node_results).await;
            
            // Find the agent for this node
            if let Some((agent_name, agent)) = self.agents.iter_mut().find(|(name, _)| {
                **name == node.agent
            }) {
                // Prepare task with context from dependencies
                let node_task = format!("Task: {}. Context: {}. Original: {}", node.task, context, task);
                
                // Run the agent
                let output = run_agent_silently(agent, &node_task).await;
                let result = format!("Node {} ({}): {}", node.id, agent_name, output);
                results.push(result.clone());
                
                // Store result for dependent nodes
                node_results.insert(node.id.clone(), output);
                completed_nodes.insert(&node.id);
                
                println!("✅ DAG: Completed node {}", node.id);
            } else {
                results.push(format!("❌ DAG: Agent '{}' not found for node {}", node.agent, node.id));
                completed_nodes.insert(&node.id);
            }
            
            // Find nodes that can now be executed (all dependencies completed)
            for edge in edges {
                if edge.from == node.id {
                    let target_node = nodes.iter().find(|n| n.id == edge.to).unwrap();
                    
                    // Check if all dependencies of target_node are completed
                    let all_deps_completed = edges.iter()
                        .filter(|e| e.to == target_node.id)
                        .all(|e| completed_nodes.contains(&e.from));
                    
                    if all_deps_completed && !ready_nodes.contains(&target_node) {
                        ready_nodes.push(target_node);
                        println!("📊 DAG: Node {} is now ready (dependencies: {:?})", 
                                target_node.id, 
                                edges.iter().filter(|e| e.to == target_node.id).map(|e| &e.from).collect::<Vec<_>>());
                    }
                }
            }
        }
        
        // Check if all nodes were completed
        if completed_nodes.len() < nodes.len() {
            let uncompleted: Vec<_> = nodes.iter()
                .filter(|n| !completed_nodes.contains(&n.id))
                .map(|n| &n.id)
                .collect();
            results.push(format!("⚠️ DAG: Some nodes could not be completed: {:?}", uncompleted));
        }
        
        Ok(results.join("\n\n"))
    }
    
    /// Build context for a DAG node based on its dependencies
    async fn build_dag_context(
        &self,
        node_id: &str,
        edges: &[DAGEdge],
        node_results: &std::collections::HashMap<String, String>,
    ) -> String {
        let dependencies: Vec<_> = edges.iter()
            .filter(|e| e.to == node_id)
            .map(|e| &e.from)
            .collect();
        
        if dependencies.is_empty() {
            return "No dependencies".to_string();
        }
        
        let mut context_parts = Vec::new();
        for dep in dependencies {
            if let Some(result) = node_results.get(dep) {
                context_parts.push(format!("{}: {}", dep, result));
            }
        }
        
        context_parts.join(" | ")
    }
}

/// Run an agent and capture its output using real LLM/tool execution
async fn run_agent_silently(agent: &mut Agent, input: &str) -> String {
    agent.run_once(input).await
} 