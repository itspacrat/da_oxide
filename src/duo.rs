use crate::{*};
use reqwest::{header::HeaderMap, Response};
use std::collections::HashMap;

/// login() takes a username, password, and endpoint
pub async fn login(
    username: &String,
    password: &String,
    endpoint: &str,
) -> Result<Client, Box<dyn std::error::Error>> {
    // DEFINE DEFAULT HEADER VALUES.
    let content_type = String::from("application/json");
    let accept = String::from("text/plain");
    let accept_encoding = String::from("identity");
    let user_agent = String::from("duoalert_oxide");

    let mut login_json = HashMap::new();
    let mut login_headers = HeaderMap::new();

    // ADD LOGIN HEADERS TO NEW CLIENT.
    println!("inserting login body...");
    login_json.insert("login", username);
    login_json.insert("password", password);
    println!("done.\n");

    println!("inserting login headers...");
    login_headers.insert("Content-Type", (&content_type).parse()?);
    login_headers.insert("Accept", (&accept).parse()?);
    login_headers.insert("Accept-Encoding", (&accept_encoding).parse()?);
    login_headers.insert("User-Agent", (&user_agent).parse()?);
    println!("done.\n");

    let client = Client::builder()
        .default_headers(login_headers.clone())
        .cookie_store(true)
        .build()?;

    println!("Posting auth request...");
    let resp = client.post(endpoint).json(&login_json).send().await?;
    println!("done.\n");

    let response_headers = resp.headers();
    //println!("\n\nRESPONSE HEADERS\n\n{:#?}",response_headers);
    //let mut response_headers_mut = response_headers.clone();

    // form Auth header with values
    login_headers.insert(
        "Authorization",
        (format!("Bearer {}", response_headers["jwt"].to_str()?)).parse()?,
    );

    Ok(client.clone())
}

/// fetches duolingo data for you and tracked users
pub async fn fetch(
    /*username: &String,*/
    users: &Vec<String>,
    client: Client,
) -> Result<String, Box<dyn std::error::Error>> {


    let mut data_val: serde_json::Map<String,Value> = serde_json::Map::new();
    
    // loop through users in config and fetch profile responses (SLOW :<)
    for user in users {
        let main_fetch_url = format!("https://duolingo.com/users/{}", &user);

        println!("fetching data for user {}",&user);

        let resp: String = client
        .get(main_fetch_url)
        //.headers(headers)
        .send()
        .await?
        .text()
        .await?;
        println!("done.\n");

        let user_val_r: Value = serde_json::from_str(&resp)?;
        let user_val: Value = user_val_r["site_streak"].clone();

        data_val.insert(format!("{}",user),user_val);
        
    }


    //let resp_val = resp;
    Ok(String::from("placeholder"))
}

///test if a streak is greater than, equal to,
/// or less than the previous streak.
pub fn check(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Read streak data file to string
    let previous_r: &str = &(read_to_string(path)?);

    // parse string as json val
    let previous: Value = serde_json::from_str(previous_r)?;

    Ok(())
}
