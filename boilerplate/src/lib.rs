use femtoclaw_sdk::{Tool, ToolCall, ToolResult};

pub struct BoilerplateTalon;

impl BoilerplateTalon {
    pub fn new() -> Self {
        Self
    }

    pub fn list_templates(&self) -> ToolResult {
        let templates = vec![
            "rust-cli".to_string(),
            "rust-web".to_string(),
            "python-cli".to_string(),
        ];
        ToolResult::success(serde_json::json!({ "templates": templates }))
    }

    pub fn create_project(&self, template: &str, project_name: &str) -> ToolResult {
        ToolResult::success(serde_json::json!({
            "message": format!("Created project '{}' from template '{}'", project_name, template)
        }))
    }
}

impl Tool for BoilerplateTalon {
    fn name(&self) -> &str {
        "boilerplate"
    }

    fn execute(&self, call: ToolCall) -> ToolResult {
        match call.tool.as_str() {
            "list" => self.list_templates(),
            "create" => {
                let template = call
                    .args
                    .get("template")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let project_name = call
                    .args
                    .get("project_name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                self.create_project(template, project_name)
            }
            _ => ToolResult::error("Unknown command"),
        }
    }
}
