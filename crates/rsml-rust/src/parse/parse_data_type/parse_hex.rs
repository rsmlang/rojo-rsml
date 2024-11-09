// Modules -------------------------------------------------------------------------------------------
use colors_transform::{Rgb, Color};
use rbx_types::{Color3, Variant};
// ---------------------------------------------------------------------------------------------------


pub fn parse_hex(capture: &str) -> Variant {
    let rgb = match Rgb::from_hex_str(capture) {
        Ok(ok_rgb) => ok_rgb,
        Err(_) => Rgb::from(255.0, 0.0, 249.0)
    };

    Variant::Color3(Color3::new(rgb.get_red() / 255.0, rgb.get_green() / 255.0, rgb.get_blue() / 255.0))
}