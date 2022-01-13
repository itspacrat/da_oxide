use std::{
  collections::HashMap,
  error::Error,
};
use reqwest::header::HeaderMap;

use crate::Client;

pub async fn post_discord(r_body: String, url: &str, icon_url: &str, client: &mut Client)
-> Result<(), Box<dyn Error>> {
    
    /*let icon_url = "https://cdn.discordapp.com/attachments/722708774967574618/841396425429352488/68747470733a2f2f692e696d6775722e636f6d2f68534c30784b502e706e67-NEW-icon.png";*/
    let mut discord_headers: HeaderMap = HeaderMap::new();
    discord_headers.insert("content-type","application/json".parse()?);
    discord_headers.insert("accept","text/plain".parse()?);
    discord_headers.insert("user-agent","duoAlert Oxide".parse()?);

    let data = format!(r#" {{
      "avatar_url":"{icon}",
      "username":"Oxide Alert System",
      "content":"duolingo has updated data for us!",
      "embeds":[{{
          "title":"Oxide Alert",
          "description":"{r_msg}",
          "color":"693382",
          "type":"rich",
          "thumbnail": {{
            "url":""
          }},
          "image": {{
            "url":""
          }},
          "footer":{{
            "text":"duoAlert Oxide",
            "icon_url":"{icon}"
          }}
        }}]
    }}"#, r_msg=format!("{}",&r_body), icon=&icon_url);

    let discord_resp = client.post(url)
    .headers(discord_headers)
    .body(data)
    .send().await?
    .text().await?;

    println!("{:#?}",&discord_resp);
    Ok(())

}