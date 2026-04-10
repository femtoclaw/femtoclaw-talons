use femtoclaw_sdk::{Tool, ToolCall, ToolResult};

pub struct DiscordTalon;

impl Tool for DiscordTalon {
    fn name(&self) -> &str {
        "discord"
    }
    fn execute(&self, call: ToolCall) -> ToolResult {
        match call.tool.as_str() {
            "send_message" => ToolResult::success(serde_json::json!({ "sent": true })),
            "send_embed" => ToolResult::success(serde_json::json!({ "sent": true })),
            "list_channels" => ToolResult::success(serde_json::json!({ "channels": [] })),
            _ => ToolResult::error("Unknown command"),
        }
    }
}
