use femtoclaw_sdk::{Tool, ToolCall, ToolResult};

pub struct ExampleTalon;

impl ExampleTalon {
    pub fn new() -> Self {
        Self
    }

    pub fn greet(&self, name: &str, enthusiastic: bool) -> ToolResult {
        let message = if enthusiastic {
            format!("Hello, {}!!!", name)
        } else {
            format!("Hello, {}!", name)
        };
        ToolResult::success(serde_json::json!({ "message": message }))
    }

    pub fn echo(&self, text: &str) -> ToolResult {
        ToolResult::success(serde_json::json!({ "echo": text }))
    }
}

impl Tool for ExampleTalon {
    fn name(&self) -> &str {
        "example"
    }

    fn execute(&self, call: ToolCall) -> ToolResult {
        match call.tool.as_str() {
            "greet" => {
                let name = call
                    .args
                    .get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("World");
                let enthusiastic = call
                    .args
                    .get("enthusiastic")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                self.greet(name, enthusiastic)
            }
            "echo" => {
                let text = call.args.get("text").and_then(|v| v.as_str()).unwrap_or("");
                self.echo(text)
            }
            _ => ToolResult::error("Unknown command"),
        }
    }
}
