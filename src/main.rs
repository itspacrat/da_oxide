extern crate serde_json;
extern crate serde;
extern crate serde_derive;
extern crate reqwest;

//use rand::Rng;
use std::{error,fs::{File, read_to_string,}, path::Path, env };
//use std::
//use std::fs::{File, };
//use std::io::Read;
use serde::{Deserialize, Serialize};
use serde_json::{Value, Map, Result};

// Holds the config data
// #[derive(Serialize, Deserialize, Debug)]
struct Config {
    users: String,
    webhook_url: String,
    giphy_apikey: String,
    giphy_rating: String,
    username: String,
    password: String
}

fn get_config(cfg_path: &str) {

    if !Path::new(cfg_path).exists() {
        
        // cry about nonexistent path
        println!("failed to retrieve config file from {}", String::from(cfg_path));

    } else {
        
        // check the config data in the file
        let config_r = read_to_string(cfg_path)
        .expect("Something went wrong whilst reading the config file");

        let config_r: &str = &config_r;
    
        // parse string as json val
        let config: serde_json::Value = serde_json::from_str(config_r)
        .expect("JSON was not well-formatted");

        // debug
        println!("LOAD CONFIG: {}",String::from(config_r))
        
    }
}

fn send_discord(r_msg: String, url :String, version: String, timestamp: String ) {
    
    let data = (r#" {
        "embeds":[{
          "title":"Oxide Alert",
          "description":"{}",
          "color":0xff8000,
          "type":"rich",
          "thumbnail": {
            "url":"https://cdn.discordapp.com/attachments/722708774967574618/841396425429352488/68747470733a2f2f692e696d6775722e636f6d2f68534c30784b502e706e67-NEW-icon.png"
          },
          "image": {
            "url":"{}"
          },
          "footer":{
            "text":"DuoAlert v{} | {} | Powered by GIPHY",
            "icon_url":"https://cdn.discordapp.com/attachments/722708774967574618/841396425429352488/68747470733a2f2f692e696d6775722e636f6d2f68534c30784b502e706e67-NEW-icon.png"
          }
        }]
    }"#, r_msg, url, version, timestamp);
}

fn login(logindata: Config) {
    let logindata = logindata;//plceholder
    
    // maybe fix and turn this into json later

    //let jwt: &str = "";
}

fn update_data() {

}

fn check_data() {

    // Read streak data file to string
    let previous_r = read_to_string("streak_data.json")
        .expect("Something went wrong whilst reading the config file");

    // make previous_r string literal by borrowing previous_r into itself
    let previous_r: &str = &previous_r;
    
    /*/ debug
    println!("LOAD PREVIOUS STREAK DATA: {}",String::from(previous_r));
    */
    
    // parse string as json val
    let previous: serde_json::Value = serde_json::from_str(previous_r)
        .expect("JSON was not well-formatted");
    
    /* map json to struct? */
    
}

fn update_data_file() {
    /*
    Dump json to streak_data.json
    */
}


fn main() {

    /*
    // Todo: add argv/argc checking before
    // setting vars for custom path
    // support
    */

    // define filepaths
    let config_path: &str = "config.json";
    let streak_data_path: &str = "streak_data.json";

    get_config(config_path);

    //login(my_config);

    update_data();

    /*
    get streak data
    */

    // Todo: Impliment this check better
    if !Path::new(streak_data_path).exists() {
        
        // cry about nonexistent path
        println!("no data");

    } else {
        
        // check the data in the file
        check_data();

    }

    update_data_file();
    
}

// Todo: impliment tests
