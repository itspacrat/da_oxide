use crate::{config::Config, *};
//use serde::Serialize;
//use serde_json::{Value, to_string, to_value};
use reqwest::{Response, header::HeaderMap};
use std::collections::{HashMap};
use regex::Regex;

/// login() takes a username, password, and endpoint 
pub async fn login(username: &String,password: &String, endpoint: &str) -> Result<HeaderMap, Box<dyn std::error::Error>> {

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
    .default_headers(login_headers)
    .build()?;

    println!("Posting auth request...");
    let resp = client
        .post(endpoint)
        .json(&login_json)
        .send().await?;
    println!("done.\n");

    let response_headers = resp.headers();
    println!("\n\nRESPONSE HEADERS\n\n{:#?}",response_headers);
    let mut response_headers_mut = response_headers.clone();
    /* 
    // csrf token sanitize/stringifying
    println!("converting response to token strings...");
    let csrf_string = &(format!("{:#?}",&resp)[1269..1329]);

    // jwt stringifying
    let resp_string_jwt = &(format!("{:#?}",&resp)[1703..1840]);
    let jwt_string = format!("jwt_token={:#?};",&resp_string_jwt);
    println!("done.\n");

    // token sanitization
    println!("sanitizing tokens...");
    let strip_mangles = Regex::new("\"")?;
    let csrf_string = String::from(strip_mangles.replace_all(&csrf_string,""));
    let jwt_string = String::from(strip_mangles.replace_all(&jwt_string,""));
    let cookie_string = String::from(strip_mangles
        .replace_all(&(format!("{csrf}{jwt}",csrf = &csrf_string,jwt = &jwt_string)),""));
    println!("done.\n");
    println!("{:#?}",cookie_string);
    

    // return the client eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjYzMDcyMDAwMDAsImlhdCI6MCwic3ViIjoyNzgwMjI0MDd9.prezuFQ2Uq_CDyHVXtukHjn-t6q5EjcmSoSXqXUqhbI
    println!("inserting sanitized tokens to new map...");
    let mut fetch_map = HeaderMap::new();
    fetch_map.insert("Authorization",(&jwt_string).parse()?);
    fetch_map.insert("Content-Type", (&content_type).parse()?);
    fetch_map.insert("Accept",(&accept).parse()?);
    //fetch_map.insert("Accept-Encoding",(&accept_encoding).parse()?);// -- not necessary?
    fetch_map.insert("Cookie",(&cookie_string).parse()?);
    fetch_map.insert("User-Agent",(&user_agent).parse()?);
    println!("done.\n");
    */
    

    Ok(response_headers_mut)
}

/// fetches duolingo data for you and tracked users
pub async fn fetch(username: &String,headers: HeaderMap) -> Result<String, Box<dyn std::error::Error>> {

    let main_fetch_url = format!("https://duolingo.com/users/{}",&username);

    let client = Client::builder()
    .default_headers(headers)
    .build()?;

    let resp  = client
    .get(main_fetch_url)
    .send().await?
    .text().await?;

    Ok(resp)
}