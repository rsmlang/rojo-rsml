use std::{collections::HashMap, sync::LazyLock};

use rbx_types::Variant;

use super::parse_hex::parse_hex;

// Globals -------------------------------------------------------------------------------------------
static TAILWIND_COLORS: LazyLock<HashMap<String, String>> = LazyLock::new(|| {
    serde_json::from_slice(include_bytes!("../../tailwind.json"))
        .expect("Could not read tailwind.json file.")
});
// ---------------------------------------------------------------------------------------------------

pub fn parse_tailwind(capture: &str) -> Variant {
    parse_hex(TAILWIND_COLORS.get(capture).unwrap())
}