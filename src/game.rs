use lor_api_wrapper::card::Card;
use lor_api_wrapper::deck::Deck;

/**
* A rectangle is how cards are represented in the local API.
*/
struct Rectangle {
    card_id: i32,
    card_code: Option<String>,
    top_left_x: i32,
    top_left_y: i32,
    width: i32,
    height: i32,
    is_local_player: Option<bool>,
    card: Card,
}

impl Rectangle {
    fn new(rectangle_informations: &std::collections::HashMap<String, serde_json::Value>) -> Self {
        let card_id = rectangle_informations.get("CardID").and_then(|id| id.as_i64()).unwrap_or(0) as i32;
        let card_code = rectangle_informations.get("CardCode").and_then(|code| code.as_str()).map(|s| s.to_owned());
        let top_left_x = rectangle_informations.get("TopLeftX").and_then(|x| x.as_i64()).unwrap_or(0) as i32;
        let top_left_y = rectangle_informations.get("TopLeftY").and_then(|y| y.as_i64()).unwrap_or(0) as i32;
        let width = rectangle_informations.get("Width").and_then(|w| w.as_i64()).unwrap_or(0) as i32;
        let height = rectangle_informations.get("Height").and_then(|h| h.as_i64()).unwrap_or(0) as i32;
        let is_local_player = rectangle_informations.get("LocalPlayer").and_then(|lp| lp.as_bool());
        let card = Card::new(card_code.clone());
        
        Rectangle {
            card_id,
            card_code,
            top_left_x,
            top_left_y,
            width,
            height,
            is_local_player,
            card,
        }
    }
}

impl std::fmt::Debug for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rectangle(Card: {:?})", self.card_code)
    }
}

/**
* Screen
*/
struct Screen {
    width: i32,
    height: i32,
}

impl Screen {
    fn new(screen_informations: &std::collections::HashMap<String, serde_json::Value>) -> Self {
        let width = screen_informations.get("ScreenWidth").and_then(|w| w.as_i64()).unwrap_or(0) as i32;
        let height = screen_informations.get("ScreenHeight").and_then(|h| h.as_i64()).unwrap_or(0) as i32;
        
        Screen {
            width,
            height,
        }
    }
}

/**
* Game frame from local API.
*/
struct GameFrame {
    player: String,
    opponent: String,
    game_state: String,
    screen: Screen,
    rectangles: Vec<Rectangle>,
}

impl GameFrame {
    /**
    * Create a new game frame.
    */
    fn new(game_frame_informations: &std::collections::HashMap<String, serde_json::Value>) -> Self {
        let player = game_frame_informations.get("PlayerName").and_then(|name| name.as_str()).unwrap_or("The Man With No Name").to_owned();
        let opponent = game_frame_informations.get("OpponentName").and_then(|name| name.as_str()).unwrap_or("The Man With No Name").to_owned();
        let game_state = game_frame_informations.get("GameState").and_then(|state| state.as_str()).unwrap_or("Menus").to_owned();
        let screen = Screen::new(game_frame_informations.get("Screen").and_then(|s| s.as_object()).unwrap_or(&std::collections::HashMap::new()));
        let _rectangles = game_frame_informations.get("Rectangles").and_then(|rects| rects.as_array()).unwrap_or(&Vec::new());
        let rectangles = GameFrame::parse_rectangles(&_rectangles);
        
        GameFrame {
            player,
            opponent,
            game_state,
            screen,
            rectangles,
        }
    }
    
    /**
    * Parse rectangles values from the local API.
    */
    fn parse_rectangles(rectangles: &Vec<serde_json::Value>) -> Vec<Rectangle> {
        let rects = rectangles.iter().filter(|rect| rect["CardCode"] != "face");
        rects.map(|rect| Rectangle::new(rect.as_object().unwrap())).collect()
    }
    
    /**
    * Get local player rectangles.
    */
    fn player_rects(&self) -> Vec<&Rectangle> {
        self.rectangles.iter().filter(|rect| rect.is_local_player.unwrap_or(false)).collect()
    }
    
    /**
    * Get opponent player rectangles.
    */
    fn opponent_rects(&self) -> Vec<&Rectangle> {
        self.rectangles.iter().filter(|rect| !rect.is_local_player.unwrap_or(true)).collect()
    }
}

/**
* A game of LoR, between 2 players, with 2 decks, their cards and a result.
*/
struct Game {
    player: String,
    opponent: String,
    screen: Screen,
    player_cards_used: Deck,
    opponent_cards_used: Deck,
    initial_player_deck: Deck,
    current_player_deck: Deck,
    result: Option<serde_json::Value>,
}

impl Game {
    fn new(player: String, opponent: String, screen: Screen, player_deck: Deck) -> Self {
        Game {
            player,
            opponent,
            screen,
            player_cards_used: Deck::new(),
            opponent_cards_used: Deck::new(),
            initial_player_deck: player_deck.clone(),
            current_player_deck: player_deck,
            result: None,
        }
    }
    
    /**
    * Process a new game frame.
    */
    fn process_frame(&mut self, frame: &GameFrame) {
        for rect in frame.player_rects() {
            if !self.player_cards_used.cards.iter().any(|card| card.id == rect.card_id) {
                let current_card = Card::new(rect.card_code.clone());
                self.player_cards_used.add_card(current_card);
                // remove card from current player deck
            }
        }
        
        for rect in frame.opponent_rects() {
            if !self.opponent_cards_used.cards.iter().any(|card| card.id == rect.card_id) {
                let current_card = Card::new(rect.card_code.clone());
                self.opponent_cards_used.add_card(current_card);
            }
        }
    }
}

/**
* Expedition state deals with the expedition mode.
*/
struct ExpeditionState {
    is_active: bool,
    state: String,
    record: Vec<serde_json::Value>,
    draft_picks: Vec<serde_json::Value>,
    deck: Option<Deck>,
    games_played: i32,
    wins: i32,
    losses: i32,
}

impl ExpeditionState {
    /**
    * Create a new Expedition State.
    */
    fn new(expedition_state_informations: &std::collections::HashMap<String, serde_json::Value>) -> Self {
        let is_active = expedition_state_informations.get("IsActive").and_then(|active| active.as_bool()).unwrap_or(false);
        let state = expedition_state_informations.get("State").and_then(|s| s.as_str()).unwrap_or("Inactive").to_owned();
        let record = expedition_state_informations.get("Record").and_then(|r| r.as_array()).unwrap_or(&Vec::new()).to_owned();
        let draft_picks = expedition_state_informations.get("DraftPicks").and_then(|picks| picks.as_array()).unwrap_or(&Vec::new()).to_owned();
        let deck = None;
        let games_played = expedition_state_informations.get("Games").and_then(|games| games.as_i64()).unwrap_or(0) as i32;
        let wins = expedition_state_informations.get("Wins").and_then(|w| w.as_i64()).unwrap_or(0) as i32;
        let losses = expedition_state_informations.get("Losses").and_then(|l| l.as_i64()).unwrap_or(0) as i32;
        
        ExpeditionState {
            is_active,
            state,
            record,
            draft_picks,
            deck,
            games_played,
            wins,
            losses,
        }
    }
}

impl std::fmt::Debug for ExpeditionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Expedition(State: {}, Games Played: {})", self.state, self.games_played)
    }
}