use femtoclaw_sdk::{Tool, ToolCall, ToolResult};

pub struct SqliteTalon;

impl Tool for SqliteTalon {
    fn name(&self) -> &str {
        "sqlite"
    }
    fn execute(&self, call: ToolCall) -> ToolResult {
        match call.tool.as_str() {
            "query" => ToolResult::success(serde_json::json!({ "rows": [] })),
            "execute" => ToolResult::success(serde_json::json!({ "affected": 0 })),
            "list_tables" => ToolResult::success(serde_json::json!({ "tables": [] })),
            _ => ToolResult::error("Unknown command"),
        }
    }
}
