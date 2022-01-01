#[allow(unused_imports)]
use std::{
    //collections::HashMap, //hashmaps aren't real, they can't hurt you
    fs::{read_to_string, File},
    io::prelude::*,
    path::Path,
};

use reqwest::{Client};
//use serde::{Deserialize,Serialize};
use serde_json::Value;

pub mod config;
pub mod discord;
pub mod duo;
use duo::{login,fetch,StreakData,check};
//pub mod duo_data; //obsolete
//use duo_data::{check, update}; //obsolete

/// MAIN. RUNS FIRST
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // define file paths
    let config_path: &str = "config.json";
    let streak_data_path: &str = "streak_data.json";

    // define endpoints
    let login_endpoint: &str = "https://www.duolingo.com/login";

    // check if config path exists, if not
    // cry about it
    if !Path::new(config_path).exists() {

        println!("no data");

    } else {
        // login with stored details
        let my_config = config::get_config(config_path).await?;
        let auth_client = login(&my_config.username, &my_config.password, &login_endpoint).await?;

        // FETCH USERDATA 
        println!("fetching streak data...");
        let streak_data: StreakData = fetch(
            /*&my_config.username,*/
            &my_config.users,
            auth_client
        ).await?;

        //println!("{:?}", streak_data);
        //let my_data_val: Value = serde_json::from_str(&my_data_r)?;

        // check if streak data exists
        if !Path::new(streak_data_path).exists() {

            // if not, cry about nonexistent path
            println!("no old data to check against. fetching new data...");

        } else {

            // if so, check the data in the file
            //check(streak_data_path);
        };

        //update();
    };

    Ok(())
}
