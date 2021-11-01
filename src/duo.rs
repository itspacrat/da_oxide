use crate::*;

/// login takes in a username, password, a url, and a mutable
/// reference to a client and sends out a mutated/tokenized client
pub async fn login(login_username: &String, login_password: &String, login_url: &str, login_client: &mut Client) 
-> Result<Client, Box<dyn std::error::Error>> {
    
    let login_r:&str = &(format!(r#"
    {{
        "login":"{login_username}",
        "password":"{login_password}"
    }}
    "#,login_username=&login_username,login_password=&login_password).clone());
    
    let login_data_json = serde_json::from_str(login_r)?;

    let resp = login_client.post(login_url)
    .json(&login_data_json)
    .header("Content-type", "application/json")
    .header("Accept","identity")
    .header("User-agent","duoAlertOxide")
    .send()
    .await?;

    println!("response: {:#?}",resp);
    Ok(login_client.clone())
}