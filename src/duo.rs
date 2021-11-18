use crate::{config::Config, *};
//use serde::Serialize;
//use serde_json::{Value, to_string, to_value};
use reqwest::{Response, header::HeaderMap};
use std::collections::{HashMap};
use regex::Regex;

pub async fn login(username: &String,password: &String, endpoint: &str) -> Result<Client, Box<dyn std::error::Error>> {

    // Insert relevant headers
    let content_type = String::from("application/json");
    let accept = String::from("text/plain");
    let accept_encoding = String::from("identity");
    let user_agent = String::from("duoalert_oxide");

    let mut login_json = HashMap::new();
    let mut login_headers = HeaderMap::new();

    println!("Inserting headers");
    login_json.insert("login",username);
    login_json.insert("password",password);
    
    login_headers.insert("Content-Type", (&content_type).parse()?);
    login_headers.insert("Accept",(&accept).parse()?);
    login_headers.insert("Accept-Encoding",(&accept_encoding).parse()?);
    login_headers.insert("User-Agent",(&user_agent).parse()?);
    
    println!("done.\n");

    let client = Client::builder().default_headers(login_headers.clone()).build()?;

    println!("Posting auth request");
    let resp = client
        .post(endpoint)
        .json(&login_json)
        .send().await?;
    println!("done.\n");

    // make that shit USABLE
    println!("converting response to string");
    let csrf_string = &(format!("{:#?}",&resp)[1269..1329]);

    

    let resp_string_jwt = &(format!("{:#?}",&resp)[1703..1840]);
    
    //let resp_vec: &Vec<char> = &(resp_string.chars().collect::<Vec<char>>());
    //println!("{:?}",&resp_string);
    let strip_mangles = Regex::new("\"")?;
    println!("jwt:");
    let jwt_string = format!("Bearer {:#?}",&resp_string_jwt);

    let jwt_string = String::from(strip_mangles.replace_all(&jwt_string,""));
    println!("done.\n");
    println!("{}",&jwt_string);
    
    //login_headers.insert("Authorization",(&jwt_string).parse()?);

    println!("\n{:?}",&login_json);

    // return the client eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjYzMDcyMDAwMDAsImlhdCI6MCwic3ViIjoyNzgwMjI0MDd9.prezuFQ2Uq_CDyHVXtukHjn-t6q5EjcmSoSXqXUqhbI
    println!("inserting jwt");
    let mut auth_map = HeaderMap::new();
    auth_map.insert("Authorization",(&jwt_string).parse()?);
    auth_map.insert("Content-Type", (&content_type).parse()?);
    auth_map.insert("Accept",(&accept).parse()?);
    auth_map.insert("Accept-Encoding",(&accept_encoding).parse()?);
    auth_map.insert("User-Agent",(&user_agent).parse()?);

    let jwt_client = Client::builder().default_headers(auth_map).build()?;

    println!("{:#?}",&jwt_client);
    Ok(jwt_client)
}

/// fetches duolingo data for you and tracked users
pub async fn fetch(config: &Config,client: &mut Client) -> Result<String, Box<dyn std::error::Error>> {
    
    let main_fetch_url = format!("https://duolingo.com/users/{}",&config.username);

    /*
    let mut login_json = HashMap::new();

    let content_type = String::from("application/json");
    let accept = String::from("text/plain");
    let accept_encoding = String::from("identity");
    let user_agent = String::from("duoalert_oxide");

    println!("Inserting headers");
    login_json.insert("login",&config.username);
    login_json.insert("password",&config.password);
    login_json.insert("Content-Type", &content_type);
    login_json.insert("Accept",&accept);
    login_json.insert("Accept-Encoding",&accept_encoding);
    login_json.insert("User-Agent",&user_agent);
    println!("done.\n");
    */

    let resp  = client
    .get(main_fetch_url)
    .send().await?
    .text().await?;

    println!("\n\n{:?}",resp);
    Ok(resp)
}