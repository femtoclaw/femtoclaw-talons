use femtoclaw_sdk::{Tool, ToolCall, ToolResult};

pub struct RedisTalon;

impl Tool for RedisTalon {
    fn name(&self) -> &str {
        "redis"
    }
    fn execute(&self, call: ToolCall) -> ToolResult {
        match call.tool.as_str() {
            "get" => ToolResult::success(serde_json::json!({ "value": "" })),
            "set" => ToolResult::success(serde_json::json!({ "set": true })),
            "keys" => ToolResult::success(serde_json::json!({ "keys": [] })),
            _ => ToolResult::error("Unknown command"),
        }
    }
}
