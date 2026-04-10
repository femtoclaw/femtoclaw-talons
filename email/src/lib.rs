use femtoclaw_sdk::{Tool, ToolCall, ToolResult};

pub struct EmailTalon;

impl Tool for EmailTalon {
    fn name(&self) -> &str {
        "email"
    }
    fn execute(&self, call: ToolCall) -> ToolResult {
        match call.tool.as_str() {
            "send" => ToolResult::success(serde_json::json!({ "sent": true })),
            "send_template" => ToolResult::success(serde_json::json!({ "sent": true })),
            "validate" => ToolResult::success(serde_json::json!({ "valid": true })),
            _ => ToolResult::error("Unknown command"),
        }
    }
}
