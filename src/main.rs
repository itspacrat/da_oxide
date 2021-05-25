extern crate serde_json;
extern crate serde;
extern crate serde_derive;
extern crate reqwest;
extern crate chrono;


use core::time;
//use rand::Rng;
use std::{
    /*collections::{
        /*HashMap*/
    },*/
    convert::{
        *
    },
    /*time::{
        SystemTime
    },*/
    fs::{
        File,
        read_to_string
    },
    path::{
        Path
    }
};
use chrono::{
    DateTime,
    Local,
    offset::{
        Utc
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
// HOLDS SERVER CONFIG (from config.json)
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
// HOLDS STREAK DATA (from streak_data.json)
#[derive(Serialize, Deserialize)]
struct StreakData {
    user: String,
    streak: i32
}
// HOLDS LOGIN DATA (from config.json > login: Login)
struct Login {

    username: String,
    password: String

}

struct Headers {
    authorization: String
}

fn login(configdata: Config) {

    //set up logindata and local header struct
    // print username for clarity's sake
    println!("login: attempting login as \"{}\"...",configdata.username);
    println!("login: storing login details...");
    let server_login = Login {

        // pull values from the config into here
        // these may get stolen by this struct,
        // but im not sure -Blake
        username: configdata.username,
        password: configdata.password

    };
    println!("login: details stored.");

    // set up json web token for auth
    let jwt: &str = "None";

    println!("login: preparing jwt...");
    if jwt != "None" {
        // prepare header string and pass in as a Header.authorization value
        let header_string: String = "Bearer ".to_string() + jwt ;
        println!("login: setting headers...");
        let headers = Headers {
            authorization: header_string
        };
        println!("login: headers set.");
    
    } else {
        println!("login: fetch jwt: jwt is {}",jwt);
        println!("login: exit.");
    };

}

// grabs and parses config.json to Config type struct
fn get_config(cfg_path: &str) -> Config {
    
    // read config into a String
    println!("get_config: reading config from \"{}\"", cfg_path);
    let config_r = read_to_string(cfg_path)
        .expect("Something went wrong whilst reading the config file");

    // borrow config_r into itself to make it a string litteral
    let config_r: &str = &config_r;
    println!("get_config: config read.");
    
    // parse string as Config structure
    println!("get_config: parsing config...");
    let config: Config = serde_json::from_str(config_r).unwrap();
    println!(r#"get_config: config parsed.
    "#);
    
    // return the Config type "config"
    config
}

fn send_discord(r_msg: String, url :String, version: String, timestamp: String ) {
    
    let data: &str = format!(r#" {{
        "embeds":[{{
          "title":"Oxide Alert",
          "description":"{msg}",
          "color":0xff8000,
          "type":"rich",
          "thumbnail": {{
            "url":"https://cdn.discordapp.com/attachments/722708774967574618/841396425429352488/68747470733a2f2f692e696d6775722e636f6d2f68534c30784b502e706e67-NEW-icon.png"
          }},
          "image": {{
            "url":"{gif}"
          }},
          "footer":{{
            "text":"DuoAlert v{ver} | {tstamp} | Powered by GIPHY",
            "icon_url":"https://cdn.discordapp.com/attachments/722708774967574618/841396425429352488/68747470733a2f2f692e696d6775722e636f6d2f68534c30784b502e706e67-NEW-icon.png"
          }}
        }}]
    }}"#,msg=r_msg,gif=url,ver=version,tstamp=timestamp).as_str();
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
fn fetch_streak_data(streak_data_path: &str) {
    
    // check if streak data file exists
    println!("main: checking streak data path...");
    if !Path::new(streak_data_path).exists() {
        
        // cry about nonexistent path
        println!("Oxide: error: file \"{}\" dosen't exist!",streak_data_path);
        println!("main: fetch {}: exit.",streak_data_path);
    
    } else {

        // check the data in the file
        check_data(streak_data_path);

    };
}
// check if streak data file exists
        println!("main: checking streak data path...");
        if !Path::new(streak_data_path).exists() {
            
            // cry about nonexistent path
            println!("Oxide: error: file \"{}\" dosen't exist!",streak_data_path);
            println!("main: fetch {}: exit.",streak_data_path);
        
        } else {

            // check the data in the file
            check_data(streak_data_path);

        };

fn update_data_file() {
    /*
    Dump json to streak_data.json
    */
}

fn truncate_timestamp(init_time: DateTime<Local>) -> String  {
    
    // grab raw timestamp and String > &str-ify
    let timestamp: String = format!("{}",init_time);
    let timestamp: &str = &timestamp;

    // collect the raw timestamp into a Vec<char>
    let timestamp_chars: Vec<char> = timestamp.chars().collect();

    // slice and collect the previous Vec<char> and re-String it
    let timestamp_truncated = &timestamp_chars[0..16].to_vec();
    let timestamp = timestamp_truncated.iter().collect::<String>();

    // return the bullshit
    timestamp
}
fn main() {

    //set up timestamping
    let init_time: DateTime<Local> = Local::now();

    // pretty-ify the timestamp for printing
    let timestamp = truncate_timestamp(init_time);
    
    // define startup info vars
    let version: String = String::from("0.1.0");
    
    // define filepaths
    let config_path: &str = "config.json";
    let streak_data_path: &str = "streak_data.json";

    println!(r#"starting duoAlertOxide"#);
    println!(r#"
    timestamp: {}
    duoAlertOxide ver: {}
    config path: {}
    streak data path: {}
    "#,timestamp,version,config_path,streak_data_path);
    // define urls and endpoints
    let login_url: &str = "https://www.duolingo.com/login";
    let sadness_gif: &str = "https://media.giphy.com/media/Ty9Sg8oHghPWg/giphy.gif";

    // Todo: Impliment these checks better ?????
    println!("main: checking config path...");
    if !Path::new(config_path).exists() {

        // cry about nonexistent path
        println!("main: error: file \"{}\" dosen't exist!",config_path);
        println!("main: fetch {}: exit.",config_path);

    } else {

        // instanciate config server_config: Config
        let server_config = get_config(config_path);


        // locally update the data
        update_data_file()

    };
    println!(r#"main: exit.
    "#);
}

// Todo: impliment tests
