use femtoclaw_sdk::{Tool, ToolCall, ToolResult};

pub struct SlackTalon;

impl Tool for SlackTalon {
    fn name(&self) -> &str {
        "slack"
    }
    fn execute(&self, call: ToolCall) -> ToolResult {
        match call.tool.as_str() {
            "send_message" => ToolResult::success(serde_json::json!({ "sent": true })),
            "list_channels" => ToolResult::success(serde_json::json!({ "channels": [] })),
            "upload_file" => ToolResult::success(serde_json::json!({ "uploaded": true })),
            _ => ToolResult::error("Unknown command"),
        }
    }
}
