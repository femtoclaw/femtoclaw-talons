use femtoclaw_sdk::{Tool, ToolCall, ToolResult};

pub struct TelegramTalon;

impl Tool for TelegramTalon {
    fn name(&self) -> &str {
        "telegram"
    }
    fn execute(&self, call: ToolCall) -> ToolResult {
        match call.tool.as_str() {
            "send_message" => ToolResult::success(serde_json::json!({ "sent": true })),
            "send_photo" => ToolResult::success(serde_json::json!({ "sent": true })),
            "send_document" => ToolResult::success(serde_json::json!({ "sent": true })),
            _ => ToolResult::error("Unknown command"),
        }
    }
}
