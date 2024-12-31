use candid::Principal;
use ic_cdk_macros::*;
use std::cell::RefCell;
use std::collections::HashMap;

mod types;
mod game_logic;

use types::Game;
use game_logic::make_move;

// Thread-local storage for game instances associated with each caller
thread_local! {
    static GAMES: RefCell<HashMap<Principal, Game>> = RefCell::new(HashMap::new());
}

/// Creates a new game for the caller and returns the initial game state.
#[update]
fn create_game() -> Game {
    let caller = ic_cdk::caller(); // Get the caller's principal ID
    let game = Game::default();   // Create a default game instance
    GAMES.with(|games| {
        games.borrow_mut().insert(caller, game.clone()); // Store the game in the thread-local storage
    });
    game
}

/// Makes a move in the current game for the caller at the specified position.
/// Returns the updated game state or an error message.
#[update]
fn make_game_move(position: usize) -> Result<Game, String> {
    let caller = ic_cdk::caller();
    GAMES.with(|games| {
        let mut games = games.borrow_mut();
        let game = games.get_mut(&caller).ok_or_else(|| "Game not found".to_string())?;

        make_move(game, position).map_err(|e| e.to_string())?;
        Ok(game.clone())
    })
}

/// Retrieves the current game state for the caller.
/// Returns `None` if no game exists for the caller.
#[query]
fn get_game() -> Option<Game> {
    let caller = ic_cdk::caller();
    GAMES.with(|games| games.borrow().get(&caller).cloned())
}
