use std::fs;

use rbx_rsml::{tokenize_rsml, parse_rsml, parse_data_type, tokenize_data_type};

//let file = fs::read_to_string("/Volumes/T7/rsml/project/src/shared/Styles.rsml").unwrap();


fn main() {

    let file = fs::read_to_string("/Volumes/T7/rsml/project/src/shared/Styles.rsml").unwrap();
    
    let tokenized = &tokenize_rsml(&file);
    //let parsed = parse_rsml(tokenized);

    println!("{:#?}", tokenized);
}