use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Tracks the state of workflow execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowState {
    pub workflow_id: String,
    pub current_step: usize,
    pub current_agent: Option<String>,
    pub variables: HashMap<String, String>,
    pub start_time: u64,
    pub step_results: Vec<StepResult>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepResult {
    pub step_index: usize,
    pub agent: String,
    pub input: String,
    pub output: String,
    pub timestamp: u64,
    pub duration_ms: u64,
}

impl WorkflowState {
    pub fn new(workflow_id: String) -> Self {
        let start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
            
        Self {
            workflow_id,
            current_step: 0,
            current_agent: None,
            variables: HashMap::new(),
            start_time,
            step_results: Vec::new(),
            metadata: HashMap::new(),
        }
    }
    
    pub fn set_current_step(&mut self, step: usize) {
        self.current_step = step;
    }
    
    pub fn set_current_agent(&mut self, agent: &str) {
        self.current_agent = Some(agent.to_string());
    }
    
    pub fn set_variable(&mut self, key: &str, value: &str) {
        self.variables.insert(key.to_string(), value.to_string());
    }
    
    pub fn get_variable(&self, key: &str) -> Option<&String> {
        self.variables.get(key)
    }
    
    pub fn add_step_result(&mut self, result: StepResult) {
        self.step_results.push(result);
    }
    
    pub fn set_metadata(&mut self, key: &str, value: &str) {
        self.metadata.insert(key.to_string(), value.to_string());
    }
    
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }
    
    pub fn is_complete(&self) -> bool {
        // This would be determined by the specific workflow type
        false
    }
    
    pub fn get_execution_time(&self) -> u64 {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        current_time - self.start_time
    }
}
