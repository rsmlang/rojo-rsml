// Modules -------------------------------------------------------------------------------------------
use super::{tokenize, Token, TokenConfig};

use std::sync::LazyLock;

use regex::Regex;
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
// ---------------------------------------------------------------------------------------------------


pub fn tokenize_rsml(source: &str) -> Vec<Token<RsmlTokenKind>> {
    tokenize(source, TOKENS_CONFIG.as_slice())
}