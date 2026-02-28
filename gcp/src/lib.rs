use femtoclaw_sdk::{Tool, ToolCall, ToolResult};

pub struct GcpTalon;

impl Tool for GcpTalon {
    fn name(&self) -> &str {
        "gcp"
    }
    fn execute(&self, call: ToolCall) -> ToolResult {
        match call.tool.as_str() {
            "list_instances" => ToolResult::success(serde_json::json!({ "instances": [] })),
            _ => ToolResult::error("Unknown command"),
        }
    }
}
