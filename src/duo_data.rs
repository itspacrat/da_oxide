use crate::*;

pub fn check() {
    unimplemented!();

    // Read streak data file to string
    let previous_r = read_to_string("streak_data.json")
        .expect("Something went wrong whilst reading the config file");

    // make previous_r string literal by borrowing previous_r into itself
    let previous_r: &str = &previous_r;
    
    // parse string as json val
    let previous: serde_json::Value = serde_json::from_str(previous_r)
        .expect("JSON was not well-formatted");
    
    /* dig through json data and check value deltas */
    
}

pub fn update() {
    unimplemented!()
}