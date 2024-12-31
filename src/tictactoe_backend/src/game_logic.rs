use crate::types::{Game, Player};

pub fn make_move(game: &mut Game, position: usize) -> Result<(), &'static str> {
    if position >= 9 {
        return Err("Invalid position");
    }

    if game.board[position].is_some() {
        return Err("Position already taken");
    }

    if game.winner.is_some() || game.is_draw {
        return Err("Game is already over");
    }

    game.board[position] = Some(game.current_player.clone());
    
    if let Some(winner) = check_winner(&game.board) {
        game.winner = Some(winner);
    } else if is_board_full(&game.board) {
        game.is_draw = true;
    } else {
        game.current_player = match game.current_player {
            Player::X => Player::O,
            Player::O => Player::X,
        };
    }

    Ok(())
}

fn check_winner(board: &[Option<Player>]) -> Option<Player> {
    const WINNING_LINES: [[usize; 3]; 8] = [
        [0, 1, 2], [3, 4, 5], [6, 7, 8], // Rows
        [0, 3, 6], [1, 4, 7], [2, 5, 8], // Columns
        [0, 4, 8], [2, 4, 6],            // Diagonals
    ];

    for line in WINNING_LINES.iter() {
        if let (Some(player), true) = (board[line[0]].clone(), 
            board[line[0]].is_some() && 
            board[line[0]] == board[line[1]] && 
            board[line[0]] == board[line[2]]) {
            return Some(player);
        }
    }
    None
}

fn is_board_full(board: &[Option<Player>]) -> bool {
    board.iter().all(|cell| cell.is_some())
}