use crate::types::{Game, Player};

/// Makes a move on the board for the current player.
/// Returns an error if the move is invalid.
pub fn make_move(game: &mut Game, position: usize) -> Result<(), &'static str> {
    // Validate position
    if position >= 9 {
        return Err("Invalid position");
    }

    // Check if the position is already taken
    if game.board[position].is_some() {
        return Err("Position already taken");
    }

    // Check if the game is over
    if game.winner.is_some() || game.is_draw {
        return Err("Game is already over");
    }

    // Update the board with the current player's move
    game.board[position] = Some(game.current_player.clone());

    // Check for a winner or draw
    if let Some(winner) = check_winner(&game.board) {
        game.winner = Some(winner);
    } else if is_board_full(&game.board) {
        game.is_draw = true;
    } else {
        // Switch the current player
        game.current_player = match game.current_player {
            Player::X => Player::O,
            Player::O => Player::X,
        };
    }

    Ok(())
}

/// Checks if there is a winner on the board.
/// Returns the winning player if a winning condition is met.
fn check_winner(board: &[Option<Player>]) -> Option<Player> {
    const WINNING_LINES: [[usize; 3]; 8] = [
        [0, 1, 2], [3, 4, 5], [6, 7, 8], // Rows
        [0, 3, 6], [1, 4, 7], [2, 5, 8], // Columns
        [0, 4, 8], [2, 4, 6],            // Diagonals
    ];

    // Iterate over all winning lines
    for line in WINNING_LINES.iter() {
        if let Some(player) = board[line[0]] {
            if board[line[1]] == Some(player) && board[line[2]] == Some(player) {
                return Some(player);
            }
        }
    }

    None // No winner
}

/// Checks if the board is full.
/// Returns true if all positions are filled.
fn is_board_full(board: &[Option<Player>]) -> bool {
    board.iter().all(|cell| cell.is_some())
}
