use super::board::*;

/// GameStatus implement cross-cutting concerns about a game.
/// It's useful for the game workflow and virtual players implémentations.
#[derive(Default)]
pub struct GameStatus {
    black_can_move: bool,
    white_can_move: bool,
    black_pieces: u8,
    white_pieces: u8,
}

impl GameStatus {
    /// Build a GameStatus from a board.
    pub fn evaluate_board(board: &Board) -> Self {
        let mut black_can_move = false;
        let mut white_can_move = false;
        let (black_pieces, white_pieces) = board.count_pieces();
        if (black_pieces + white_pieces) != 64 {
            black_can_move = board.can_player_move(Player::Black);
            white_can_move = board.can_player_move(Player::White);
        }

        Self {
            black_can_move,
            white_can_move,
            black_pieces,
            white_pieces,
        }
    }

    /// Returns the count of pieces for the given player.
    pub fn pieces_count(&self, player: Player) -> u8 {
        match player {
            Player::Black => self.black_pieces,
            Player::White => self.white_pieces,
        }
    }
    /// Can a given player move ?
    pub fn can_player_move(&self, player: Player) -> bool {
        match player {
            Player::Black => self.black_can_move,
            Player::White => self.white_can_move,
        }
    }

    /// Is the game over ?
    pub fn game_over(&self) -> bool {
        !self.black_can_move && !self.white_can_move
    }

    /// Who won the game ?
    pub fn winner(&self) -> Option<Player> {
        if !self.game_over() || self.black_pieces == self.white_pieces {
            None
        } else {
            if self.black_pieces > self.white_pieces {
                Some(Player::Black)
            } else {
                Some(Player::White)
            }
        }
    }
}
