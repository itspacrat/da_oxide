// use serde to serialize/deserialize
// json
extern crate serde_json;
extern crate serde;
extern crate serde_derive;
// use reqwest to manage http requests
// for the GET and POST portions
extern crate reqwest;

use std::{
    /*error,*/ // error currently unused
    fs::{
        File, // file used to write streak data
        read_to_string // used to string-ify json
    },
    path::{
        Path // used to check if files exist
    },
    collections::{
        HashMap // used to map json to rust KVPs
    }
};

use serde_json::{
    Value,
    Map,
    Result
};

// Holds the config data
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
        panic!();

    } else {
        
        // check the config data in the file
        let config_r = read_to_string(cfg_path)
        .expect("Something went wrong whilst reading the config file");

        let config_r: &str = &config_r;
    
        /*/ parse string as json val
        let config: serde_json::Value = serde_json::from_str(config_r)
        .expect("JSON was not well-formatted");

        config
        */
        
        config_r
    }
}

fn login(logindata: serde_json::Value) {
    let logindata = logindata;//plceholder
    
    // maybe fix and turn this into json later

    //let jwt: &str = "";
}

fn check_data() {

    // Read streak data file to string
    let previous_r = read_to_string("streak_data.json")
        .expect("Something went wrong whilst reading the config file");

    // make previous_r string literal by borrowing previous_r into itself
    let previous_r: &str = &previous_r;
    
    // parse string as json val
    let previous: serde_json::Value = serde_json::from_str(previous_r)
        .expect("JSON was not well-formatted");
    
    /* map json to hashmap? */
    
}

fn update_data() {

}

fn update_data_file() {
    /*
    Dump json to streak_data.json
    */
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

// MAIN. RUNS FIRST
fn main() {

    // define file paths
    let config_path: &str = "config.json";
    let streak_data_path: &str = "streak_data.json";

    // check if config path exists
    if !Path::new(config_path).exists() {
        // cry about nonexistent path
        println!("no data");
    } else {

        // login with stored details
        let my_config = get_config(config_path);
        login(my_config);
        update_data();

        // check if streak data exists
        if !Path::new(streak_data_path).exists() {

            // if not, cry about nonexistent path
            println!("no data");
            
        } else {

            // if so, check the data in the file
            check_data();

        };
        update_data_file();
    };
}
