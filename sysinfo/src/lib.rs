use femtoclaw_sdk::{Tool, ToolCall, ToolResult};

pub struct SysinfoTalon;

impl Tool for SysinfoTalon {
    fn name(&self) -> &str {
        "sysinfo"
    }
    fn execute(&self, call: ToolCall) -> ToolResult {
        match call.tool.as_str() {
            "cpu" => ToolResult::success(serde_json::json!({ "usage": 0.0 })),
            "memory" => ToolResult::success(serde_json::json!({ "total": 0, "used": 0 })),
            "disk" => ToolResult::success(serde_json::json!({ "disks": [] })),
            _ => ToolResult::error("Unknown command"),
        }
    }
}
