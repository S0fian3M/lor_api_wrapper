use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use lor_api_wrapper::Card;

/**
* A deck is a pile of cards, some ids and a win rate.
*/
struct Deck {
    cards: Vec<(Card, i32)>,
    deck_code: Option<String>,
    deck_id: Option<i32>,
    wins: i32,
    losses: i32,
}

impl Deck {
    fn new(deck_informations: &HashMap<String, serde_json::Value>) -> Self {
        let cards_in_deck = deck_informations.get("CardsInDeck").and_then(|cards| cards.as_object()).unwrap_or(&HashMap::new());
        let deck_code = deck_informations.get("DeckCode").and_then(|code| code.as_str()).map(|s| s.to_owned());
        let deck_id = deck_informations.get("deck_id").and_then(|id| id.as_i64()).map(|i| i as i32);
        let wins = deck_informations.get("wins").and_then(|w| w.as_i64()).map(|i| i as i32).unwrap_or(0);
        let losses = deck_informations.get("losses").and_then(|l| l.as_i64()).map(|i| i as i32).unwrap_or(0);
        
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
