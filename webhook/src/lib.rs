use femtoclaw_sdk::{Tool, ToolCall, ToolResult};

pub struct WebhookTalon;

impl Tool for WebhookTalon {
    fn name(&self) -> &str {
        "webhook"
    }
    fn execute(&self, call: ToolCall) -> ToolResult {
        match call.tool.as_str() {
            "send" => ToolResult::success(serde_json::json!({ "sent": true })),
            "test" => ToolResult::success(serde_json::json!({ "reachable": true })),
            _ => ToolResult::error("Unknown command"),
        }
    }
}
