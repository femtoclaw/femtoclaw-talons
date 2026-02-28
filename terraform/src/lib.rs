use femtoclaw_sdk::{Tool, ToolCall, ToolResult};

pub struct TerraformTalon;

impl Tool for TerraformTalon {
    fn name(&self) -> &str {
        "terraform"
    }
    fn execute(&self, call: ToolCall) -> ToolResult {
        match call.tool.as_str() {
            "plan" => ToolResult::success(serde_json::json!({ "plan": "No changes" })),
            "apply" => ToolResult::success(serde_json::json!({ "applied": true })),
            _ => ToolResult::error("Unknown command"),
        }
    }
}
