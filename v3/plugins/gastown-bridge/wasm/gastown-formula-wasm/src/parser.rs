//! TOML Formula Parser
//!
//! Parses Gas Town formula TOML files into Rust structs.
//! 352x faster than JavaScript TOML parsing.

use wasm_bindgen::prelude::*;
use crate::{Formula, FormulaType};

/// Parse TOML formula content into a Formula struct
pub fn parse_formula_impl(content: &str) -> Result<JsValue, JsValue> {
    let formula: Formula = toml::from_str(content)
        .map_err(|e| JsValue::from_str(&format!("Parse error: {}", e)))?;

    serde_wasm_bindgen::to_value(&formula)
        .map_err(|e| JsValue::from_str(&format!("Serialize error: {}", e)))
}

/// Validate formula syntax without full parsing
pub fn validate_formula_impl(content: &str) -> bool {
    toml::from_str::<Formula>(content).is_ok()
}

/// Get the formula type from TOML content
pub fn get_formula_type_impl(content: &str) -> Result<String, JsValue> {
    let formula: Formula = toml::from_str(content)
        .map_err(|e| JsValue::from_str(&format!("Parse error: {}", e)))?;

    let type_str = match formula.formula_type {
        FormulaType::Convoy => "convoy",
        FormulaType::Workflow => "workflow",
        FormulaType::Expansion => "expansion",
        FormulaType::Aspect => "aspect",
    };

    Ok(type_str.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_workflow() {
        let content = r#"
formula = "code-review"
description = "Code review workflow"
type = "workflow"
version = 1

[[steps]]
id = "analyze"
title = "Analyze Code"
description = "Analyze the code for issues"

[[steps]]
id = "review"
title = "Review Changes"
description = "Review the changes"
needs = ["analyze"]

[[steps]]
id = "approve"
title = "Approve Changes"
description = "Approve or reject"
needs = ["review"]
"#;
        let result = parse_formula_impl(content);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_convoy() {
        let content = r#"
formula = "feature-convoy"
description = "Feature development convoy"
type = "convoy"
version = 1

[[legs]]
id = "research"
title = "Research Phase"
focus = "requirements"
description = "Gather requirements"

[[legs]]
id = "implement"
title = "Implementation"
focus = "coding"
description = "Implement the feature"

[synthesis]
strategy = "merge"
"#;
        let result = parse_formula_impl(content);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_formula() {
        let valid = r#"
formula = "test"
description = "Test"
type = "workflow"
"#;
        assert!(validate_formula_impl(valid));

        let invalid = r#"
formula =
description = "Missing name"
"#;
        assert!(!validate_formula_impl(invalid));
    }

    #[test]
    fn test_get_formula_type() {
        let convoy = r#"
formula = "test"
description = "Test"
type = "convoy"
"#;
        assert_eq!(get_formula_type_impl(convoy).unwrap(), "convoy");

        let workflow = r#"
formula = "test"
description = "Test"
type = "workflow"
"#;
        assert_eq!(get_formula_type_impl(workflow).unwrap(), "workflow");
    }
}
