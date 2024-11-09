// Modules -------------------------------------------------------------------------------------------
mod parse_measurement_calc;
pub use parse_measurement_calc::parse_measurement_calc;

mod parse_hex;
use parse_hex::parse_hex;

mod parse_tailwind;
use parse_tailwind::parse_tailwind;

mod parse_tuple;
use parse_tuple::parse_tuple;

mod parse_bool;
use parse_bool::parse_bool;

mod parse_number;
use parse_number::parse_number;

mod parse_string;
use parse_string::parse_string;
use regex::Captures;

use crate::tokenize::{tokenize_data_type, FieldTokenKind};

use rbx_types::Variant;
// ---------------------------------------------------------------------------------------------------


// Private Functions ---------------------------------------------------------------------------------
fn get_first_capture_as_str(captures: Captures) -> &str {
    captures.get(0).unwrap().as_str()
}
// ---------------------------------------------------------------------------------------------------


pub fn parse_data_type(field_value: &str) -> Variant {
    let (kind, captures) = match tokenize_data_type(&field_value) {
        None => return Variant::String(field_value.to_owned()),
        Some(parsed) => parsed
    };

    match kind {
        FieldTokenKind::Tuple => parse_tuple(captures),
        FieldTokenKind::MeasurementCalc => parse_measurement_calc(get_first_capture_as_str(captures)),
        FieldTokenKind::String => parse_string(get_first_capture_as_str(captures)),
        FieldTokenKind::Number => parse_number(get_first_capture_as_str(captures)),
        FieldTokenKind::Boolean => parse_bool(get_first_capture_as_str(captures)),
        FieldTokenKind::ColorTailwind => parse_tailwind(get_first_capture_as_str(captures)),
        FieldTokenKind::ColorHex => parse_hex(get_first_capture_as_str(captures)),
        //_ => Variant::String(get_first_capture_as_str(captures).to_owned())
    }
}