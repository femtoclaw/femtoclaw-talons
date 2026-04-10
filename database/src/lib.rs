use femtoclaw_sdk::{Tool, ToolCall, ToolResult};

pub struct DatabaseTalon;

impl Tool for DatabaseTalon {
    fn name(&self) -> &str {
        "database"
    }
    fn execute(&self, call: ToolCall) -> ToolResult {
        match call.tool.as_str() {
            "query" => ToolResult::success(serde_json::json!({ "rows": [] })),
            "execute" => ToolResult::success(serde_json::json!({ "affected": 0 })),
            "backup" => ToolResult::success(serde_json::json!({ "backed_up": true })),
            "list_tables" => ToolResult::success(serde_json::json!({ "tables": [] })),
            _ => ToolResult::error("Unknown command"),
        }
    }
}
