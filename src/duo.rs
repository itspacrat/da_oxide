use crate::{config::Config, *};

/// login takes in a username, password, a url, and a mutable
/// reference to a client and sends out a mutated/tokenized client
/*
pub async fn login(
    login_username: &String,
    login_password: &String,
    login_url: &str,
    login_client: &mut Client,
) -> Result<Client, Box<dyn std::error::Error>> {
    println!("{}{}", &login_url, &login_username);
    let resp = login_client
        .get(&(format!("{}{}",&login_url, &login_username)))
        .text()
        .await?;

    println!("response: {:#?}", resp);
    Ok(login_client.clone())
}
*/

/// fetches duolingo data for you and tracked users
pub async fn fetch(config: &Config,url: &str) -> Result<String, Box<dyn std::error::Error>> {
    //unimplemented!();
    let resp = get(&(format!("{}{}",&url,&config.username))).await?.text().await?;
    // for each user, get response
    // of login_endpoint/user
    println!("{}",&resp);
    Ok(resp)
}
