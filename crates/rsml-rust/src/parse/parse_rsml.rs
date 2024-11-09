// Modules -------------------------------------------------------------------------------------------
use crate::tokenize::{RsmlTokenKind, Token};

use super::{parse_data_type, Arena};

use std::{collections::HashMap, sync::LazyLock};

use rbx_types::Variant;
use regex::Regex;
// ---------------------------------------------------------------------------------------------------


// Globals -------------------------------------------------------------------------------------------
pub static VARIABLE_TOKEN_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\$(.+)").unwrap());
// ---------------------------------------------------------------------------------------------------


// Structs -------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct TokenTreeNode<'a> {
    pub properties: HashMap<&'a str, Variant>,
    pub variables: HashMap<&'a str, Variant>,
    pub rules: HashMap<&'a str, usize>,
    pub priority: Option<f32>,
    pub parent: usize
}

impl TokenTreeNode<'_> {
    fn new(parent: usize) -> Self {
        Self {
            properties: HashMap::new(),
            variables: HashMap::new(),
            rules: HashMap::new(),
            priority: None,
            parent
        }
    }
}
// ---------------------------------------------------------------------------------------------------


pub fn parse_rsml<'a>(tokens: &'a [Token<RsmlTokenKind>]) -> Arena<TokenTreeNode> {
    let data: TokenTreeNode = TokenTreeNode::new(0);

    let mut arena = Arena::<TokenTreeNode>::new();

    let mut current_idx = arena.push(data);

    let mut tokens_iter = tokens.iter();

    while let Some(token) = tokens_iter.next() {
        if let Some(token_kind) = token.kind {
            match token_kind {
                RsmlTokenKind::Selector => {
                    let selector: &str = &token.value;
    
                    current_idx = match arena.get(current_idx).unwrap().rules.get(selector) {
                        Some(new_idx) => *new_idx,
    
                        None => {
                            let new_idx = arena.push(TokenTreeNode::new(current_idx));
                            arena.get_mut(current_idx).unwrap().rules.insert(&token.value, new_idx);
    
                            new_idx
                        },
                    }
                },
    
                RsmlTokenKind::FieldDeclaration => {
                    tokens_iter.next();
                    if let Some(next_token) = tokens_iter.next() {
                        let field_name = &token.value;
                        let field_value = parse_data_type(&next_token.value);
    
                        let current_data = arena.get_mut(current_idx).unwrap();
                        
                        match VARIABLE_TOKEN_REGEX.captures(&field_name) {
                            None => current_data.properties.insert(field_name, field_value),
    
                            Some(captures) => match captures.get(1) {
                                Some(capture) => current_data.variables.insert(capture.as_str(), field_value),
                                None => current_data.properties.insert(field_name, field_value),
                            }
                        };
                    }
                },

                RsmlTokenKind::PriorityDeclaration => {
                    if let Some(next_token) = tokens_iter.next() {
                        let priority_level = match &next_token.value.parse::<f32>() {
                            Ok(parsed) => *parsed,
                            Err(_) => 0.0
                        };
    
                        let current_data = arena.get_mut(current_idx).unwrap();
                        
                        current_data.priority = Some(priority_level)
                    }
                },
    
                RsmlTokenKind::BracketClosed => {
                    current_idx = arena.get(current_idx).unwrap().parent;
                }
    
                _ => ()
            }
        }
    }

    arena
}