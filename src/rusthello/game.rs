use super::board::*;

/// Manage an Othello game workflow
pub struct Game {
    board: Board,
    player: Option<Player>,
    status: GameStatus,
}

impl Game {
    /// Create a new standard game
    pub fn new() -> Game {
        let board = Board::new_start();
        Game {
            board: board,
            player: Some(Player::Black),
            status: GameStatus::evaluate_board(&board),
        }
    }

    pub fn play(&mut self, player: Player, x: u8, y: u8) -> Result<(), String> {
        match self.player {
            None => return Err("None of the players can move, the game is over.".to_string()),
            Some(p) if p != player => {
                return Err(format!("It's the turn of {}, not {}.", p, player))
            }
            _ => (),
        }
        let result = self.board.play(player, x, y)?;
        if let Some(new_board) = result {
            self.board = new_board;
            self.update_status();
            Ok(())
        } else {
            Err("The move is invalid.".to_string())
        }
    }

    fn update_status(&mut self) {
        self.status = GameStatus::evaluate_board(&self.board);
    }

    pub fn player(&self) -> Option<Player> {
        self.player
    }

    pub fn game_over(&self) -> bool {
        self.status.game_over()
    }

    pub fn winner(&self) -> Option<Player> {
        self.status.winner()
    }

    pub fn count_pieces(&self) -> (u8, u8) {
        (self.status.black_pieces, self.status.white_pieces)
    }
}

struct GameStatus {
    black_can_move: bool,
    white_can_move: bool,
    black_pieces: u8,
    white_pieces: u8,
}

impl GameStatus {
    fn evaluate_board(board: &Board) -> Self {
        let mut black_can_move = false;
        let mut white_can_move = false;
        let (black_pieces, white_pieces) = board.count_pieces();
        if (black_pieces + white_pieces) != 64 {
            black_can_move = Self::can_player_move(board, Player::Black);
            white_can_move = Self::can_player_move(board, Player::White);
        }

        Self {
            black_can_move,
            white_can_move,
            black_pieces,
            white_pieces,
        }
    }

    fn can_player_move(board: &Board, player: Player) -> bool {
        for (x, y) in GridIterator::new() {
            if board.play(player, x, y).unwrap().is_some() {
                return true;
            }
        }

        false
    }

    fn game_over(&self) -> bool {
        !self.black_can_move && !self.white_can_move
    }

    fn winner(&self) -> Option<Player> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_over_not_for_a_new_game() {
        let game = Game::new();
        assert!(!game.game_over())
    }

    #[test]
    fn game_over_if_all_cells_are_occupied() {
        let mut game = Game::new();
        for (x, y) in GridIterator::new() {
            game.board.set_piece(x, y, Some(Player::Black)).unwrap();
            game.update_status();
        }
        assert!(game.game_over());
    }

    #[test]
    fn game_over_if_none_of_the_players_can_move() {
        let mut game = Game::new();
        game.board = Board::new();
        game.board.set_piece(0, 0, Some(Player::Black)).unwrap();
        game.board.set_piece(7, 7, Some(Player::White)).unwrap();
        game.update_status();
        assert!(game.game_over());
    }

    #[test]
    fn no_winner_for_a_new_game() {
        let game = Game::new();
        assert!(game.winner().is_none())
    }

    #[test]
    fn no_winner_if_pieces_counts_are_equals() {
        let mut board_50_50 = Board::new();
        for (x, y) in GridIterator::new() {
            let piece = if x % 2 == 0 {
                Some(Player::Black)
            } else {
                Some(Player::White)
            };
            board_50_50.set_piece(x, y, piece).unwrap();
        }
        let mut game = Game::new();
        game.board = board_50_50;
        game.update_status();
        assert!(game.winner().is_none())
    }

    #[test]
    fn winner_if_no_one_can_move_and_one_has_more_pieces() {
        let mut unicolor_board = Board::new();
        for (x, y) in GridIterator::new() {
            unicolor_board.set_piece(x, y, Some(Player::Black)).unwrap();
        }
        let mut game = Game::new();
        game.board = unicolor_board;
        game.update_status();
        assert_eq!(game.winner(), Some(Player::Black));
    }

    #[test]
    fn count_pieces() {
        let game = Game::new();
        assert_eq!(game.count_pieces(), (2, 2));
    }
}
