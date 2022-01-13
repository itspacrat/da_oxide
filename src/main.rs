#[allow(unused_imports)]
use duolingo_rs::*;

use reqwest::Client;
use serde_json::{to_value};
use std::{
    fs::{File},
    io::prelude::*,
    path::Path,
};

pub mod config;
pub mod discord;
//use discord::*;
pub mod duo;
use duo::{check};

/// MAIN. RUNS FIRST
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // define paths & endpoints
    let login_endpoint: &str = "https://www.duolingo.com/login";
    let config_path: &str = "config.json";
    let streak_data_path: &str = "streak_data.json";

    //
    // check if config path exists, if not
    // cry about it
    if !Path::new(config_path).exists() {
        println!("no config to parse. so sad :( go make one (example at https://github.com/parkcitymedia/duo_alert_oxide)");
    } else {

        //
        // login with stored details
        let my_config = config::get_config(config_path).await?;
        let auth_client = login(my_config.username, my_config.password, &login_endpoint).await?;

        //
        // fetch userdata
        println!("fetching streak data...");
        let streak_data  = to_value(fetch_streak_map(my_config.users, auth_client).await?)?;
        println!("done.\n");

        //
        // check file path and create streak data
        // with the new data if the file dosen't exist.
        if Path::new(streak_data_path).exists() {
            check(streak_data_path, streak_data)?;
            // post()
        } else {
            let mut streak_file = File::create(streak_data_path)?;
            streak_file.write_all(streak_data.to_string().as_bytes())?;

        } 
    };

    Ok(())
}
