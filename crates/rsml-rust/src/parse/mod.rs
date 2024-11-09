// Modules -------------------------------------------------------------------------------------------
pub use super::arena::Arena;

mod parse_rsml;
pub use parse_rsml::{parse_rsml, TokenTreeNode};

mod parse_field_value;
pub use parse_field_value::parse_field_value;

use std::sync::LazyLock;

use regex::Regex;
// ---------------------------------------------------------------------------------------------------


// Globals -------------------------------------------------------------------------------------------
pub static VARIABLE_TOKEN_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\$(.+)").unwrap());
// ---------------------------------------------------------------------------------------------------

