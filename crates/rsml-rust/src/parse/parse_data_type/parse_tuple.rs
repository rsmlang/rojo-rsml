// Modules -------------------------------------------------------------------------------------------
use super::parse_data_type;

use std::{collections::HashMap, sync::LazyLock};

use rbx_types::{Color3, UDim, UDim2, Variant, Vector2};
use regex::Captures;
// ---------------------------------------------------------------------------------------------------


// Globals -------------------------------------------------------------------------------------------
static TUPPLE_ANNOTATION_VARIANTS: LazyLock<HashMap<String, fn(Vec<Variant>) -> Variant>> = LazyLock::new(|| {
    let mut hashmap: HashMap<String, fn(Vec<Variant>) -> Variant> = HashMap::new();
    hashmap.insert(String::from("udim2"), parse_tuple_as_udim2);
    hashmap.insert(String::from("udim"), parse_tuple_as_udim);
    hashmap.insert(String::from("vec2"), parse_tuple_as_vec2);
    hashmap.insert(String::from("rgb"), parse_tuple_as_rgb);
    hashmap.insert(String::from("color3"), parse_tuple_as_color3);

    hashmap
});
// ---------------------------------------------------------------------------------------------------


// Private Functions ---------------------------------------------------------------------------------
fn get_udim_from_variant(component: &Variant) -> UDim {
    match component {
        Variant::UDim(udim) => *udim, 
        // floats are interpretted as a udim with only the scale component.
        Variant::Float32(scale) => UDim::new(*scale, 0),
        _ => UDim::new(0.0, 0),
    }
}

fn get_float_from_variant(component: &Variant) -> f32 {
    match component {
        Variant::Float32(float) => *float, 
        _ => 0.0,
    }
}

fn parse_tuple_as_udim2(components: Vec<Variant>) -> Variant {
    let component_x = match components.get(0) {
        Some(component) => get_udim_from_variant(component),
        None => UDim::new(0.0, 0)
    };

    let component_y = match components.get(1) {
        Some(component) => get_udim_from_variant(component),
        None => component_x
    };

    Variant::UDim2(UDim2::new(component_x, component_y))
}

fn parse_tuple_as_udim(components: Vec<Variant>) -> Variant {
    match components.get(0) {
        Some(component) => Variant::UDim(get_udim_from_variant(component)),
        None => Variant::UDim(UDim::new(0.0, 0))
    }
}

fn parse_tuple_as_vec2(components: Vec<Variant>) -> Variant {
    let component_x = match components.get(0) {
        Some(component) => get_udim_from_variant(component).scale,
        None => 0.0
    };

    let component_y = match components.get(1) {
        Some(component) => get_udim_from_variant(component).scale,
        None => component_x
    };

    Variant::Vector2(Vector2::new(component_x, component_y))
}


fn parse_tuple_as_color3(components: Vec<Variant>) -> Variant {
    let component_r = match components.get(0) {
        Some(component) => get_float_from_variant(component),
        None => 0.0
    };

    let component_g = match components.get(1) {
        Some(component) => get_float_from_variant(component),
        None => 0.0
    };

    let component_b = match components.get(2) {
        Some(component) => get_float_from_variant(component),
        None => 0.0
    };

    Variant::Color3(Color3::new(component_r, component_g, component_b))
}

fn parse_tuple_as_rgb(components: Vec<Variant>) -> Variant {
    let component_r = match components.get(0) {
        Some(component) => get_float_from_variant(component),
        None => 0.0
    };

    let component_g = match components.get(1) {
        Some(component) => get_float_from_variant(component),
        None => 0.0
    };

    let component_b = match components.get(2) {
        Some(component) => get_float_from_variant(component),
        None => 0.0
    };

    Variant::Color3(Color3::new(component_r / 255.0, component_g / 255.0, component_b / 255.0))
}
// ---------------------------------------------------------------------------------------------------


pub fn parse_tuple(captures: Captures) -> Variant {
    let datatype = captures.get(1).unwrap().as_str();
    let components = captures.get(2).unwrap().as_str().split(",").map(parse_data_type).collect();

    match TUPPLE_ANNOTATION_VARIANTS.get(datatype) {
        None => Variant::Bool(false),
        Some(tuple_fn) => tuple_fn(components)
    }
}