use femtoclaw_sdk::{Tool, ToolCall, ToolResult};

pub struct GithubTalon;

impl Tool for GithubTalon {
    fn name(&self) -> &str {
        "github"
    }
    fn execute(&self, call: ToolCall) -> ToolResult {
        match call.tool.as_str() {
            "list_repos" => ToolResult::success(serde_json::json!({ "repos": [] })),
            "create_issue" => ToolResult::success(serde_json::json!({ "created": true })),
            "list_issues" => ToolResult::success(serde_json::json!({ "issues": [] })),
            "create_pr" => ToolResult::success(serde_json::json!({ "created": true })),
            _ => ToolResult::error("Unknown command"),
        }
    }
}
