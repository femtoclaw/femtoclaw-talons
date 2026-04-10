use femtoclaw_sdk::{Tool, ToolCall, ToolResult};

pub struct TemplateTalon;

impl TemplateTalon {
    pub fn new() -> Self {
        Self
    }

    pub fn list_engines(&self) -> ToolResult {
        let engines = vec![
            "mustache".to_string(),
            "handlebars".to_string(),
            "jinja".to_string(),
        ];
        ToolResult::success(serde_json::json!({ "engines": engines }))
    }

    pub fn render(&self, template: &str, variables: &str) -> ToolResult {
        ToolResult::success(serde_json::json!({
            "rendered": format!("Template rendered with variables: {}", variables)
        }))
    }
}

impl Tool for TemplateTalon {
    fn name(&self) -> &str {
        "template"
    }

    fn execute(&self, call: ToolCall) -> ToolResult {
        match call.tool.as_str() {
            "list_engines" => self.list_engines(),
            "render" => {
                let template = call
                    .args
                    .get("template")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let variables = call
                    .args
                    .get("variables")
                    .map(|v| v.to_string())
                    .unwrap_or_default();
                self.render(template, &variables)
            }
            _ => ToolResult::error("Unknown command"),
        }
    }
}
