// Modules -------------------------------------------------------------------------------------------
use std::{collections::HashMap, sync::LazyLock};
use rbx_types::{Color3, UDim, UDim2, Variant, Vector2};
use enum_map::{enum_map, EnumMap};
use serde_json;
use colors_transform::{Rgb, Color};

use crate::{tokenize_field_value, tokenize_measurement_sum, FieldTokenKind, MeasurementBroadTokenKind, MeasurementNarrowTokenKind, RsmlTokenKind, Token, VARIABLE_TOKEN_REGEX};
// ---------------------------------------------------------------------------------------------------


// Structs -------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct TokenTreeNode<'a> {
    pub properties: HashMap<&'a str, Variant>,
    pub variables: HashMap<&'a str, Variant>,
    pub rules: HashMap<&'a str, usize>,
    pub parent: usize
}

impl TokenTreeNode<'_> {
    fn new(parent: usize) -> Self {
        Self {
            properties: HashMap::new(),
            variables: HashMap::new(),
            rules: HashMap::new(),
            parent
        }
    }
}


#[derive(Debug)]
pub struct Arena<T> {
    data: Vec<T>,
    size: usize,
}

impl<'a, T> Arena<T> {
    pub fn new() -> Self {

        Self {
            data: vec![],
            size: 0,
        }
    }

    pub fn push(&mut self, value: T) -> usize {
        let this_idx = self.size;
        self.data.push(value);
        self.size += 1;
        this_idx
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.data.get_mut(index)
    }
}
// ---------------------------------------------------------------------------------------------------


// Globals -------------------------------------------------------------------------------------------
static TUPPLE_ANNOTATION_VARIANTS: LazyLock<HashMap<String, fn(&str) -> Variant>> = LazyLock::new(|| {
    let mut hashmap: HashMap<String, fn(&str) -> Variant> = HashMap::new();
    hashmap.insert(String::from("udim2"), parse_tuple_as_udim2);
    hashmap.insert(String::from("udim"), parse_tuple_as_udim);
    hashmap.insert(String::from("vec2"), parse_tuple_as_vec2);
    hashmap.insert(String::from("rgb"), parse_tuple_as_rgb);
    hashmap.insert(String::from("color3"), parse_tuple_as_color3);

    hashmap
});

static OPERATIONS_F32: LazyLock<EnumMap<MeasurementNarrowTokenKind, fn(f32, f32) -> f32>> = LazyLock::new(|| {
    enum_map! {
        MeasurementNarrowTokenKind::OperatorAdd => add_f32,
        MeasurementNarrowTokenKind::OperatorSubtract => subtract_f32,
        MeasurementNarrowTokenKind::OperatorMultiply => multiply_f32,
        MeasurementNarrowTokenKind::OperatorDivide => divide_f32,
        MeasurementNarrowTokenKind::OperatorModulo => modulo_f32,
        MeasurementNarrowTokenKind::OperatorPower => power_f32,
        _ => multiply_f32
    }
});

static OPERATIONS_I32: LazyLock<EnumMap<MeasurementNarrowTokenKind, fn(i32, i32) -> i32>> = LazyLock::new(|| {
    enum_map! {
        MeasurementNarrowTokenKind::OperatorAdd => add_i32,
        MeasurementNarrowTokenKind::OperatorSubtract => subtract_i32,
        MeasurementNarrowTokenKind::OperatorMultiply => multiply_i32,
        MeasurementNarrowTokenKind::OperatorDivide => divide_i32,
        MeasurementNarrowTokenKind::OperatorModulo => modulo_i32,
        MeasurementNarrowTokenKind::OperatorPower => power_i32,
        _ => multiply_i32
    }
});

static TAILWIND_COLORS: LazyLock<HashMap<String, String>> = LazyLock::new(|| {
    serde_json::from_slice(include_bytes!("tailwind.json"))
        .expect("Could not read tailwind.json file.")
});
// ---------------------------------------------------------------------------------------------------


// Private Functions ---------------------------------------------------------------------------------
fn add_f32(a: f32, b: f32) -> f32 { a + b }
fn subtract_f32(a: f32, b: f32) -> f32 { a - b }
fn multiply_f32(a: f32, b: f32) -> f32 { a * b }
fn divide_f32(a: f32, b: f32) -> f32 { a / b }
fn power_f32(a: f32, b: f32) -> f32 { a.powf(b) }
fn modulo_f32(a: f32, b: f32) -> f32 { a % b }

fn add_i32(a: i32, b: i32) -> i32 { a + b }
fn subtract_i32(a: i32, b: i32) -> i32 { a - b }
fn multiply_i32(a: i32, b: i32) -> i32 { a * b }
fn divide_i32(a: i32, b: i32) -> i32 { a / b }
fn power_i32(a: i32, b: i32) -> i32 { a.pow(b.try_into().unwrap()) }
fn modulo_i32(a: i32, b: i32) -> i32 { a % b }


pub fn parse_measurement_sum(source: &str) -> (f32, i32) {
    let (mut scale, mut offset): (f32, i32) = (0.0, 0);

    let tokens = tokenize_measurement_sum(&source);
    let mut tokens_iter = tokens.iter();

    let mut current_operator: MeasurementNarrowTokenKind = MeasurementNarrowTokenKind::OperatorAdd;
    let mut previous_number_type: Option<MeasurementNarrowTokenKind> = None;
  
    while let Some(token) = tokens_iter.next() {
        if let Some(token_kind) = token.kind {
            let (token_broad_kind, token_narrow_kind) = (token_kind.0, token_kind.1);

            match token_broad_kind {
                MeasurementBroadTokenKind::Operator => {
                    // If the current operator and this tokens operator are both either `-` or `+`
                    // then the current operator should be changed to be `-`.
                    // In other cases the current operator should be replaced with this tokens operator.
                    match (current_operator, token_narrow_kind) {
                        (MeasurementNarrowTokenKind::OperatorAdd, MeasurementNarrowTokenKind::OperatorSubtract) => {
                            current_operator = MeasurementNarrowTokenKind::OperatorSubtract
                        },

                        (MeasurementNarrowTokenKind::OperatorSubtract, MeasurementNarrowTokenKind::OperatorAdd) => {
                            current_operator = MeasurementNarrowTokenKind::OperatorSubtract
                        },

                        _ => current_operator = token_narrow_kind
                    }
                },

                MeasurementBroadTokenKind::Number => {
                    let mut token_value = token.value.to_owned();

                    match token_narrow_kind {
                        MeasurementNarrowTokenKind::NumberOffset => {
                            // Removes `px` from end and converts to i32.
                            token_value.truncate(token_value.len() - 2);
                            let parsed_token_value = token_value.parse::<i32>().unwrap();

                            let operation_fn = OPERATIONS_I32[current_operator];

                            offset = operation_fn(offset, parsed_token_value);
                        },

                        MeasurementNarrowTokenKind::NumberScale => {
                            // Removes `%` from end, converts to f32 and divides by 100.
                            token_value.truncate(token_value.len() - 1);
                            let parsed_token_value = token_value.parse::<f32>().unwrap() / 100.0;
                            

                            let operation_fn = OPERATIONS_F32[current_operator];

                            scale = operation_fn(scale, parsed_token_value);
                        },

                        MeasurementNarrowTokenKind::NumberAmbiguous => {
                            let parsed_token_value = token_value.parse::<f32>().unwrap();
                            

                            let operation_fn = OPERATIONS_F32[current_operator];

                            scale = operation_fn(scale, parsed_token_value);
                        },

                        // This case is never hit as `token_kind.1` should
                        // only ever be a narrow number enum variant.
                        _ => ()
                    }

                    // current_operator needs to be reset as operations which appeared before this
                    // number should not effect numbers which will come after this number.
                    current_operator = MeasurementNarrowTokenKind::OperatorAdd;
                },

                _ => ()
            }
        }
    }

    (scale, offset)
}

fn parse_tuple_as_udim2(tuple: &str) -> Variant {
    let components: Vec<&str> = tuple.split(",").collect();

    let component_x = match components.get(0) {
        Some(component_x) => {
            let (scale, offset) = parse_measurement_sum(component_x);
            UDim::new(scale, offset)
        },
        None => UDim::new(0.0, 0)
    };

    let component_y = match components.get(1) {
        Some(component_y) => {
            let (scale, offset) = parse_measurement_sum(component_y);
            UDim::new(scale, offset)
        },
        None => UDim::new(0.0, 0)
    };

    Variant::UDim2(UDim2::new(component_x, component_y))
}

fn parse_tuple_as_udim(tuple: &str) -> Variant {
    let (scale, offset) = parse_measurement_sum(tuple);
    Variant::UDim(UDim::new(scale, offset))
}

fn parse_tuple_as_vec2(tuple: &str) -> Variant {
    let components: Vec<&str> = tuple.split(",").collect();

    let component_x = match components.get(0) {
        Some(component_x) => {
            let (scale, _offset) = parse_measurement_sum(component_x);
            scale
        },
        None => 0.0
    };

    let component_y = match components.get(1) {
        Some(component_y) => {
            let (scale, _offset) = parse_measurement_sum(component_y);
            scale
        },
        None => 0.0
    };

    Variant::Vector2(Vector2::new(component_x, component_y))
}


fn parse_color3(tuple: &str) -> (f32, f32, f32) {
    let components: Vec<&str> = tuple.split(",").collect();

    let component_r = match components.get(0) {
        Some(component) => match component.trim().parse::<f32>() {
            Ok(parsed_componenent) => parsed_componenent,
            Err(_) => 0.0
        },
        None => 0.0
    };

    let component_g = match components.get(1) {
        Some(component) => match component.trim().parse::<f32>() {
            Ok(parsed_componenent) => parsed_componenent,
            Err(_) => 0.0
        },
        None => 0.0
    };

    let component_b = match components.get(2) {
        Some(component) => match component.trim().parse::<f32>() {
            Ok(parsed_componenent) => parsed_componenent,
            Err(_) => 0.0
        },
        None => 0.0
    };

    (component_r, component_g, component_b)
}

fn parse_tuple_as_color3(tuple: &str) -> Variant {
    let (r, g, b) = parse_color3(tuple);

    Variant::Color3(Color3::new(r, g, b))
}

fn parse_tuple_as_rgb(tuple: &str) -> Variant {
    let (r, g, b) = parse_color3(tuple);
    Variant::Color3(Color3::new(r / 255.0, g / 255.0, b / 255.0))
}


fn parse_hex(hex: &str) -> (f32, f32, f32) {
    let rgb = match Rgb::from_hex_str(&hex) {
        Ok(ok_rgb) => ok_rgb,
        Err(_) => Rgb::from(255.0, 0.0, 249.0)
    };
    
    return (rgb.get_red() / 255.0, rgb.get_green() / 255.0, rgb.get_blue() / 255.0)
}


fn parse_field_value(field_value: &str) -> Variant {
    let (kind, captures) = match tokenize_field_value(&field_value) {
        None => return Variant::String(field_value.to_owned()),
        Some(parsed) => parsed
    };

    match kind {
        FieldTokenKind::Tuple => {
            match TUPPLE_ANNOTATION_VARIANTS.get(captures.get(1).unwrap().as_str()) {
                None => Variant::String(captures.get(0).unwrap().as_str().to_owned()),
                Some(tuple_fn) => return tuple_fn(captures.get(2).unwrap().as_str())
            }
        },

        FieldTokenKind::String => {
            Variant::String(captures.get(1).unwrap().as_str().to_owned())
        },

        FieldTokenKind::Number => {
            Variant::Float32(match captures.get(0).unwrap().as_str().parse::<f32>() {
                Ok(parsed) => parsed,
                Err(_) => 0.0
            })
        }

        FieldTokenKind::Boolean => {
            Variant::Bool(if captures.get(0).unwrap().as_str() == "true" { true } else { false })
        },

        FieldTokenKind::ColorTailwind => {
            let tailwind_color = captures.get(0).unwrap().as_str();

            let hex_code = TAILWIND_COLORS.get(tailwind_color).unwrap();
            let (r, g, b) = parse_hex(hex_code);

            Variant::Color3(Color3::new(r, g, b))
        },

        FieldTokenKind::ColorHex => {
            let (r, g, b) = parse_hex(captures.get(0).unwrap().as_str());
            Variant::Color3(Color3::new(r, g, b))
        }
        
        //_ => Variant::String(captures.get(0).unwrap().as_str().to_owned())
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
                        let field_value = parse_field_value(&next_token.value);
    
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
    
                RsmlTokenKind::BracketClosed => {
                    current_idx = arena.get(current_idx).unwrap().parent;
                }
    
                _ => ()
            }
        }
    }

    arena
}