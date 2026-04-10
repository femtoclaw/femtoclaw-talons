use femtoclaw_sdk::{Tool, ToolCall, ToolResult};

pub struct TeamsTalon;

impl Tool for TeamsTalon {
    fn name(&self) -> &str {
        "teams"
    }
    fn execute(&self, call: ToolCall) -> ToolResult {
        match call.tool.as_str() {
            "send_message" => ToolResult::success(serde_json::json!({ "sent": true })),
            "send_card" => ToolResult::success(serde_json::json!({ "sent": true })),
            _ => ToolResult::error("Unknown command"),
        }
    }
}
