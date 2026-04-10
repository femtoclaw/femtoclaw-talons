use femtoclaw_sdk::{Tool, ToolCall, ToolResult};

pub struct TerminalTalon;

impl Tool for TerminalTalon {
    fn name(&self) -> &str {
        "terminal"
    }
    fn execute(&self, call: ToolCall) -> ToolResult {
        match call.tool.as_str() {
            "execute" => ToolResult::success(serde_json::json!({ "output": "" })),
            "list_processes" => ToolResult::success(serde_json::json!({ "processes": [] })),
            "system_info" => ToolResult::success(serde_json::json!({ "info": {} })),
            _ => ToolResult::error("Unknown command"),
        }
    }
}
