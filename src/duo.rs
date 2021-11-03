use crate::{config::Config, *};

/// fetches duolingo data for you and tracked users
pub async fn fetch(config: &Config,url: &str) -> Result<String, Box<dyn std::error::Error>> {
    //unimplemented!();
    let resp = get(&(format!("{}{}",&url,&config.username))).await?.text().await?;
    // for each user, get response
    // of login_endpoint/user
    println!("{}",&resp);
    Ok(resp)
}
