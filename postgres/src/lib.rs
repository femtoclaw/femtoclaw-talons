use femtoclaw_sdk::{Tool, ToolCall, ToolResult};

pub struct PostgresTalon;

impl Tool for PostgresTalon {
    fn name(&self) -> &str {
        "postgres"
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
