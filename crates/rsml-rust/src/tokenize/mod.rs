// Modules -------------------------------------------------------------------------------------------
mod tokenize_rsml;
pub use tokenize_rsml::{tokenize_rsml, RsmlTokenKind};

mod tokenize_data_type;
pub use tokenize_data_type::{tokenize_data_type, FieldTokenKind, tokenize_measurement_calc, MeasurementBroadTokenKind, MeasurementNarrowTokenKind};

use std::sync::LazyLock;

use regex::Regex;
// ---------------------------------------------------------------------------------------------------


// Structs -------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct TokenConfig<'a, Kind> {
    kind: Option<Kind>,
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
static DEFAULT_TOKEN_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new("^[ \n\t]*[^ \n\t]*").unwrap());
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

            tokens.push(Token::<T> { kind: token.kind, value: found.as_str().to_owned() });

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