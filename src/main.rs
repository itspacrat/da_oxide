/*
extern crate serde_json;
extern crate serde;
extern crate serde_derive;
extern crate reqwest;
*/

//use rand::Rng;
use std::{
    /*collections::{
        /*HashMap*/
    },*/
    fs::{
        File,
        read_to_string
    },
    path::{
        Path
    }
};

use serde::{
    Deserialize,
    Serialize
};

use serde_json::{
    Value,
    Map,
    Result
};

#[derive(Serialize, Deserialize)]
struct Config {
    username: String,
    password: String,
    webhook_url: String,
    users: Vec<String>,
    use_giphy: bool,
    giphy_rating: String,
    giphy_apikey: String
}

#[derive(Serialize, Deserialize)]
struct StreakData {
    user: String,
    streak: i32
}

struct Login {

    username: String,
    password: String

}

struct Headers {
    authorization: &'static str
}

fn login(configdata: Config) {

    //set up logindata and local header struct
    let headers = Headers{authorization: ""};
    let server_login = Login {

        // pull values from the config into here
        // these may get stolen by this struct 
        username: configdata.username,
        password: configdata.password

    };

    // set up json web token for auth
    let jwt: &str = "None";

    if jwt != "None"  {
        let headers.authorization = concat!("Bearer ",jwt);
    } else {

    }

}

fn get_config(cfg_path: &str) -> Config {
    
    // 
    let config_r = read_to_string(cfg_path)
        .expect("Something went wrong whilst reading the config file");
    
    // borrow config_r into itself to make it a string litteral
    let config_r: &str = &config_r;
    
    // parse string as ConfigMap structure
    let config: Config = serde_json::from_str(config_r).unwrap();
    
    // return the Config type "config"
    config
}

fn send_discord(r_msg: String, url :String, version: String, timestamp: String ) {
    
    let data = (r#" {
        "embeds":[{
          "title":"Oxide Alert",
          "description":"{r_msg}",
          "color":0xff8000,
          "type":"rich",
          "thumbnail": {
            "url":"https://cdn.discordapp.com/attachments/722708774967574618/841396425429352488/68747470733a2f2f692e696d6775722e636f6d2f68534c30784b502e706e67-NEW-icon.png"
          },
          "image": {
            "url":"{url}"
          },
          "footer":{
            "text":"DuoAlert v{} | {} | Powered by GIPHY",
            "icon_url":"https://cdn.discordapp.com/attachments/722708774967574618/841396425429352488/68747470733a2f2f692e696d6775722e636f6d2f68534c30784b502e706e67-NEW-icon.png"
          }
        }]
    }"#, r_msg, url, version, timestamp);
}

fn update_data() {

}

fn check_data(path: &str) {

    // Read streak data file to string
    let previous_r = read_to_string(path)
        .expect("Something went wrong whilst reading the config file");

    // make previous_r string literal by borrowing previous_r into itself
    let previous_r: &str = &previous_r;
    
    // parse string as StreakData structure
    let previous: StreakData = serde_json::from_str(previous_r).unwrap();


}

fn update_data_file() {
    /*
    Dump json to streak_data.json
    */
}

fn main() {

    // define filepaths
    let config_path: &str = "config.json";
    let streak_data_path: &str = "streak_data.json";

    // define urls and endpoints
    let login_url: &str = "https://www.duolingo.com/login";
    let sadness_gif: &str = "https://media.giphy.com/media/Ty9Sg8oHghPWg/giphy.gif";

    // Todo: Impliment these checks better ?????
    if !Path::new(config_path).exists() {

        // cry about nonexistent path
        println!("error: file \"{}\" dosen't exist!",config_path);

    } else {

        // check the data in the file
        let server_config = get_config(config_path);
        
        // 
        println!("logging in with user {}",server_config.username);

        // 
        if !Path::new(streak_data_path).exists() {
            
            // cry about nonexistent path
            println!("error: file \"{}\" dosen't exist!",streak_data_path);
        
        } else {

            // check the data in the file
            check_data(streak_data_path);

        };

        // locally update the data
        update_data_file()

    };
 
}

// Todo: impliment tests
