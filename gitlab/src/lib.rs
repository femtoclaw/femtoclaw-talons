use femtoclaw_sdk::{Tool, ToolCall, ToolResult};

pub struct GitlabTalon;

impl Tool for GitlabTalon {
    fn name(&self) -> &str {
        "gitlab"
    }
    fn execute(&self, call: ToolCall) -> ToolResult {
        match call.tool.as_str() {
            "list_projects" => ToolResult::success(serde_json::json!({ "projects": [] })),
            "create_issue" => ToolResult::success(serde_json::json!({ "created": true })),
            "list_merge_requests" => {
                ToolResult::success(serde_json::json!({ "merge_requests": [] }))
            }
            _ => ToolResult::error("Unknown command"),
        }
    }
}
