use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use std::path::Path;

use lor_api_wrapper::{GameStatus, LoRClient};
mod utils;
mod card;

fn test_game_status() {
    let api_key = "your_api_key".to_string(); // TODO: .env file
    let port = 21337; // Port by default
    let client = LoRClient::new(api_key, port);

    let game_status = GameStatus::new(1, true);
    println!("Game Status: {:?}", game_status);

    let result = game_status.result();
    println!("Result: {}", result);

    let serialized_data = game_status.serialize(true);
    println!("Serialized Data:\n{}", serialized_data);

    let endpoint = "some_endpoint";
    match client.get_endpoint(endpoint) {
        Ok(json) => {
            println!("Response JSON:\n{}", serde_json::to_string_pretty(&json).unwrap());
        }
        Err(err) => {
            eprintln!("Error: {:?}", err);
        }
    }
}

// TODO: download automatically the sets of card you want
// + online REST API
fn main() {
    // Configure logging
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d} {l} - {m}{n}",
        )))
        .build();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(
            Root::builder()
                .appender("stdout")
                .build(LevelFilter::Info),
        )
        .unwrap();

    log4rs::init_config(config).unwrap();

    let all_cards: serde_json::Value = utils::read_json_file(Path::new("./data/set7b-en_us.json")).unwrap();
    let first_card: card::Card = card::Card::new(all_cards[0].clone());
    println!("{:?}", first_card);
    test_game_status();
}
