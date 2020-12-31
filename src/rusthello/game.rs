use super::board::*;
use super::game_status::*;
/// Manage an Othello game workflow
pub struct Game {
    board: Board,
    player: Option<Player>,
    opponent_is_blocked: bool,
    status: GameStatus,
}

impl Game {
    /// Create a new standard game
    pub fn new() -> Game {
        let board = Board::new_start();
        let mut game = Game {
            board: board,
            player: Some(Player::Black),
            opponent_is_blocked: false,
            status: Default::default(),
        };
        game.update_status();

        game
    }

    pub fn board(&self) -> &Board {
        &self.board
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
            self.update_player();
            Ok(())
        } else {
            Err("The move is invalid.".to_string())
        }
    }

    fn update_status(&mut self) {
        self.status = GameStatus::evaluate_board(&self.board);
    }

    fn update_player(&mut self) {
        if self.game_over() {
            self.player = None;
            return;
        }

        let mut player = self
            .player
            .expect("Unexpected None for the current player.");

        // As the game isn't over, at least one player can move, then
        // we don't need to check both cases.
        if self.status.can_player_move(player.opponent()) {
            player = player.opponent();
            self.opponent_is_blocked = false;
        } else {
            self.opponent_is_blocked = true;
        }
        self.player = Some(player);
    }

    pub fn player(&self) -> Option<Player> {
        self.player
    }

    pub fn opponent_is_blocked(&self) -> bool {
        self.opponent_is_blocked
    }

    pub fn game_over(&self) -> bool {
        self.status.game_over()
    }

    pub fn winner(&self) -> Option<Player> {
        self.status.winner()
    }

    pub fn count_pieces(&self) -> (u8, u8) {
        (
            self.status.pieces_count(Player::Black),
            self.status.pieces_count(Player::White),
        )
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
