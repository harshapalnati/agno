/// Manages conversation memory and builds prompts for the agent

#[derive(Debug, Default)]
pub struct AgentContext {
    pub memory: Vec<String>,
}

impl AgentContext {
    /// Create a new empty context
    pub fn new() -> Self {
        Self { memory: Vec::new() }
    }

    /// Add a message to memory
    pub fn add_message(&mut self, message: &str) {
        self.memory.push(message.to_string());
    }

    /// Build a full prompt using the agent’s instructions + memory + current input
    pub fn build_context_prompt(&self, instructions: &str, user_input: &str) -> Vec<String> {
        let mut prompt = Vec::new();

        prompt.push(format!("System: {}", instructions.trim()));
        for message in &self.memory {
            prompt.push(message.clone());
        }
        prompt.push(format!("User: {}", user_input.trim()));

        prompt
    }
}
