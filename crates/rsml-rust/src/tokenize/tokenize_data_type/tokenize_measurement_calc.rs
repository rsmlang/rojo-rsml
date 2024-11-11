// Modules -------------------------------------------------------------------------------------------
use crate::tokenize::{tokenize, Token, TokenConfig};

use std::sync::LazyLock;

use enum_map::Enum;
use regex::Regex;
// ---------------------------------------------------------------------------------------------------


// Structs -------------------------------------------------------------------------------------------
#[derive(Debug, Copy, Clone, Enum)]
pub enum MeasurementNarrowTokenKind {
    NumberScale,
    NumberOffset,
    NumberAmbiguous,

    OperatorAdd,
    OperatorSubtract,
    OperatorMultiply,
    OperatorDivide,
    OperatorModulo,
    OperatorPower,

    BracketOpen,
    BracketClosed
}

#[derive(Debug, Copy, Clone)]
pub enum MeasurementBroadTokenKind {
    Number,
    Operator,
    Bracket
}

pub type MeasurementKind = (MeasurementBroadTokenKind, MeasurementNarrowTokenKind);
// ---------------------------------------------------------------------------------------------------


// Globals -------------------------------------------------------------------------------------------
static MEASUREMENT_TOKEN_CONFIG: LazyLock<[TokenConfig<'static, MeasurementKind>; 11]> = LazyLock::new(|| [
    TokenConfig {
        kind: Some((MeasurementBroadTokenKind::Number, MeasurementNarrowTokenKind::NumberScale)),
        pattern: Regex::new(r"^[ \n\t]*(\d*\.?\d+%)").unwrap(),
        next: None
    },

    TokenConfig {
        kind: Some((MeasurementBroadTokenKind::Number, MeasurementNarrowTokenKind::NumberOffset)),
        pattern: Regex::new(r"^[ \n\t]*(\d*\.?\d+px)").unwrap(),
        next: None
    },

    TokenConfig {
        kind: Some((MeasurementBroadTokenKind::Number, MeasurementNarrowTokenKind::NumberAmbiguous)),
        pattern: Regex::new(r"^[ \n\t]*(\d*\.?\d+)").unwrap(),
        next: None
    },

    TokenConfig {
        kind: Some((MeasurementBroadTokenKind::Operator, MeasurementNarrowTokenKind::OperatorAdd)),
        pattern: Regex::new(r"^[ \n\t]*(\+)").unwrap(),
        next: None
    },

    TokenConfig {
        kind: Some((MeasurementBroadTokenKind::Operator, MeasurementNarrowTokenKind::OperatorSubtract)),
        pattern: Regex::new(r"^[ \n\t]*(\-)").unwrap(),
        next: None
    },

    TokenConfig {
        kind: Some((MeasurementBroadTokenKind::Operator, MeasurementNarrowTokenKind::OperatorMultiply)),
        pattern: Regex::new(r"^[ \n\t]*(\*)").unwrap(),
        next: None
    },

    TokenConfig {
        kind: Some((MeasurementBroadTokenKind::Operator, MeasurementNarrowTokenKind::OperatorDivide)),
        pattern: Regex::new(r"^[ \n\t]*(/)").unwrap(),
        next: None
    },

    TokenConfig {
        kind: Some((MeasurementBroadTokenKind::Operator, MeasurementNarrowTokenKind::OperatorModulo)),
        pattern: Regex::new(r"^[ \n\t]*(%)").unwrap(),
        next: None
    },

    TokenConfig {
        kind: Some((MeasurementBroadTokenKind::Operator, MeasurementNarrowTokenKind::OperatorPower)),
        pattern: Regex::new(r"^[ \n\t]*(\^)").unwrap(),
        next: None
    },

    TokenConfig {
        kind: Some((MeasurementBroadTokenKind::Bracket, MeasurementNarrowTokenKind::BracketOpen)),
        pattern: Regex::new(r"^[ \n\t]*(\()").unwrap(),
        next: None
    },

    TokenConfig {
        kind: Some((MeasurementBroadTokenKind::Bracket, MeasurementNarrowTokenKind::BracketClosed)),
        pattern: Regex::new(r"^[ \n\t]*(\))").unwrap(),
        next: None
    },
]);
// ---------------------------------------------------------------------------------------------------


pub fn tokenize_measurement_calc(source: &str) -> Vec<Token<MeasurementKind>> {
    tokenize(source, MEASUREMENT_TOKEN_CONFIG.as_slice())
}
 