use femtoclaw_sdk::{Tool, ToolCall, ToolResult};

pub struct SvnTalon;

impl Tool for SvnTalon {
    fn name(&self) -> &str {
        "svn"
    }
    fn execute(&self, call: ToolCall) -> ToolResult {
        match call.tool.as_str() {
            "status" => ToolResult::success(serde_json::json!({ "status": "clean" })),
            _ => ToolResult::error("Unknown command"),
        }
    }
}
