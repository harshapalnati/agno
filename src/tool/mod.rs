pub mod builtin;
pub mod tool_traits;

use crate::tool::tool_traits::Tool;
use builtin::{echo::EchoTool, math::MathTool, search::SearchTool};

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
