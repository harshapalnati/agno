use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::workflow::WorkflowState;

/// Trait for different workflow implementations
#[async_trait]
pub trait Workflow: Send + Sync {
    /// Execute the workflow with given input
    async fn execute(&self, input: &str, state: &mut WorkflowState) -> Result<String, Box<dyn std::error::Error>>;
    
    /// Get workflow metadata
    fn metadata(&self) -> WorkflowMetadata;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowMetadata {
    pub name: String,
    pub description: String,
    pub workflow_type: WorkflowType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkflowType {
    ChainOfThought,
    FSM,
    DAG,
    Custom(String),
}

/// Chain-of-Thought workflow implementation
pub struct ChainOfThoughtWorkflow {
    steps: Vec<WorkflowStep>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStep {
    pub name: String,
    pub description: String,
    pub agent: String,
    pub tools: Vec<String>,
}

impl ChainOfThoughtWorkflow {
    pub fn new(steps: Vec<WorkflowStep>) -> Self {
        Self { steps }
    }
}

#[async_trait]
impl Workflow for ChainOfThoughtWorkflow {
    async fn execute(&self, input: &str, state: &mut WorkflowState) -> Result<String, Box<dyn std::error::Error>> {
        let mut current_input = input.to_string();
        let mut results = Vec::new();
        
        for (i, step) in self.steps.iter().enumerate() {
            println!("🔗 CoT Step {}: {} ({})", i + 1, step.name, step.agent);
            
            // Store step in state
            state.set_current_step(i);
            state.set_current_agent(&step.agent);
            
            // In a real implementation, you'd run the actual agent here
            let output = format!("Step {} ({}): Processed '{}'", i + 1, step.agent, current_input);
            results.push(output.clone());
            
            // Pass output to next step
            current_input = output;
        }
        
        Ok(results.join("\n\n"))
    }
    
    fn metadata(&self) -> WorkflowMetadata {
        WorkflowMetadata {
            name: "Chain of Thought".to_string(),
            description: "Sequential reasoning workflow".to_string(),
            workflow_type: WorkflowType::ChainOfThought,
        }
    }
} 