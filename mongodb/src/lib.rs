use femtoclaw_sdk::{Tool, ToolCall, ToolResult};

pub struct MongodbTalon;

impl Tool for MongodbTalon {
    fn name(&self) -> &str {
        "mongodb"
    }
    fn execute(&self, call: ToolCall) -> ToolResult {
        match call.tool.as_str() {
            "find" => ToolResult::success(serde_json::json!({ "documents": [] })),
            "insert" => ToolResult::success(serde_json::json!({ "inserted": true })),
            "update" => ToolResult::success(serde_json::json!({ "updated": true })),
            "delete" => ToolResult::success(serde_json::json!({ "deleted": true })),
            _ => ToolResult::error("Unknown command"),
        }
    }
}
