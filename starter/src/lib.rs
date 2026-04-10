use femtoclaw_sdk::{Tool, ToolCall, ToolResult};

pub struct StarterTalon;

impl Tool for StarterTalon {
    fn name(&self) -> &str {
        "starter"
    }
    fn execute(&self, call: ToolCall) -> ToolResult {
        match call.tool.as_str() {
            "init" => ToolResult::success(serde_json::json!({ "initialized": true })),
            "list_templates" => ToolResult::success(serde_json::json!({ "templates": [] })),
            _ => ToolResult::error("Unknown command"),
        }
    }
}
