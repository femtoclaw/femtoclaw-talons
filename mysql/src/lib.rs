use femtoclaw_sdk::{Tool, ToolCall, ToolResult};

pub struct MysqlTalon;

impl Tool for MysqlTalon {
    fn name(&self) -> &str {
        "mysql"
    }
    fn execute(&self, call: ToolCall) -> ToolResult {
        match call.tool.as_str() {
            "query" => ToolResult::success(serde_json::json!({ "rows": [] })),
            "execute" => ToolResult::success(serde_json::json!({ "affected": 0 })),
            "list_databases" => ToolResult::success(serde_json::json!({ "databases": [] })),
            _ => ToolResult::error("Unknown command"),
        }
    }
}
