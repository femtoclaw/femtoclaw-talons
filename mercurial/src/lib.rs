use femtoclaw_sdk::{Tool, ToolCall, ToolResult};

pub struct MercurialTalon;

impl Tool for MercurialTalon {
    fn name(&self) -> &str {
        "mercurial"
    }
    fn execute(&self, call: ToolCall) -> ToolResult {
        match call.tool.as_str() {
            "status" => ToolResult::success(serde_json::json!({ "status": "clean" })),
            _ => ToolResult::error("Unknown command"),
        }
    }
}
