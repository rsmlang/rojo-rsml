use std::fs;

use rbx_rsml::{parse_measurement_sum, tokenize_measurement_sum, tokenize_rsml};

fn main() {
    let file = fs::read_to_string("/Volumes/T7/rsml/project/src/shared/Styles.rsml").unwrap();

    println!("{:#?}", tokenize_rsml(&file));

}