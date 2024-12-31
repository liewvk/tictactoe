use candid::{Principal};
use ic_cdk::storage;
use ic_cdk_macros::*;
use std::cell::RefCell;
use std::collections::HashMap;

mod types;
mod game_logic;

use types::{Game, Player};
use game_logic::make_move;

thread_local! {
    static GAMES: RefCell<HashMap<Principal, Game>> = RefCell::new(HashMap::new());
}

#[update]
fn create_game() -> Game {
    let caller = ic_cdk::caller();
    let game = Game::default();
    GAMES.with(|games| {
        games.borrow_mut().insert(caller, game.clone());
    });
    game
}

#[update]
fn make_game_move(position: usize) -> Result<Game, String> {
    let caller = ic_cdk::caller();
    
    GAMES.with(|games| {
        let mut games = games.borrow_mut();
        let game = games.get_mut(&caller).ok_or("Game not found")?;
        
        make_move(game, position).map_err(|e| e.to_string())?;
        Ok(game.clone())
    })
}

#[query]
fn get_game() -> Option<Game> {
    let caller = ic_cdk::caller();
    GAMES.with(|games| games.borrow().get(&caller).cloned())
}