use femtoclaw_sdk::{Tool, ToolCall, ToolResult};

pub struct DemoTalon;

impl Tool for DemoTalon {
    fn name(&self) -> &str {
        "demo"
    }
    fn execute(&self, call: ToolCall) -> ToolResult {
        match call.tool.as_str() {
            "hello" => {
                ToolResult::success(serde_json::json!({ "message": "Hello from Demo Skill!" }))
            }
            "echo" => {
                let text = call.args.get("text").and_then(|v| v.as_str()).unwrap_or("");
                ToolResult::success(serde_json::json!({ "echo": text }))
            }
            "timestamp" => ToolResult::success(
                serde_json::json!({ "timestamp": chrono::Utc::now().to_rfc3339() }),
            ),
            _ => ToolResult::error("Unknown command"),
        }
    }
}
