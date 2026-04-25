use regex::Regex;
use serde_json::Value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse_debug_to_json(input: &str) -> String {
    // Strip Object wrappers when followed by a JSON-like body,
    // e.g. `Object { "key": ... }` becomes `{ "key": ... }`.
    // Otherwise, serde_dbgfmt fails to parse the Debug string, returning
    // "unexpected token `"key"`, expected an identifier"
    let normalized_input = Regex::new(r#"\bObject\s*\{\s*""#)
        .expect("regex should be valid")
        .replace_all(input, "{\"")
        .into_owned();

    match serde_dbgfmt::from_str::<Value>(&normalized_input) {
        Ok(parsed_data) => {
            match serde_json::to_string_pretty(&parsed_data) {
                Ok(json_string) => json_string,
                Err(e) => format!("Error formatting JSON: {}", e),
            }
        }
        Err(e) => format!("Error parsing debug output: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Import the functions from above
    use indoc::indoc; // For handling indented multiline strings

    #[test]
    fn test_successful_parse() {
        let debug_str = indoc! {r#"AppConfig {
          name: "Foobar",
          version: 1.5,
          active: true,
          features: ["auth", "logging"],
          metadata: Some(Object {theme: String("dark")}),
          error_code: None
        }"#};

        let expected_json = indoc! {r#"{
          "name": "Foobar",
          "version": 1.5,
          "active": true,
          "features": [
            "auth",
            "logging"
          ],
          "metadata": [
            {
              "theme": [
                "dark"
              ]
            }
          ],
          "error_code": null
        }"#};

        let result = parse_debug_to_json(debug_str);
        let actual_value: Value = serde_json::from_str(&result).expect("result should be valid JSON");
        let expected_value: Value =
            serde_json::from_str(expected_json).expect("expected JSON should be valid");

        assert_eq!(actual_value, expected_value);
    }

    #[test]
    fn test_invalid_parse() {
        let debug_str = "Just a random string { not matching standard Debug syntax }";

        let result = parse_debug_to_json(debug_str);

        assert!(result.starts_with("Error parsing debug output:"));
    }

    #[test]
    fn test_handling_json_like_content() {
        let debug_str = indoc! {r#"Interrupted {
          tool_input: Object {"command": String("echo hi"), "summary": String("Say hi")}
        }"#};

        let result = parse_debug_to_json(debug_str);
        let parsed: Value = serde_json::from_str(&result).expect("result should be valid JSON");

        assert_eq!(parsed["tool_input"]["command"][0], "echo hi");
        assert_eq!(parsed["tool_input"]["summary"][0], "Say hi");
    }
}
