use serde_json::Value;
use serde::{Serialize, Deserialize};

/**
* A card with its metadata.
*/
#[derive(Debug, Deserialize, Serialize)]
pub struct Card {
    associated_cards: Option<Vec<String>>,
    associated_card_refs: Option<Vec<String>>,
    assets: Option<Vec<Asset>>,
    regions: Option<Vec<String>>,
    region_refs: Option<Vec<String>>,
    attack: Option<u32>,
    cost: Option<u32>,
    health: Option<u32>,
    description: Option<String>,
    description_raw: Option<String>,
    levelup_description: Option<String>,
    levelup_description_raw: Option<String>,
    flavor_text: Option<String>,
    artist_name: Option<String>,
    name: Option<String>,
    pub card_code: Option<String>,
    keywords: Option<Vec<String>>,
    keyword_refs: Option<Vec<String>>,
    spell_speed: Option<String>,
    spell_speed_ref: Option<String>,
    rarity: Option<String>,
    rarity_ref: Option<String>,
    subtypes: Option<Vec<String>>,
    supertype: Option<String>,
    card_type: Option<String>,
    collectible: Option<bool>,
    set: Option<String>,
    formats: Option<Vec<String>>,
    format_refs: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Asset {
    game_absolute_path: Option<String>,
    full_absolute_path: Option<String>,
}

impl Card {
    pub fn new(card_as_json: Value) -> Self {
        serde_json::from_value(card_as_json).unwrap()
    }

    pub fn card_info(cards: &[Value], card_code: &str) -> Value {
        cards.iter()
            .find(|card| card["card_code"] == card_code)
            .unwrap_or(&Value::Null)
            .clone()
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f, 
            "({}) {}: {}", 
            self.cost.unwrap_or(0), 
            self.name.as_ref().unwrap_or(&"".to_string()), 
            self.description.as_ref().unwrap_or(&"".to_string())
        )?;
        Ok(())
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.card_code == other.card_code
    }
}

impl Eq for Card {}

impl std::hash::Hash for Card {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.card_code.hash(state);
    }
}