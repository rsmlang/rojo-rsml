use std::fs;

use rbx_rsml::{tokenize_rsml, parse_rsml};

fn main() {
    let file = fs::read_to_string("/Volumes/T7/rsml/project/src/shared/Styles.rsml").unwrap();

    println!("{:#?}", tokenize_rsml(&file));

}