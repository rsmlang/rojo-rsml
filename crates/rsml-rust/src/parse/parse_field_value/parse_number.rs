// Modules -------------------------------------------------------------------------------------------
use rbx_types::Variant;
// ---------------------------------------------------------------------------------------------------


pub fn parse_number(capture: &str) -> Variant {
    Variant::Float32(match capture.parse::<f32>() {
        Ok(parsed) => parsed,
        Err(_) => 0.0
    })
}