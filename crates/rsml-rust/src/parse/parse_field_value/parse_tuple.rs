// Modules -------------------------------------------------------------------------------------------
use super::{parse_color3, parse_measurement_calc};

use std::{collections::HashMap, sync::LazyLock};

use rbx_types::{Color3, UDim, UDim2, Variant, Vector2};
use regex::Captures;
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
// ---------------------------------------------------------------------------------------------------


// Private Functions ---------------------------------------------------------------------------------
fn parse_tuple_as_udim2(tuple: &str) -> Variant {
    let components: Vec<&str> = tuple.split(",").collect();

    let component_x = match components.get(0) {
        Some(component_x) => {
            let (scale, offset) = parse_measurement_calc(component_x);
            UDim::new(scale, offset)
        },
        None => UDim::new(0.0, 0)
    };

    let component_y = match components.get(1) {
        Some(component_y) => {
            let (scale, offset) = parse_measurement_calc(component_y);
            UDim::new(scale, offset)
        },
        None => UDim::new(0.0, 0)
    };

    Variant::UDim2(UDim2::new(component_x, component_y))
}

fn parse_tuple_as_udim(tuple: &str) -> Variant {
    let (scale, offset) = parse_measurement_calc(tuple);
    Variant::UDim(UDim::new(scale, offset))
}

fn parse_tuple_as_vec2(tuple: &str) -> Variant {
    let components: Vec<&str> = tuple.split(",").collect();

    let component_x = match components.get(0) {
        Some(component_x) => {
            let (scale, _offset) = parse_measurement_calc(component_x);
            scale
        },
        None => 0.0
    };

    let component_y = match components.get(1) {
        Some(component_y) => {
            let (scale, _offset) = parse_measurement_calc(component_y);
            scale
        },
        None => 0.0
    };

    Variant::Vector2(Vector2::new(component_x, component_y))
}


fn parse_tuple_as_color3(tuple: &str) -> Variant {
    let (r, g, b) = parse_color3(tuple);

    Variant::Color3(Color3::new(r, g, b))
}

fn parse_tuple_as_rgb(tuple: &str) -> Variant {
    let (r, g, b) = parse_color3(tuple);
    Variant::Color3(Color3::new(r / 255.0, g / 255.0, b / 255.0))
}
// ---------------------------------------------------------------------------------------------------


pub fn parse_tuple(captures: Captures) -> Variant {
    match TUPPLE_ANNOTATION_VARIANTS.get(captures.get(1).unwrap().as_str()) {
        None => Variant::String(captures.get(0).unwrap().as_str().to_owned()),
        Some(tuple_fn) => return tuple_fn(captures.get(2).unwrap().as_str())
    }
}