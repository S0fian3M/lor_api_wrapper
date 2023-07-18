use std::error::Error;
use reqwest::blocking::{get, Response};
use log::info;

/**
* Wrapper about the current game status.
*/
#[derive(Debug)]
pub struct GameStatus {
    game_id: i32,
    is_winner: bool,
}

impl GameStatus {
    pub fn new(game_id: i32, is_winner: bool) -> Self {
        GameStatus { game_id, is_winner }
    }

    pub fn result(&self) -> &str {
        if self.game_id == -1 {
            "No games played"
        } else if self.is_winner {
            "Win"
        } else {
            "Loss"
        }
    }

    pub fn serialize(&self, to_dict: bool) -> String {
        let data = serde_json::json!({
            "game_id": self.game_id,
            "result": self.result()
        });

        if to_dict {
            serde_json::to_string_pretty(&data).unwrap()
        } else {
            data.to_string()
        }
    }
}

/**
* LoR local Client API.
*/
pub struct LoRClient {
    baseurl: String,
    api_key: String,
    port: u16,
}

impl LoRClient {
    const LOCALHOST_URL: &'static str = "http://localhost:";

    pub fn new(api_key: String, port: u16) -> Self {
        let baseurl = format!("{}{}", LoRClient::LOCALHOST_URL, port);
        LoRClient {
            baseurl,
            api_key,
            port,
        }
    }

    pub fn get_endpoint(&self, endpoint: &str) -> Result<serde_json::Value, Box<dyn Error>> {
        let url = format!("{}/{}", self.baseurl, endpoint);
        info!("Getting {}", endpoint);
        let response: Response = get(&url)?;

        let status = format!("{} - {}", response.status(), response.status().is_success());
        info!("Endpoint: {} response {}", endpoint, status);

        let json = response.json()?;
        Ok(json)
    }
}
