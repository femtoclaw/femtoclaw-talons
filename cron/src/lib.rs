use femtoclaw_sdk::{Tool, ToolCall, ToolResult};

pub struct CronTalon;

impl Tool for CronTalon {
    fn name(&self) -> &str {
        "cron"
    }
    fn execute(&self, call: ToolCall) -> ToolResult {
        match call.tool.as_str() {
            "list" => ToolResult::success(serde_json::json!({ "jobs": [] })),
            "add" => ToolResult::success(serde_json::json!({ "added": true })),
            _ => ToolResult::error("Unknown command"),
        }
    }
}
