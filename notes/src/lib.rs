use femtoclaw_sdk::{Tool, ToolCall, ToolResult};

pub struct NotesTalon;

impl Tool for NotesTalon {
    fn name(&self) -> &str {
        "notes"
    }
    fn execute(&self, call: ToolCall) -> ToolResult {
        match call.tool.as_str() {
            "list" => ToolResult::success(serde_json::json!({ "notes": [] })),
            "create" => ToolResult::success(serde_json::json!({ "created": true })),
            "read" => ToolResult::success(serde_json::json!({ "content": "" })),
            "update" => ToolResult::success(serde_json::json!({ "updated": true })),
            "delete" => ToolResult::success(serde_json::json!({ "deleted": true })),
            _ => ToolResult::error("Unknown command"),
        }
    }
}
