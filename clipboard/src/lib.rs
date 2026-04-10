use femtoclaw_sdk::{Tool, ToolCall, ToolResult};

pub struct ClipboardTalon;

impl Tool for ClipboardTalon {
    fn name(&self) -> &str {
        "clipboard"
    }
    fn execute(&self, call: ToolCall) -> ToolResult {
        match call.tool.as_str() {
            "read" => ToolResult::success(serde_json::json!({ "content": "" })),
            "write" => ToolResult::success(serde_json::json!({ "copied": true })),
            _ => ToolResult::error("Unknown command"),
        }
    }
}
