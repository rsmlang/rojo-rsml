pub fn parse_color3(tuple: &str) -> (f32, f32, f32) {
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