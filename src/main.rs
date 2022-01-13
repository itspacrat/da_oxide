#[allow(unused_imports)]
use duolingo_rs::*;

use reqwest::Client;
use serde_json::to_value;
use std::{collections::HashMap, fs::File, io::prelude::*, path::Path};

pub mod config;
pub mod discord;
use discord::*;
pub mod duo;
use duo::check;

/// MAIN. RUNS FIRST
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // define paths & endpoints
    let login_endpoint: &str = "https://www.duolingo.com/login";
    let config_path: &str = "config.json";
    let streak_data_path: &str = "streak_data.json";
    let extension_icon = "https://cdn.discordapp.com/attachments/722708774967574618/841396425429352488/68747470733a2f2f692e696d6775722e636f6d2f68534c30784b502e706e67-NEW-icon.png";
    let sadness_icon = "https://media.discordapp.net/attachments/722708774967574618/911016021593305128/68747470733a2f2f692e696d6775722e636f6d2f68534c30784b502e706e67-NEW-icon-INVERT.png";
    //
    // define r_msg bodies
    let sadness_body = String::from("STREAK LOSS - LOSER ALERT: ");
    let extension_body = String::from("New day, one step further: ");

    let extension_key = String::from("extensions");
    let loss_key = String::from("losses");
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
        let streak_data = to_value(fetch_streak_map(my_config.users, auth_client).await?)?;
        println!("done.\n");

        //
        // check file path and create streak data
        // with the new data if the file dosen't exist.
        if Path::new(streak_data_path).exists() {
            let streaks: HashMap<String, HashMap<String, u16>> =
                check(streak_data_path, streak_data)?;
            /* DEBUG */
            println!("{:#?}", &streaks);
            for (streak_type, streak_map) in streaks {
                if streak_type == extension_key {
                    for (streak_user, streak_length) in streak_map.to_owned() {
                        println!("posting extension for {}", &streak_user);
                        println!("{}{} - {}", &extension_body, &streak_user, &streak_length);
                        post_discord(
                            format!("{}{} - {}", &extension_body, &streak_user, &streak_length),
                            &my_config.webhook_url,
                            extension_icon,
                            &mut Client::new(),
                        )
                        .await?;
                    }
                } else if streak_type == loss_key {
                    for (streak_user, streak_length) in streak_map.to_owned() {
                        println!("posting loss for {}", &streak_user);
                        println!("{}{} - {}", &sadness_body, &streak_user, &streak_length);
                        post_discord(
                            format!("{}{} - {}", &sadness_body, &streak_user, &streak_length),
                            &my_config.webhook_url,
                            sadness_icon,
                            &mut Client::new(),
                        )
                        .await?;
                    }
                }
            }
        } else {
            let mut streak_file = File::create(streak_data_path)?;
            streak_file.write_all(streak_data.to_string().as_bytes())?;
        }
    }

    Ok(())
}
