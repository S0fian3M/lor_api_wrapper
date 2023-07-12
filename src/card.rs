use std::path::Path;
use serde::Deserialize;
use serde_json::Value;

struct Card {
    id: Option<String>,
    card_code: String,
    card_set: u32,
    count: u32,
    _card_data: Value,
}

impl Card {
    fn new(card: Option<&str>, kwargs: &[(&str, Value)]) -> Self {
        let card_code = match card {
            Some(code) => code.to_string(),
            None => kwargs.iter().find_map(|(key, value)| {
                if key == &"CardCode" {
                    value.as_str().map(str::to_string)
                } else {
                    None
                }
            }).unwrap_or_else(String::new),
        };
        let card_set = card_code[..2].parse().unwrap_or(0);
        let count = kwargs.iter().find_map(|(key, value)| {
            if key == &"count" {
                value.as_u64().map(|count| count as u32)
            } else {
                None
            }
        }).unwrap_or(1);
        let _card_data = Self::card_info(&cards, &card_code);
        Self {
            id: None,
            card_code,
            card_set,
            count,
            _card_data,
        }
    }

    fn card_info(cards: &[Value], card_code: &str) -> Value {
        cards.iter()
            .find(|card| card["cardCode"] == card_code)
            .unwrap_or(&Value::Null)
            .clone()
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(cost) = self.cost() {
            write!(f, "({}) ", cost)?;
        }
        if let Some(name) = self.name() {
            write!(f, "{}: ", name)?;
        }
        if let Some(description) = self.description() {
            write!(f, "{}", description)?;
        }
        Ok(())
    }
}

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(name) = self.name() {
            write!(f, "Card({}, Name: {}, Cost: {})", self.card_code, name, self.cost().unwrap_or(0))
        } else {
            write!(f, "Card({}, Cost: {})", self.card_code, self.cost().unwrap_or(0))
        }
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