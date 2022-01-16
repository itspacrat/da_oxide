use {
    crate::*,
    serde::{Deserialize, Serialize},
    serde_json::from_str,
    std::{error::Error, fs::read_to_string},
};

// Holds the config data
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub users: Vec<String>,
    pub webhook_url: String,
    //pub giphy_apikey: String,
    //pub giphy_rating: String,
    pub username: String,
    pub password: String,
    //pub advanced_options: bool,
}

///takes a `config.json` file and serializes it with serde_json
pub async fn get_config(cfg_path: &str) -> Result<Config, Box<dyn Error>> {
    if !Path::new(cfg_path).exists() {
        // cry about nonexistent path
        panic!("could not find or open config file!")
    } else {
        let config: Config = from_str(&(read_to_string(cfg_path)?).clone())?;
        Ok(config)
    }
}
