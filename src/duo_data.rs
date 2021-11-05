use crate::{*, config::Config};
use reqwest::{Response};
//use serde_json::{Value};
///test if a streak is greater than, equal to,
/// or less than the previous streak.
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

async fn update_data(session: &mut Client, config: &config::Config) -> Result<(), Box<dyn std::error::Error>> {

    let users = &config.users;

    for i in users {

        let user = i;
        let resp = session.get(format!("https://www.duolingo.com/users/{}",user));
        println!("USER: {} | {:#?}",user,resp);

    }

    Ok(())

}

pub async fn mine_jwt(json: Response) {

}