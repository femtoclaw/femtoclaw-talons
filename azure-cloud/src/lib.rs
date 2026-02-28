use femtoclaw_sdk::{Tool, ToolCall, ToolResult};

pub struct AzureCloudTalon;

impl Tool for AzureCloudTalon {
    fn name(&self) -> &str {
        "azure-cloud"
    }
    fn execute(&self, call: ToolCall) -> ToolResult {
        match call.tool.as_str() {
            "list_vms" => ToolResult::success(serde_json::json!({ "vms": [] })),
            _ => ToolResult::error("Unknown command"),
        }
    }
}
