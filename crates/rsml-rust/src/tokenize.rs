use enum_map::Enum;
// Modules -------------------------------------------------------------------------------------------
use regex::{Regex, Captures};
use std::sync::LazyLock;
// ---------------------------------------------------------------------------------------------------


// Structs -------------------------------------------------------------------------------------------
#[derive(Debug, Copy, Clone)]
pub enum RsmlTokenKind {
    Selector,
    FieldDeclaration,
    FieldValue,
    BracketClosed,
    Default
}

#[derive(Clone, Copy, Debug)]
pub enum FieldTokenKind {
    Tuple,
    String,
    Number,
    Boolean,

    ColorTailwind,
    ColorHex,
}


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

type MeasurementKind = (MeasurementBroadTokenKind, MeasurementNarrowTokenKind);

#[derive(Debug)]
pub struct TokenConfig<'a, Kind> {
    kind: Kind,
    pattern: Regex,
    next: Option<&'a [TokenConfig<'a, Kind>]>,


}

#[derive(Debug)]
pub struct Token<Kind> {
    pub kind: Option<Kind>,
    pub value: String
}
// ---------------------------------------------------------------------------------------------------


// Globals -------------------------------------------------------------------------------------------
static FIELD_DEC_EQUALS_NEXT_TOKENS: LazyLock<[TokenConfig<'static, RsmlTokenKind>; 1]> = LazyLock::new(|| [
    TokenConfig {
        kind: RsmlTokenKind::FieldValue,
        pattern: Regex::new(r"^ *([^\n\t]+)[;,]").unwrap(),
        next: None
    }
]);

static FIELD_DEC_NEXT_TOKENS: LazyLock<[TokenConfig<'static, RsmlTokenKind>; 1]> = LazyLock::new(|| [TokenConfig {
    kind: RsmlTokenKind::Default,
    pattern: Regex::new(r"^ *(=)").unwrap(),
    next: Some(FIELD_DEC_EQUALS_NEXT_TOKENS.as_slice())
}]);


static TOKENS_CONFIG: LazyLock<[TokenConfig<'static, RsmlTokenKind>; 3]> =  LazyLock::new(|| [
    TokenConfig {
        kind: RsmlTokenKind::Selector,
        pattern: Regex::new("^[\n\t ]*([^\n\t ]+)[\n\t ]*\\{").unwrap(),
        next: None
    },

    TokenConfig {
        kind: RsmlTokenKind::FieldDeclaration,
        pattern: Regex::new("^[\n\t ]*([^\n\t ]+) *=").unwrap(),
        next: Some(FIELD_DEC_NEXT_TOKENS.as_slice())
    },

    TokenConfig {
        kind: RsmlTokenKind::BracketClosed,
        pattern: Regex::new(r"^[\n\t ]*(\})").unwrap(),
        next: None
    }
]);

static DEFAULT_TOKEN_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new("^[ \n\t]*[^ \n\t]*").unwrap());

pub static VARIABLE_TOKEN_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\$(.+)").unwrap());


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


static MEASUREMENT_TOKEN_CONFIG: LazyLock<[TokenConfig<'static, MeasurementKind>; 11]> = LazyLock::new(|| [
    TokenConfig {
        kind: (MeasurementBroadTokenKind::Number, MeasurementNarrowTokenKind::NumberScale),
        pattern: Regex::new(r"^[ \n\t]*(\d*\.?\d+%)").unwrap(),
        next: None
    },

    TokenConfig {
        kind: (MeasurementBroadTokenKind::Number, MeasurementNarrowTokenKind::NumberOffset),
        pattern: Regex::new(r"^[ \n\t]*(\d*\.?\d+px)").unwrap(),
        next: None
    },

    TokenConfig {
        kind: (MeasurementBroadTokenKind::Number, MeasurementNarrowTokenKind::NumberAmbiguous),
        pattern: Regex::new(r"^[ \n\t]*(\d*\.?\d+)").unwrap(),
        next: None
    },

    TokenConfig {
        kind: (MeasurementBroadTokenKind::Operator, MeasurementNarrowTokenKind::OperatorAdd),
        pattern: Regex::new(r"^[ \n\t]*(\+)").unwrap(),
        next: None
    },

    TokenConfig {
        kind: (MeasurementBroadTokenKind::Operator, MeasurementNarrowTokenKind::OperatorSubtract),
        pattern: Regex::new(r"^[ \n\t]*(\-)").unwrap(),
        next: None
    },

    TokenConfig {
        kind: (MeasurementBroadTokenKind::Operator, MeasurementNarrowTokenKind::OperatorMultiply),
        pattern: Regex::new(r"^[ \n\t]*(\*)").unwrap(),
        next: None
    },

    TokenConfig {
        kind: (MeasurementBroadTokenKind::Operator, MeasurementNarrowTokenKind::OperatorDivide),
        pattern: Regex::new(r"^[ \n\t]*(/)").unwrap(),
        next: None
    },

    TokenConfig {
        kind: (MeasurementBroadTokenKind::Operator, MeasurementNarrowTokenKind::OperatorModulo),
        pattern: Regex::new(r"^[ \n\t]*(%)").unwrap(),
        next: None
    },

    TokenConfig {
        kind: (MeasurementBroadTokenKind::Operator, MeasurementNarrowTokenKind::OperatorPower),
        pattern: Regex::new(r"^[ \n\t]*(\^)").unwrap(),
        next: None
    },

    TokenConfig {
        kind: (MeasurementBroadTokenKind::Bracket, MeasurementNarrowTokenKind::BracketOpen),
        pattern: Regex::new(r"^[ \n\t]*(\()").unwrap(),
        next: None
    },

    TokenConfig {
        kind: (MeasurementBroadTokenKind::Bracket, MeasurementNarrowTokenKind::BracketClosed),
        pattern: Regex::new(r"^[ \n\t]*(\))").unwrap(),
        next: None
    },
]);
// ---------------------------------------------------------------------------------------------------


// Private Functions ---------------------------------------------------------------------------------
fn tokenize<T>(source: &str, tokens_config: &[TokenConfig<T>]) -> Vec<Token<T>> where T: Copy {
    let mut next_token_configs = tokens_config;
    let mut cursor_idx = 0;

    let mut tokens: Vec<Token<T>> = vec![];

    'outer: while cursor_idx < source.len() {

        for token in next_token_configs {
            let found = match token.pattern.captures(&source[cursor_idx..]) {
                Some(captures) => match captures.get(1) {
                    Some(capture) => capture,
                    None => match captures.get(0) {
                        Some(capture) => capture,
                        None => continue
                    }
                },
                None => continue
            };

            tokens.push(Token::<T> { kind: Some(token.kind), value: found.as_str().to_owned() });

            next_token_configs = match token.next {
                Some(next) => next,
                None => tokens_config
            };

            cursor_idx += found.end();
            continue 'outer;
        }

        // The code below only runs if a match was not found above.
        let found = DEFAULT_TOKEN_REGEX.find(&source[cursor_idx..]);
        tokens.push(Token { kind: None, value: found.unwrap().as_str().to_owned() });
        cursor_idx += found.unwrap().end();

    }

    tokens
}
// ---------------------------------------------------------------------------------------------------


pub fn tokenize_measurement_sum(source: &str) -> Vec<Token<MeasurementKind>> {
   tokenize(source, MEASUREMENT_TOKEN_CONFIG.as_slice())
}


pub fn tokenize_field_value(field_value: &str) -> Option<(FieldTokenKind, Captures)> {
    for token in FIELD_VALUE_TOKEN_CONFIG.iter() {
        if let Some(captures) = token.pattern.captures(field_value) {
            return Some((token.kind, captures))
        }
    }

    None
}

pub fn tokenize_rsml(source: &str) -> Vec<Token<RsmlTokenKind>> {
    tokenize(source, TOKENS_CONFIG.as_slice())
}