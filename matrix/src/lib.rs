use femtoclaw_sdk::{Tool, ToolCall, ToolResult};

pub struct MatrixTalon;

impl Tool for MatrixTalon {
    fn name(&self) -> &str {
        "matrix"
    }
    fn execute(&self, call: ToolCall) -> ToolResult {
        match call.tool.as_str() {
            "send_message" => ToolResult::success(serde_json::json!({ "sent": true })),
            "list_rooms" => ToolResult::success(serde_json::json!({ "rooms": [] })),
            _ => ToolResult::error("Unknown command"),
        }
    }
}
