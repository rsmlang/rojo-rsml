// Modules -------------------------------------------------------------------------------------------
mod tokenize_measurement_calc;
pub use tokenize_measurement_calc::{tokenize_measurement_calc, MeasurementBroadTokenKind, MeasurementNarrowTokenKind};

use super::TokenConfig;

use std::sync::LazyLock;

use regex::{Captures, Regex};
// ---------------------------------------------------------------------------------------------------


// Structs -------------------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug)]
pub enum FieldTokenKind {
    Tuple,
    String,
    Number,
    Boolean,

    ColorTailwind,
    ColorHex,
}
// ---------------------------------------------------------------------------------------------------


// Globals -------------------------------------------------------------------------------------------
static FIELD_VALUE_TOKEN_CONFIG: LazyLock<[TokenConfig<'static, FieldTokenKind>; 6]> = LazyLock::new(|| [
    TokenConfig {
        kind: FieldTokenKind::Tuple,
        pattern: Regex::new(r"^[\n\t ]*([^ \n\t]*)[ \n\t]*\((.+)\)$").unwrap(),
        next: None
    },

    TokenConfig {
        kind: FieldTokenKind::Boolean,
        pattern: Regex::new(r"^[\n\t ]*(true|false)").unwrap(),
        next: None
    },

    TokenConfig {
        kind: FieldTokenKind::String,
        pattern: Regex::new(r#"^[\n\t ]*"(.+)""#).unwrap(),
        next: None
    },

    TokenConfig {
        kind: FieldTokenKind::Number,
        pattern: Regex::new(r"^[\n\t ]*(\d*\.?\d+)").unwrap(),
        next: None
    },

    TokenConfig {
        kind: FieldTokenKind::ColorTailwind,
        pattern: Regex::new(r"^[\n\t ]*(tw:(slate|gray|zinc|neutral|stone|red|orange|amber|yellow|lime|green|emerald|teal|cyan|sky|blue|indigo|violet|purple|fuchsia|pink|rose)(:(950|900|800|700|600|500|400|300|200|100|50))?)").unwrap(),
        next: None
    },

    TokenConfig {
        kind: FieldTokenKind::ColorHex,
        pattern: Regex::new(r"^[\n\t ]*(#[0-9a-fA-F]+)").unwrap(),
        next: None
    },
]);
// ---------------------------------------------------------------------------------------------------


pub fn tokenize_field_value(field_value: &str) -> Option<(FieldTokenKind, Captures)> {
    for token in FIELD_VALUE_TOKEN_CONFIG.iter() {
        if let Some(captures) = token.pattern.captures(field_value) {
            return Some((token.kind, captures))
        }
    }

    None
}