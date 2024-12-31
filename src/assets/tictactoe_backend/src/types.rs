use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)] // Ensure PartialEq is here
pub enum Player {
    X,
    O,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Game {
    pub board: Vec<Option<Player>>, // Represents the game board
    pub current_player: Player,     // Tracks the current player's turn
    pub winner: Option<Player>,     // Stores the winner, if any
    pub is_draw: bool,              // Indicates if the game is a draw
}

impl Default for Game {
    fn default() -> Self {
        Self {
            board: vec![None; 9],       // Initialize with an empty board
            current_player: Player::X, // Player X starts the game
            winner: None,              // No winner initially
            is_draw: false,            // No draw initially
        }
    }
}
