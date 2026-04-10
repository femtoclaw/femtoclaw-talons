use femtoclaw_sdk::{Tool, ToolCall, ToolResult};

pub struct CalculatorTalon;

impl Tool for CalculatorTalon {
    fn name(&self) -> &str {
        "calculator"
    }
    fn execute(&self, call: ToolCall) -> ToolResult {
        match call.tool.as_str() {
            "add" => {
                let a = call.args.get("a").and_then(|v| v.as_f64()).unwrap_or(0.0);
                let b = call.args.get("b").and_then(|v| v.as_f64()).unwrap_or(0.0);
                ToolResult::success(serde_json::json!({ "result": a + b }))
            }
            "subtract" => {
                let a = call.args.get("a").and_then(|v| v.as_f64()).unwrap_or(0.0);
                let b = call.args.get("b").and_then(|v| v.as_f64()).unwrap_or(0.0);
                ToolResult::success(serde_json::json!({ "result": a - b }))
            }
            "multiply" => {
                let a = call.args.get("a").and_then(|v| v.as_f64()).unwrap_or(0.0);
                let b = call.args.get("b").and_then(|v| v.as_f64()).unwrap_or(0.0);
                ToolResult::success(serde_json::json!({ "result": a * b }))
            }
            "divide" => {
                let a = call.args.get("a").and_then(|v| v.as_f64()).unwrap_or(0.0);
                let b = call.args.get("b").and_then(|v| v.as_f64()).unwrap_or(1.0);
                if b == 0.0 {
                    ToolResult::error("Division by zero")
                } else {
                    ToolResult::success(serde_json::json!({ "result": a / b }))
                }
            }
            "sqrt" => {
                let a = call.args.get("a").and_then(|v| v.as_f64()).unwrap_or(0.0);
                ToolResult::success(serde_json::json!({ "result": a.sqrt() }))
            }
            _ => ToolResult::error("Unknown command"),
        }
    }
}
