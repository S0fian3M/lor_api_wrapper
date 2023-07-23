use std::collections::HashMap;
use lordeckcodes::{CardCodeAndCount, encoder};

use crate::card::Card;

/**
* A deck is a list of cards, some ids and a win rate.
*/
pub struct Deck {
    cards: HashMap<Card, i32>,
    deck_code: Option<String>,
    wins: i16,
    losses: i16,
}

impl Deck {
    pub fn new(cards: Vec<Card>, wins: i16, losses: i16) -> Self {
        let mut cards_in_deck = HashMap::new();
        for card in cards {
            let count = cards_in_deck.entry(card).or_insert(0);
            *count += 1;
        }

        let cards_as_lordeckcodes_format: Vec<CardCodeAndCount> = cards_in_deck.iter().map(|(card, count)| {
            CardCodeAndCount::from_data(&card.card_code.clone().unwrap(), *count).unwrap()
        }).collect();

        let deck_code = Some(encoder::code_from_deck(&lordeckcodes::Deck::from_vec(cards_as_lordeckcodes_format)).unwrap());
        
        Deck {
            cards: cards_in_deck,
            deck_code,
            wins,
            losses,
        }
    }
}

impl PartialEq for Deck {
    fn eq(&self, other: &Self) -> bool {
        self.deck_code == other.deck_code
    }
}
