use femtoclaw_sdk::{Tool, ToolCall, ToolResult};

pub struct DockerTalon;

impl Tool for DockerTalon {
    fn name(&self) -> &str {
        "docker"
    }
    fn execute(&self, call: ToolCall) -> ToolResult {
        match call.tool.as_str() {
            "list_containers" => ToolResult::success(serde_json::json!({ "containers": [] })),
            "list_images" => ToolResult::success(serde_json::json!({ "images": [] })),
            "start_container" => ToolResult::success(serde_json::json!({ "started": true })),
            "stop_container" => ToolResult::success(serde_json::json!({ "stopped": true })),
            "pull_image" => ToolResult::success(serde_json::json!({ "pulled": true })),
            _ => ToolResult::error("Unknown command"),
        }
    }
}
