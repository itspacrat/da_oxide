use std::{
    collections::HashMap,
    fs::{read_to_string, File},
    path::Path,
};

use reqwest::{Client};
//use serde::{Deserialize,Serialize};
use serde_json::Value;

pub mod config;
pub mod discord;
pub mod duo;
use duo::{fetch,login};
pub mod duo_data;
use duo_data::{check, update};

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
        let login_map =
            login(&my_config.username, &my_config.password, &login_endpoint).await?;

        // FETCH USERDATA AND PRINT RESP
        println!("fetching userdata...");
        let my_data = fetch(&my_config.username,login_map).await?;
        println!("\n\n{:#?}",&my_data);

        // check if streak data exists
        if !Path::new(streak_data_path).exists() {

            // if not, cry about nonexistent path
            println!("no data");

        } else {

            // if so, check the data in the file
            //check();
        };

        //update();
    };

    Ok(())
}
