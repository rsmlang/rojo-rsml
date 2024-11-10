// Modules -------------------------------------------------------------------------------------------
use super::{tokenize, Token, TokenConfig};

use std::sync::LazyLock;

use regex::Regex;
// ---------------------------------------------------------------------------------------------------


// Structs -------------------------------------------------------------------------------------------
#[derive(Debug, Copy, Clone)]
pub enum RsmlTokenKind {
    Comment,

    Selector,

    FieldDeclaration,
    FieldValue,

    PriorityDeclaration,
    PriorityValue,

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

static PRIORITY_DEC_NEXT_TOKENS: LazyLock<[TokenConfig<'static, RsmlTokenKind>; 1]> = LazyLock::new(|| [TokenConfig {
    kind: RsmlTokenKind::PriorityValue,
    pattern: Regex::new(r"^ *(\d+)[;,]").unwrap(),
    next: Some(FIELD_DEC_EQUALS_NEXT_TOKENS.as_slice())
}]);

static TOKENS_CONFIG: LazyLock<[TokenConfig<'static, RsmlTokenKind>; 6]> =  LazyLock::new(|| [
    TokenConfig {
        kind: RsmlTokenKind::Comment,
        pattern: Regex::new("^[\n\t ]*(\\-\\-\\[\\[(.|\n)*\\]\\])").unwrap(),
        next: None
    },
    TokenConfig {
        kind: RsmlTokenKind::Comment,
        pattern: Regex::new("^[\n\t ]*(\\-\\-[^\n]*\n?)").unwrap(),
        next: None
    },

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
        kind: RsmlTokenKind::PriorityDeclaration,
        pattern: Regex::new("^[\n\t ]*@priority").unwrap(),
        next: Some(PRIORITY_DEC_NEXT_TOKENS.as_slice())
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