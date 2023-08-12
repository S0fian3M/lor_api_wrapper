use reqwest;
use serde::{Deserialize, Serialize};

/**
* Download card set online.
*/
fn get_card_set_online(set_number: i32, region: &str, lite: bool) -> Result<serde_json::Value, reqwest::Error> {
    let url = if lite {
        format!(
            "http://dd.b.pvp.net/latest/set{}/{}/data/set{}-{}.zip",
            set_number, region, set_number, region
        )
    } else {
        format!(
            "http://dd.b.pvp.net/latest/set{}/{}/data/set{}-lite-{}.zip",
            set_number, region, set_number, region
        )
    }

    let response = reqwest::blocking::get(&url)?;
    Ok(response.json()?)
}

/**
* Get LOR globals from online API.
*/
fn get_globals_online(region: &str) -> Result<serde_json::Value, reqwest::Error> {
    let url = format!(
        "https://dd.b.pvp.net/latest/core/{}/data/globals-{}.zip",
        region, region
    );

    let response = reqwest::blocking::get(&url)?; 
    Ok(response.json()?)
}
