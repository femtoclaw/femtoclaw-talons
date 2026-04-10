use femtoclaw_sdk::{Tool, ToolCall, ToolResult};

pub struct SearchTalon;

impl Tool for SearchTalon {
    fn name(&self) -> &str {
        "search"
    }
    fn execute(&self, call: ToolCall) -> ToolResult {
        match call.tool.as_str() {
            "web" => ToolResult::success(serde_json::json!({ "results": [] })),
            "wikipedia" => ToolResult::success(serde_json::json!({ "summary": "" })),
            "news" => ToolResult::success(serde_json::json!({ "articles": [] })),
            _ => ToolResult::error("Unknown command"),
        }
    }
}
