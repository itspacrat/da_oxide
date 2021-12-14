use crate::{config::Config, *};
//use serde::Serialize;
//use serde_json::{Value, to_string, to_value};
use reqwest::{Response, header::HeaderMap};
use serde_json::to_value;
use std::collections::{HashMap};
use regex::Regex;

/// login() takes a username, password, and endpoint 
pub async fn login(username: &String,password: &String, endpoint: &str) -> Result<Client, Box<dyn std::error::Error>> {

    // DEFINE DEFAULT HEADER VALUES.
    let content_type = String::from("application/json");
    let accept = String::from("text/plain");
    let accept_encoding = String::from("identity");
    let user_agent = String::from("duoalert_oxide");

    let mut login_json = HashMap::new();
    let mut login_headers = HeaderMap::new();


    // ADD LOGIN HEADERS TO NEW CLIENT.
    println!("inserting login body...");
    login_json.insert("login",username);
    login_json.insert("password",password);
    println!("done.\n");
    
    println!("inserting login headers...");
    login_headers.insert("Content-Type", (&content_type).parse()?);
    login_headers.insert("Accept",(&accept).parse()?);
    login_headers.insert("Accept-Encoding",(&accept_encoding).parse()?);
    login_headers.insert("User-Agent",(&user_agent).parse()?);
    println!("done.\n");


    let client = Client::builder()
    .default_headers(login_headers.clone())
    .cookie_store(true)
    .build()?;

    println!("Posting auth request...");
    let resp = client
        .post(endpoint)
        .json(&login_json)
        .send().await?;
    println!("done.\n");

    let response_headers = resp.headers();
    //println!("\n\nRESPONSE HEADERS\n\n{:#?}",response_headers);
    //let mut response_headers_mut = response_headers.clone();
   

    // form Auth header with values
    login_headers.insert("Authorization",(format!("Bearer {}",response_headers["jwt"].to_str()?)).parse()?);

    Ok(client.clone())
}

/// fetches duolingo data for you and tracked users
pub async fn fetch(username: &String,client: Client) -> Result<String, Box<dyn std::error::Error>> {

    let main_fetch_url = format!("https://duolingo.com/users/{}",&username);

    /*let client = Client::builder()
    .default_headers(headers)
    .build()?;*/

    let resp  = client.get(main_fetch_url)
    //.headers(headers)
    .send().await?
    .text().await?;

    //let resp_val = resp;
    Ok(resp)
}