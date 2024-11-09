// Modules -------------------------------------------------------------------------------------------
use crate::tokenize::{tokenize_measurement_calc, MeasurementBroadTokenKind, MeasurementNarrowTokenKind};

use std::sync::LazyLock;

use enum_map::{EnumMap, enum_map};
// ---------------------------------------------------------------------------------------------------


// Globals -------------------------------------------------------------------------------------------
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
// ---------------------------------------------------------------------------------------------------


pub fn parse_measurement_calc(source: &str) -> (f32, i32) {
    let (mut scale, mut offset): (f32, i32) = (0.0, 0);

    let tokens = tokenize_measurement_calc(&source);
    let mut tokens_iter = tokens.iter();

    let mut current_operator: MeasurementNarrowTokenKind = MeasurementNarrowTokenKind::OperatorAdd;
    //let mut previous_number_type: Option<MeasurementNarrowTokenKind> = None;
  
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