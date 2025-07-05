use crate::workflow::{Workflow, WorkflowState, WorkflowType, WorkflowStep};
use std::collections::HashMap;
use uuid::Uuid;

/// Executes workflows and manages their lifecycle
pub struct WorkflowRunner {
    workflows: HashMap<String, Box<dyn Workflow>>,
    active_states: HashMap<String, WorkflowState>,
}

impl WorkflowRunner {
    pub fn new() -> Self {
        Self {
            workflows: HashMap::new(),
            active_states: HashMap::new(),
        }
    }
    
    /// Register a workflow
    pub fn register_workflow(&mut self, name: &str, workflow: Box<dyn Workflow>) {
        self.workflows.insert(name.to_string(), workflow);
    }
    
    /// Execute a workflow by name
    pub async fn execute_workflow(
        &mut self,
        workflow_name: &str,
        input: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let workflow = self.workflows.get(workflow_name)
            .ok_or_else(|| format!("Workflow '{}' not found", workflow_name))?;
        
        let workflow_id = Uuid::new_v4().to_string();
        let mut state = WorkflowState::new(workflow_id.clone());
        
        println!("🚀 Executing workflow: {} (ID: {})", workflow_name, workflow_id);
        
        // Store active state
        self.active_states.insert(workflow_id.clone(), state.clone());
        
        // Execute the workflow
        let result = workflow.execute(input, &mut state).await?;
        
        // Update state with final results
        self.active_states.insert(workflow_id, state);
        
        Ok(result)
    }
    
    /// Get status of active workflows
    pub fn get_active_workflows(&self) -> Vec<WorkflowStatus> {
        self.active_states
            .iter()
            .map(|(id, state)| WorkflowStatus {
                workflow_id: id.clone(),
                current_step: state.current_step,
                current_agent: state.current_agent.clone(),
                execution_time: state.get_execution_time(),
                is_complete: state.is_complete(),
            })
            .collect()
    }
    
    /// Create a simple Chain-of-Thought workflow
    pub fn create_cot_workflow(&mut self, name: &str, steps: Vec<WorkflowStep>) {
        use crate::workflow::workflow_trait::ChainOfThoughtWorkflow;
        
        let workflow = ChainOfThoughtWorkflow::new(steps);
        self.register_workflow(name, Box::new(workflow));
    }
}

#[derive(Debug, Clone)]
pub struct WorkflowStatus {
    pub workflow_id: String,
    pub current_step: usize,
    pub current_agent: Option<String>,
    pub execution_time: u64,
    pub is_complete: bool,
}
