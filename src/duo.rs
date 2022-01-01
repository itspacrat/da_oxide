use crate::*;
use reqwest::{header::HeaderMap, Response};
use serde::{Deserialize, Serialize};
use serde_json;
use std::{collections::HashMap, error::Error};

#[derive(Debug, Serialize, Deserialize)]
pub struct StreakData {
    pub users: Option<HashMap<String, u32>>,
}
/// login() returns a mutated client with login cookies set.
///
/// it takes in a username, password, and a login endpoint.
pub async fn login(
    username: &String,
    password: &String,
    endpoint: &str,
) -> Result<Client, Box<dyn Error>> {
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
    let resp: Response = client.post(endpoint).json(&login_json).send().await?;
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
pub async fn fetch(users: &Vec<String>, client: Client) -> Result<StreakData, Box<dyn Error>> {
    let mut user_map: HashMap<String, u32> = HashMap::new();

    // loop through users in config and fetch profile responses (SLOW :<)
    for user in users {
        println!("    fetching data for user {}", &user);

        let main_fetch_url = format!("https://duolingo.com/users/{}", &user);
        let resp: String = client
            .get(main_fetch_url)
            //.headers(headers)
            .send()
            .await?
            .text()
            .await?;
        println!("    done.\n");

        // convert the json resp into a Value for easy map insertion
        let user_val_r: Value = serde_json::from_str(&resp)?;
        // grab the little fucker that we went through this porocess
        // for. 0/10 not worth the hassle, duolingo. i hope you push
        // a break to prod or something.
        let user_val: String = (user_val_r["site_streak"].clone()).to_string();

        user_map.insert(user.clone(), user_val.parse()?);
        //data_val.users[user] = ;

        /* // later we may add more features and tracking
        // to enrich the discord posts. you can optionally
        // compile with features=[..., "advanced"] to enable
        // these.
        #[cfg(feature="advanced")]
        {

        }
        */
    }

    let streak_data: StreakData = StreakData {
        users: Some(user_map),
    };
    println!("done.\n");

    //let agg_data_r: String = serde_json::to_string_pretty(&data_val)?;
    //let agg_data_struct: StreakData = serde_json::from_str(&(agg_data_r.clone()))?;

    //println!("{:#?}",&agg_data_struct);

    Ok(streak_data)
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
