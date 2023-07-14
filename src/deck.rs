use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;

fn read_json_file(file_path: &Path) -> Result<HashMap<String, serde_json::Value>, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let data_globals: HashMap<String, serde_json::Value> = serde_json::from_str(&contents)?;
    Ok(data_globals)
}

fn write_json_file(data: &HashMap<String, serde_json::Value>, file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(file_path)?;
    let json_string = serde_json::to_string_pretty(data)?;
    file.write_all(json_string.as_bytes())?;
    Ok(())
}

struct Deck {
    cards: Vec<(Card, i32)>,
    deck_code: Option<String>,
    deck_id: Option<i32>,
    wins: i32,
    losses: i32,
}

impl Deck {
    fn new(kwargs: &HashMap<String, serde_json::Value>) -> Self {
        let cards_in_deck = kwargs.get("CardsInDeck").and_then(|cards| cards.as_object()).unwrap_or(&HashMap::new());
        let deck_code = kwargs.get("DeckCode").and_then(|code| code.as_str()).map(|s| s.to_owned());
        let deck_id = kwargs.get("deck_id").and_then(|id| id.as_i64()).map(|i| i as i32);
        let wins = kwargs.get("wins").and_then(|w| w.as_i64()).map(|i| i as i32).unwrap_or(0);
        let losses = kwargs.get("losses").and_then(|l| l.as_i64()).map(|i| i as i32).unwrap_or(0);
        
        let mut cards = Vec::new();
        
        for (card, amount) in cards_in_deck.iter() {
            if let (Some(card_code), Some(count)) = (card.as_str(), amount.as_i64()) {
                cards.push((Card::new(card_code.to_owned(), count as i32), count as i32));
            }
        }
        
        Deck {
            cards,
            deck_code,
            deck_id,
            wins,
            losses,
        }
    }
}
