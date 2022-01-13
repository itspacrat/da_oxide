use serde_json::{from_str, from_value, Value};
use std::{collections::HashMap, error::Error, fs::read_to_string};
use crate::discord::*;

/// test if a streak is greater than, equal to,
/// or less than the previous streak.
pub fn check(old_path: &str, new_streak_json: Value)
-> Result<HashMap<String, HashMap<String,u16>>, Box<dyn Error>> {
    /* Create data storage local vars */
    let previous_data: HashMap<String, u16> = from_str(&(read_to_string(old_path)?))?; // > Value
    let new_data: HashMap<String, u16> = from_value(new_streak_json)?;
    let mut already_fucking_checked: Vec<String> = Vec::new();
    let mut streak_post_map: HashMap<String, HashMap<String,u16>> = HashMap::new();
    let extension_map: HashMap<String,u16> = HashMap::new();
    let loss_map: HashMap<String,u16> = HashMap::new();
    streak_post_map.insert(String::from("losses"),loss_map);
    streak_post_map.insert(String::from("extenstions"), extension_map);
    /* DEBUG STATEMENTS */
    println!("OLD:\n{:#?}\n", &previous_data);
    println!("NEW:\n{:#?}\n", &new_data);

    /* BIG UGLY CHECKER */
    for (old_key, old_streak) in previous_data.clone() {
        for (new_key, new_streak) in new_data.clone()
        /* comapre all keys to all keys (messy) */
        {
            //
            // was user tracked last time?
            if previous_data.contains_key(&new_key) {
                //
                // are we comparing the same username
                if new_key == old_key {
                    //
                    // is the streak larger now?
                    if new_streak > old_streak {
                        //
                        //extend streak for user
                        /*
                        let extension_key = String::from("extensions");
                        let mut extension_ref = streak_post_map
                        .get(&extension_key)
                        .unwrap().to_owned();
                        extension_ref.insert(new_key.clone(), new_streak.clone());
                        */
                        /* DEBUG */println!("streak extension: {} - {}", &new_key, &new_streak)
                    }
                    //
                    // is it lower than last time?
                    else if new_streak < old_streak {
                        //
                        // if new data is less (lost streak)
                        /*
                        let loss_key = String::from("losses");
                        let mut loss_ref = streak_post_map
                        .get(&loss_key)
                        .unwrap().to_owned();
                        loss_ref.insert(new_key.clone(), old_streak.clone());
                        */
                        /* DEBUG */println!("streak loss: {} - {}", &new_key, &old_streak);
                        post()
                        
                    }
                    //
                    // did nothing change??????????
                    else {
                        //
                        // if they are neither greater nor less than eachother
                        println!("no change: {}", &new_key)
                    }
                }
                //
                // if we aren't comparing the same username
                else {
                    // ! Don't do shit because you're making an apple to orange comparison !
                }
            }
            //
            // if user wasn't tracked last time
            else {
                //
                // did we already add them to the new user list?
                if !already_fucking_checked.contains(&new_key) {
                    // add key to the restart post
                    println!("started tracking: {} - {}", &new_key, &new_streak);
                    already_fucking_checked.insert(0, new_key)
                }
                //
                // did we add them to no-no list?
                else {
                    // you already checked if that user wasn't in the list, idiot
                }
            }
        }
    }

    Ok(streak_post_map.clone())
}
