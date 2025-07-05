#[cfg(test)]
mod tests {
    use crate::team::{Team, TeamAgent, TeamWorkflow};

    #[test]
    fn test_team_creation() {
        let mut team = Team::new("TestTeam".to_string());
        
        let agent1 = TeamAgent {
            name: "researcher".to_string(),
            role: "Data Researcher".to_string(),
            instructions: "Research data".to_string(),
            tools: vec!["search".to_string()],
            model: "openai:gpt-4-turbo".to_string(),
        };
        
        let agent2 = TeamAgent {
            name: "analyst".to_string(),
            role: "Analyst".to_string(),
            instructions: "Analyze data".to_string(),
            tools: vec!["math".to_string()],
            model: "openai:gpt-4-turbo".to_string(),
        };
        
        team.add_agent(agent1);
        team.add_agent(agent2);
        
        assert_eq!(team.name, "TestTeam");
        assert_eq!(team.agents.len(), 2);
        assert_eq!(team.agents[0].name, "researcher");
        assert_eq!(team.agents[1].name, "analyst");
    }

    #[test]
    fn test_workflow_types() {
        // Test Chain of Thought
        let cot_workflow = TeamWorkflow::ChainOfThought;
        
        // Test FSM
        let fsm_workflow = TeamWorkflow::FSM {
            states: vec!["start".to_string(), "process".to_string(), "end".to_string()],
            transitions: vec![],
            initial_state: "start".to_string(),
        };
        
        // Test DAG
        let dag_workflow = TeamWorkflow::DAG {
            nodes: vec![],
            edges: vec![],
        };
        
        assert!(matches!(cot_workflow, TeamWorkflow::ChainOfThought));
        assert!(matches!(fsm_workflow, TeamWorkflow::FSM { .. }));
        assert!(matches!(dag_workflow, TeamWorkflow::DAG { .. }));
    }
} 