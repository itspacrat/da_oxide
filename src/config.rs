use {
    crate::*,
    serde_json::{Value},
    serde::{Deserialize,Serialize},
    std::{
        fs::{read_to_string}
    }
};

// Holds the config data
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub users: Vec<String>,
    pub webhook_url: String,
    pub giphy_apikey: String,
    pub giphy_rating: String,
    pub username: String,
    pub password: String
}

///takes a `config.json` file and serializes it with serde_json
pub async fn get_config(cfg_path: &str) -> Result<Config, Box<dyn std::error::Error>> {

    if !Path::new(cfg_path).exists() {
        // cry about nonexistent path
        println!("failed to retrieve config file from {}", String::from(cfg_path));
        panic!()

    } else {
        
        // check the config data in the file
        let config_r = read_to_string(cfg_path)
        .expect("Something went wrong whilst reading the config file");

        let config_r: &str = &(config_r.clone());
        
        let config: Config = serde_json::from_str(config_r)?;

        Ok(config)

    }
}