use femtoclaw_sdk::{Tool, ToolCall, ToolResult};

pub struct ScaffoldTalon;

impl ScaffoldTalon {
    pub fn new() -> Self {
        Self
    }

    pub fn list_patterns(&self) -> ToolResult {
        let patterns = vec![
            "rust-crate".to_string(),
            "rust-binary".to_string(),
            "api-endpoint".to_string(),
        ];
        ToolResult::success(serde_json::json!({ "patterns": patterns }))
    }

    pub fn generate(&self, pattern: &str, name: &str) -> ToolResult {
        ToolResult::success(serde_json::json!({
            "message": format!("Generated '{}' from pattern '{}'", name, pattern)
        }))
    }
}

impl Tool for ScaffoldTalon {
    fn name(&self) -> &str {
        "scaffold"
    }

    fn execute(&self, call: ToolCall) -> ToolResult {
        match call.tool.as_str() {
            "list_patterns" => self.list_patterns(),
            "generate" => {
                let pattern = call
                    .args
                    .get("pattern")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let name = call.args.get("name").and_then(|v| v.as_str()).unwrap_or("");
                self.generate(pattern, name)
            }
            _ => ToolResult::error("Unknown command"),
        }
    }
}
