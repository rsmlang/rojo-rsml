// Modules -------------------------------------------------------------------------------------------
use crate::tokenize::{RsmlTokenKind, Token};

use super::{parse_data_type, Arena};

use std::{collections::HashMap, sync::LazyLock};

use rbx_types::Variant;
use regex::Regex;
// ---------------------------------------------------------------------------------------------------


// Globals -------------------------------------------------------------------------------------------
pub static ROOT_SELECTOR_NAME: LazyLock<String> = LazyLock::new(|| String::from("@:ROOT"));

pub static VARIABLE_TOKEN_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\$(.+)").unwrap());
// ---------------------------------------------------------------------------------------------------


// Structs -------------------------------------------------------------------------------------------
#[derive(Debug)]
pub enum TokenTreeKind {
    Sheet,
    Rule,
    Macro
}

#[derive(Debug)]
pub struct TokenTreeNodeRules<'a>(pub HashMap<&'a str, Vec<usize>>);

impl<'a> TokenTreeNodeRules<'a> {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn insert(&mut self, selector: &'a str, node_idx: usize) {
        let rules = self.0.entry(&selector).or_insert(vec![]);

        rules.push(node_idx);
    }
}

#[derive(Debug)]
pub struct TokenTreeNode<'a> { 
    pub kind: TokenTreeKind,
    pub properties: HashMap<&'a str, Variant>,
    pub variables: HashMap<&'a str, Variant>,
    pub rules: TokenTreeNodeRules<'a>,
    pub priority: Option<i32>,
    pub parent: usize
}

impl<'a> TokenTreeNode<'a> {
    fn new(kind: TokenTreeKind, parent: usize) -> TokenTreeNode<'a> {
        TokenTreeNode {
            kind,
            properties: HashMap::new(),
            variables: HashMap::new(),
            rules: TokenTreeNodeRules::new(),
            priority: None,
            parent
        }
    }
}
// ---------------------------------------------------------------------------------------------------


pub fn parse_rsml<'a>(tokens: &'a [Token<RsmlTokenKind>]) -> Arena<TokenTreeNode> {
    let data: TokenTreeNode = TokenTreeNode::new(TokenTreeKind::Sheet,0);

    let mut arena = Arena::<TokenTreeNode>::new();

    let mut current_idx = arena.push(data);

    let mut tokens_iter = tokens.iter();

    while let Some(token) = tokens_iter.next() {
        if let Some(token_kind) = token.kind {
            match token_kind {
                RsmlTokenKind::Selector => {
                    let selector: &str = &token.value;
    
        
                    let new_idx = arena.push(
                        TokenTreeNode::new(TokenTreeKind::Rule,current_idx)
                    );
                    arena.get_mut(current_idx).unwrap().rules.insert(selector, new_idx);

                    current_idx = new_idx
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

                RsmlTokenKind::MacroDeclaration => {
                    //let current_data = arena.get_mut(current_idx).unwrap();

                },

                RsmlTokenKind::PriorityDeclaration => {
                    if let Some(next_token) = tokens_iter.next() {
                        let priority_level = match &next_token.value.parse::<i32>() {
                            Ok(parsed) => *parsed,
                            Err(_) => 0
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