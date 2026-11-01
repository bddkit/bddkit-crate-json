//! JSON path reader and matcher for bddkit.
//!
//! [`matcher::contains`] is deliberately loose and [`matcher::equals`] is strict; see the README.

use serde_json::Value;

pub mod matcher;
pub mod path;

pub use matcher::{Mismatch, UnexpectedMatch};

/// A path that cannot be read: malformed (`list[x]`) or absent from the document.
#[derive(Debug, Clone, PartialEq)]
pub struct PathError(String);

impl PathError {
    pub(crate) fn new(message: String) -> Self {
        Self(message)
    }
}

impl std::fmt::Display for PathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for PathError {}

/// The expected JSON of a step is not JSON.
#[derive(Debug)]
pub struct InvalidJson(serde_json::Error);

impl std::fmt::Display for InvalidJson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "expected JSON is invalid: {}", self.0)
    }
}

impl std::error::Error for InvalidJson {}

/// Parses the expected JSON of a step.
pub fn parse_expected(source: &str) -> Result<Value, InvalidJson> {
    serde_json::from_str(source).map_err(InvalidJson)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn parse_expected_reads_json() {
        assert_eq!(parse_expected(r#"{"a": 1}"#).unwrap(), json!({"a": 1}));
    }

    #[test]
    fn parse_expected_names_the_problem() {
        let e = parse_expected("{nope").unwrap_err().to_string();
        assert!(e.starts_with("expected JSON is invalid: "), "{e}");
    }

    /// Invariant: the same pair passes `contains` and fails `equals`.
    #[test]
    fn contains_is_loose_and_equals_is_strict_on_the_same_input() {
        let actual = json!({"id": 1, "extra": true, "items": [3, 1, 2]});
        let expected = json!({"id": 1, "items": [1]});
        assert!(matcher::contains(&actual, &expected).is_ok());
        assert!(matcher::equals(&actual, &expected).is_err());
    }

    #[test]
    fn not_contains_passes_when_absent_and_fails_when_present() {
        let actual = json!({"items": [{"id": 1}]});
        assert!(matcher::not_contains(&actual, &json!({"items": [{"id": 9}]})).is_ok());
        let e = matcher::not_contains(&actual, &json!({"items": [{"id": 1}]})).unwrap_err();
        let text = e.describe("body");
        assert!(
            text.starts_with("    expected: body to NOT contain JSON matching:\n"),
            "{text}"
        );
        assert!(text.ends_with("\n    actual:   it does"), "{text}");
    }

    #[test]
    fn path_errors_display_their_message() {
        let e = path::read(&json!({}), "a").unwrap_err();
        assert_eq!(e.to_string(), "path root.a does not exist");
    }
}
