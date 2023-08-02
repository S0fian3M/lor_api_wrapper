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
    /**
    * Create a new deck.
    */
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

    /**
    * Get the 2 regions of the deck.
    */
    pub fn regions(&self) -> Vec<String> {
        // Extract regions from cards
        let card_regions: Vec<Vec<String>> = self.cards.keys().map(|card| &card.regions.unwrap()).collect();

        // Count the occurrences of each region
        let mut region_count: HashMap<&String, usize> = HashMap::new();
        for region in card_regions.iter() {
            *region_count.entry(region).or_insert(0) += 1;
        }

        // Sort regions by frequency and take the top 2
        let mut card_regions_descending: Vec<(&&String, &usize)> = region_count.iter().collect();
        card_regions_descending.sort_by(|a, b| b.1.cmp(a.1));
        card_regions_descending.truncate(2);

        // Extract the region names and return them as Vec<String>
        let regions: Vec<String> = card_regions_descending.iter().map(|(region, _)| (*region).to_string()).collect();
        regions
    }

    /**
    * Get the list of Champions cards.
    */
    pub fn get_champions(&self) -> Vec<&Card> {
        // Filter champions from cards and extract their names
        self.cards.keys().filter(|card| card.rarity.unwrap() == "Champion").collect()
    }
}

impl PartialEq for Deck {
    fn eq(&self, other: &Self) -> bool {
        self.deck_code == other.deck_code
    }
}
