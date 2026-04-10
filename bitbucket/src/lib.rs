use femtoclaw_sdk::{Tool, ToolCall, ToolResult};

pub struct BitbucketTalon;

impl Tool for BitbucketTalon {
    fn name(&self) -> &str {
        "bitbucket"
    }
    fn execute(&self, call: ToolCall) -> ToolResult {
        match call.tool.as_str() {
            "list_repos" => ToolResult::success(serde_json::json!({ "repos": [] })),
            _ => ToolResult::error("Unknown command"),
        }
    }
}
