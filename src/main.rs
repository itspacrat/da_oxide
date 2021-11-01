use std::{
    fs::{File,read_to_string},
    path::{Path},
    collections::{HashMap}
};

use reqwest::Client;
use serde::{Deserialize,Serialize};
use serde_json::{Value,Map};

pub mod config;
pub mod discord;
pub mod duo;
pub mod duo_data;

async fn update_data(session: &mut Client, config: &config::Config) -> Result<(), Box<dyn std::error::Error>> {

    let users = &config.users;

    for i in users {

        let user = i;
        let resp = session.get(format!("https://www.duolingo.com/users/{}",user));
        println!("USER: {} | {:#?}",user,resp);

    }

    Ok(())

}

/// MAIN. RUNS FIRST
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // define file paths
    let config_path: &str = "config.json";
    let streak_data_path: &str = "streak_data.json";
    let mut my_client: Client = Client::new();

    // define endpoints
    let login_endpoint: &str = "https://www.duolingo.com/login";

    // check if config path exists
    if !Path::new(config_path).exists() {
        
        // cry about nonexistent path
        println!("no data");

    } else {

        // login with stored details
        let my_config = config::get_config(config_path).await?;
        let mut session = duo::login(
            &my_config.username,
            &my_config.password,
            &login_endpoint, 
            &mut my_client
        ).await?;

        update_data(&mut session, &my_config);

        // check if streak data exists
        if !Path::new(streak_data_path).exists() {

            // if not, cry about nonexistent path
            println!("no data");
            
        } else {

            // if so, check the data in the file
            duo_data::check();

        };

        duo_data::update();
    };

    Ok(())
}
