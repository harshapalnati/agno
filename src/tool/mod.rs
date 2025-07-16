pub mod builtin;
pub mod tool_traits;

use crate::tool::tool_traits::Tool;
use builtin::{echo::EchoTool, math::MathTool, search::SearchTool};

/// Registry for managing available tools
pub struct ToolRegistry {
    tools: std::collections::HashMap<String, Box<dyn Tool + Send + Sync>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            tools: std::collections::HashMap::new(),
        };
        
        // Register built-in tools
        registry.register("math", Box::new(MathTool::new()));
        registry.register("search", Box::new(SearchTool::new()));
        registry.register("echo", Box::new(EchoTool::new()));
        
        registry
    }

    pub fn register(&mut self, name: &str, tool: Box<dyn Tool + Send + Sync>) {
        self.tools.insert(name.to_string(), tool);
    }

    pub fn get(&self, name: &str) -> Option<&Box<dyn Tool + Send + Sync>> {
        self.tools.get(name)
    }

    pub fn list_available(&self) -> Vec<String> {
        self.tools.keys().cloned().collect()
    }
}

/// Dynamically load tools from config-defined names
pub fn load_tools(tool_names: &[String]) -> Vec<Box<dyn Tool + Send + Sync>> {
    let mut tools: Vec<Box<dyn Tool + Send + Sync>> = Vec::new();

    for name in tool_names {
        match name.as_str() {
            "math" => tools.push(Box::new(MathTool::new())),
            "search" => tools.push(Box::new(SearchTool::new())),
            "echo" => tools.push(Box::new(EchoTool::new())),
            unknown => {
                eprintln!("⚠️ Unknown tool: '{}', skipping.", unknown);
            }
        }
    }

    tools
}
