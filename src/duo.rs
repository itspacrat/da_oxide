use crate::{config::Config, *};
use reqwest::Response;
use std::collections::{HashMap};

pub async fn login(username: &String,password: &String, endpoint: &str, client: &mut Client) -> Result<Client, Box<dyn std::error::Error>> {
    
    let mut session: Client = Client::new();

    let content_type = String::from("application/json");
    let accept = String::from("text/plain");
    let accept_encoding = String::from("identity");
    let user_agent = String::from("duoalert_oxide");

    let mut login_json = HashMap::new();
    login_json.insert("login",username);
    login_json.insert("password",password);
    login_json.insert("Content-Type", &content_type);
    login_json.insert("Accept",&accept);
    login_json.insert("Accept-Encoding",&accept_encoding);
    login_json.insert("User-Agent",&user_agent);

    let resp = session
    .post(endpoint)
    .json(&login_json)
    .send().await;
    println!("{:?}",resp);
    Ok(session)
}

/// fetches duolingo data for you and tracked users
pub async fn fetch(config: &Config,session: &mut Client) -> Result<String, Box<dyn std::error::Error>> {
    
    let main_fetch_url = format!("https://duolingo.com/users/{}",&config.username);

    let resp  = session
    .get(main_fetch_url)
    //.header("Authorization")
    .send().await?
    .text().await?;

    println!("\n\n{:?}",resp);
    Ok(resp)
}