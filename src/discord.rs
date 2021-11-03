use crate::*;

pub fn send_discord(r_msg: String, url :String, version: String, timestamp: String, client: &mut Client ) {
    
    let icon_url = "https://cdn.discordapp.com/attachments/722708774967574618/841396425429352488/68747470733a2f2f692e696d6775722e636f6d2f68534c30784b502e706e67-NEW-icon.png";

    let data = format!(r#" {{
        "embeds":[{{
          "title":"Oxide Alert",
          "description":"{r_msg}",
          "color":"693382",
          "type":"rich",
          "thumbnail": {{
            "url":"{icon}"
          }},
          "image": {{
            "url":"{icon}"
          }},
          "footer":{{
            "text":"DuoAlert v{version} | {timestamp} | Powered by GIPHY",
            "icon_url":"{icon}"
          }}
        }}]
    }}"#, r_msg=&r_msg, version=&version, timestamp=&timestamp, icon=&icon_url);
}