// Modules -------------------------------------------------------------------------------------------
use crate::tokenize::{tokenize_measurement_calc, MeasurementBroadTokenKind, MeasurementNarrowTokenKind};

use std::sync::LazyLock;

use enum_map::{EnumMap, enum_map};
use rbx_types::{UDim, Variant};
use regex::Regex;
// ---------------------------------------------------------------------------------------------------


// Globals -------------------------------------------------------------------------------------------
static OPERATIONS: LazyLock<EnumMap<MeasurementNarrowTokenKind, fn(f32, f32) -> f32>> = LazyLock::new(|| {
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

static NON_AMBIGUOUS_CALC_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\d*\.?\d+(px|%)").unwrap()
});
// ---------------------------------------------------------------------------------------------------


// Private Functions ---------------------------------------------------------------------------------
fn will_divide_by_zero(a: f32, b: f32) -> bool {
    if a != 0.0 && b != 0.0 { return false }
    return true
}

fn add_f32(a: f32, b: f32) -> f32 { a + b }
fn subtract_f32(a: f32, b: f32) -> f32 { a - b }
fn multiply_f32(a: f32, b: f32) -> f32 { a * b }
fn divide_f32(a: f32, b: f32) -> f32 {
    if will_divide_by_zero(a, b) { return a }
    a / b
}
fn power_f32(a: f32, b: f32) -> f32 { a.powf(b) }
fn modulo_f32(a: f32, b: f32) -> f32 { a % b }

fn operator_is_add_or_subtract(current_operator: MeasurementNarrowTokenKind) -> bool {
    if (
        matches!(current_operator, MeasurementNarrowTokenKind::OperatorAdd) ||
        matches!(current_operator, MeasurementNarrowTokenKind::OperatorSubtract)
    ) {
        return true
    }
    return false
}
// ---------------------------------------------------------------------------------------------------


pub fn parse_measurement_calc(source: &str) -> Variant {
    let (mut scale, mut offset): (f32, f32) = (0.0, 0.0);

    let tokens = tokenize_measurement_calc(&source);
    let mut tokens_iter = tokens.iter();

    let mut current_operator: MeasurementNarrowTokenKind = MeasurementNarrowTokenKind::OperatorAdd;
    let mut previous_number_type: MeasurementNarrowTokenKind = MeasurementNarrowTokenKind::NumberAmbiguous;
  
    while let Some(token) = tokens_iter.next() {
        if let Some(token_kind) = token.kind {
            let (token_broad_kind, token_narrow_kind) = (token_kind.0, token_kind.1);

            match token_broad_kind {
                MeasurementBroadTokenKind::Operator => {
                    match (current_operator, token_narrow_kind) {
                        (MeasurementNarrowTokenKind::OperatorSubtract, MeasurementNarrowTokenKind::OperatorAdd) => {
                            current_operator = MeasurementNarrowTokenKind::OperatorSubtract
                        },

                        (MeasurementNarrowTokenKind::OperatorSubtract, MeasurementNarrowTokenKind::OperatorSubtract) => {
                            current_operator = MeasurementNarrowTokenKind::OperatorAdd
                        },

                        _ => current_operator = token_narrow_kind
                    }
                },

                MeasurementBroadTokenKind::Number => {
                    let mut token_value = token.value.to_owned();

                    match token_narrow_kind {
                        MeasurementNarrowTokenKind::NumberAmbiguous => {
                            let parsed_token_value = token_value.parse::<f32>().unwrap();

                            let operation_fn = OPERATIONS[current_operator];

                            // Scales can only be added or subtracted from other scales.
                            if operator_is_add_or_subtract(current_operator) {
                                scale = operation_fn(scale, parsed_token_value);
                            } else {
                                if matches!(previous_number_type, MeasurementNarrowTokenKind::NumberOffset) {
                                    offset = operation_fn(offset, parsed_token_value);
                                } else {
                                    scale = operation_fn(scale, parsed_token_value);
                                }
                            }

                            // For the purposes of previous_number_type,
                            // NumberAmbigious is treated the same as NumberScale.
                            previous_number_type = MeasurementNarrowTokenKind::NumberScale;
                        },

                        MeasurementNarrowTokenKind::NumberOffset => {
                            // Removes `px` from end and converts to i32.
                            token_value.truncate(token_value.len() - 2);
                            let parsed_token_value = token_value.parse::<f32>().unwrap();

                            let operation_fn = OPERATIONS[current_operator];

                            // Offsets can only be added or subtracted from other offsets.
                            if operator_is_add_or_subtract(current_operator) {
                                offset = operation_fn(offset, parsed_token_value);
                            } else {
                                if matches!(previous_number_type, MeasurementNarrowTokenKind::NumberOffset) {
                                    offset = operation_fn(offset, parsed_token_value);
                                } else {
                                    scale = operation_fn(scale, parsed_token_value);
                                }
                            }

                            previous_number_type = MeasurementNarrowTokenKind::NumberOffset;
                        },

                        MeasurementNarrowTokenKind::NumberScale => {
                            // Removes `%` from end, converts to f32 and divides by 100.
                            token_value.truncate(token_value.len() - 1);
                            let parsed_token_value = token_value.parse::<f32>().unwrap() / 100.0;
                            
                            let operation_fn = OPERATIONS[current_operator];

                            // Scales can only be added or subtracted from other scales.
                            if operator_is_add_or_subtract(current_operator) {
                                scale = operation_fn(scale, parsed_token_value);
                            } else {
                                if matches!(previous_number_type, MeasurementNarrowTokenKind::NumberOffset) {
                                    offset = operation_fn(offset, parsed_token_value);
                                } else {
                                    scale = operation_fn(scale, parsed_token_value);
                                }
                            }

                            previous_number_type = MeasurementNarrowTokenKind::NumberScale;
                        },

                        // This case is never hit as `token_narrow_kind` should
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

    match NON_AMBIGUOUS_CALC_REGEX.is_match(source) {
        true => Variant::UDim(UDim::new(scale, offset as i32)),
        false => Variant::Float32(scale)
    }
}