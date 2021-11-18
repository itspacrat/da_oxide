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

    // csrf token sanitize/stringifying
    println!("converting response to token strings...");
    let csrf_string = &(format!("{:#?}",&resp)[1269..1329]);

    // jwt stringifying
    let resp_string_jwt = &(format!("{:#?}",&resp)[1703..1840]);
    let jwt_string = format!("Bearer {:#?}",&resp_string_jwt);
    println!("done.\n");

    // token sanitization
    println!("sanitizing tokens...");
    let strip_mangles = Regex::new("\"")?;
    let csrf_string = String::from(strip_mangles.replace_all(&csrf_string,""));
    let jwt_string = String::from(strip_mangles.replace_all(&jwt_string,""));
    println!("done.\n");
    
    //login_headers.insert("Authorization",(&jwt_string).parse()?);

    

    // return the client eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjYzMDcyMDAwMDAsImlhdCI6MCwic3ViIjoyNzgwMjI0MDd9.prezuFQ2Uq_CDyHVXtukHjn-t6q5EjcmSoSXqXUqhbI
    println!("inserting jwt");
    let mut auth_map = HeaderMap::new();
    auth_map.insert("Authorization",(&jwt_string).parse()?);
    auth_map.insert("Content-Type", (&content_type).parse()?);
    auth_map.insert("Accept",(&accept).parse()?);
    //auth_map.insert("Accept-Encoding",(&accept_encoding).parse()?);// -- not necessary?
    auth_map.insert("set-cookie",(&csrf_string).parse()?);
    auth_map.insert("User-Agent",(&user_agent).parse()?);

    //let jwt_client = Client::builder().default_headers(auth_map).build()?;// -- no longer building another client here
    println!("\n{:?}",&auth_map);
    //println!("{:#?}",&jwt_client);
    Ok(auth_map.clone())
}

/// fetches duolingo data for you and tracked users
pub async fn fetch(username: &String,headers: HeaderMap) -> Result<String, Box<dyn std::error::Error>> {
    unimplemented!();
    let main_fetch_url = format!("https://duolingo.com/users/{}",&username);

    /*

    BUILD CLIENT HERE WITH HEADERMAP FISRT THING!!!!!!!!!!!!!!!!!!!!!!!!!!!!

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

    let resp  = client
    .get(main_fetch_url)
    .send().await?
    .text().await?;

    println!("\n\n{:?}",resp);
    Ok(resp)
    */
}