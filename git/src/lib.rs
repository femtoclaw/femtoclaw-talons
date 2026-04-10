use femtoclaw_sdk::{Tool, ToolCall, ToolResult};

pub struct GitTalon;

impl Tool for GitTalon {
    fn name(&self) -> &str {
        "git"
    }
    fn execute(&self, call: ToolCall) -> ToolResult {
        match call.tool.as_str() {
            "status" => ToolResult::success(serde_json::json!({ "clean": true })),
            "log" => ToolResult::success(serde_json::json!({ "commits": [] })),
            "branch" => {
                ToolResult::success(serde_json::json!({ "current": "main", "branches": [] }))
            }
            "commit" => ToolResult::success(serde_json::json!({ "committed": true })),
            "push" => ToolResult::success(serde_json::json!({ "pushed": true })),
            "pull" => ToolResult::success(serde_json::json!({ "pulled": true })),
            _ => ToolResult::error("Unknown command"),
        }
    }
}
