use femtoclaw_sdk::{Tool, ToolCall, ToolResult};

pub struct MattermostTalon;

impl Tool for MattermostTalon {
    fn name(&self) -> &str {
        "mattermost"
    }
    fn execute(&self, call: ToolCall) -> ToolResult {
        match call.tool.as_str() {
            "send_message" => ToolResult::success(serde_json::json!({ "sent": true })),
            "list_channels" => ToolResult::success(serde_json::json!({ "channels": [] })),
            _ => ToolResult::error("Unknown command"),
        }
    }
}
