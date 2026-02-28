use femtoclaw_sdk::{Tool, ToolCall, ToolResult};

pub struct ElasticsearchTalon;

impl Tool for ElasticsearchTalon {
    fn name(&self) -> &str {
        "elasticsearch"
    }
    fn execute(&self, call: ToolCall) -> ToolResult {
        match call.tool.as_str() {
            "search" => ToolResult::success(serde_json::json!({ "hits": [] })),
            "index" => ToolResult::success(serde_json::json!({ "indexed": true })),
            "list_indices" => ToolResult::success(serde_json::json!({ "indices": [] })),
            _ => ToolResult::error("Unknown command"),
        }
    }
}
