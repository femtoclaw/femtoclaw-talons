use femtoclaw_sdk::{Tool, ToolCall, ToolResult};

pub struct FetchTalon;

impl Tool for FetchTalon {
    fn name(&self) -> &str {
        "fetch"
    }
    fn execute(&self, call: ToolCall) -> ToolResult {
        match call.tool.as_str() {
            "get" => ToolResult::success(serde_json::json!({ "status": 200, "body": "" })),
            "post" => ToolResult::success(serde_json::json!({ "status": 201, "body": "" })),
            "put" => ToolResult::success(serde_json::json!({ "status": 200, "body": "" })),
            "delete" => ToolResult::success(serde_json::json!({ "status": 204, "body": "" })),
            _ => ToolResult::error("Unknown command"),
        }
    }
}
