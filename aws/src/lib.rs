use femtoclaw_sdk::{Tool, ToolCall, ToolResult};

pub struct AwsTalon;

impl Tool for AwsTalon {
    fn name(&self) -> &str {
        "aws"
    }
    fn execute(&self, call: ToolCall) -> ToolResult {
        match call.tool.as_str() {
            "list_instances" => ToolResult::success(serde_json::json!({ "instances": [] })),
            "list_buckets" => ToolResult::success(serde_json::json!({ "buckets": [] })),
            "list_functions" => ToolResult::success(serde_json::json!({ "functions": [] })),
            _ => ToolResult::error("Unknown command"),
        }
    }
}
