use std::{cell::Cell, cmp};

use super::board::*;
use super::game_status::*;

/// The VirtualPlayer trait standardize the public interface of algorithms to
/// find moves (virtual player, move suggestion, ...).
pub trait VirtualPlayer {
    /// Returns the 'best move' the given board and player.
    fn compute_move(&self, board: &Board, me: Player) -> Option<(u8, u8)>;

    /// Returns the total count of move while exploring tree game.
    fn move_count(&self) -> u32;
}

/// Implementation of the MiniMax algorithm.
pub struct Minimax {
    depth: u8,
    move_count: Cell<u32>,
}

impl Minimax {
    /// Creates a new MiniMax with, fixing its exploration depth.
    pub fn new(depth: u8) -> Self {
        Self { depth, move_count: Cell::new(0) }
    }

    /// Minimax implementation.
    fn inner_compute_move(
        &self,
        board: &Board,
        current_player: Player,
        depth: u8,
    ) -> Option<BestMove> {
        GridIterator::new().fold(None, |best_move, (x, y)| {
            let opt_board_after_move = board
                .play(current_player, x, y)
                .expect("Unexpected error while computing move.");

            // is the move valid ?
            if let Some(board_after_move) = opt_board_after_move {
                self.move_count.set(self.move_count() + 1);
                if depth == self.depth {
                    // max depth, just evaluate and returns
                    let evaluation = Evaluator::evaluate(&board_after_move, current_player);
                    return BestMove::best_move_for_player(
                        current_player,
                        best_move,
                        Some(BestMove { x, y, evaluation }),
                    );
                }

                // determine the next player, and check if the game is blocked.
                let next_player = if board_after_move.can_player_move(current_player.opponent()) {
                    // the player changes.
                    current_player.opponent()
                } else {
                    if board_after_move.can_player_move(current_player) {
                        // the game is not blocked, but the player does not change.
                        current_player
                    } else {
                        // the game is blocked.
                        let evaluation = Evaluator::evaluate(&board_after_move, current_player);
                        return BestMove::best_move_for_player(
                            current_player,
                            best_move,
                            Some(BestMove { x, y, evaluation }),
                        );
                    }
                };

                let inner_best_move = self
                    .inner_compute_move(&board_after_move, next_player, depth + 1)
                    .unwrap();
                let BestMove {
                    x: _,
                    y: _,
                    evaluation,
                } = inner_best_move;
                return BestMove::best_move_for_player(
                    current_player,
                    best_move,
                    Some(BestMove { x, y, evaluation }),
                );
            }

            // it's not a valid move, just return the current best move.
            best_move
        })
    }
}

impl VirtualPlayer for Minimax {
    fn move_count(&self) -> u32 {
        self.move_count.get()
    }

    fn compute_move(&self, board: &Board, me: Player) -> Option<(u8, u8)> {
        let best_move = self.inner_compute_move(board, me, 1);

        match best_move {
            Some(move_found) => Some((move_found.x, move_found.y)),
            None => None,
        }
    }
}

/// Implementation of the Alpha-Beta algorithm.
pub struct AlphaBeta {
    depth: u8,
    move_count: Cell<u32>,
}

impl AlphaBeta {
    /// Creates a new AlphaBeta with, fixing its exploration depth.
    pub fn new(depth: u8) -> Self {
        Self { depth, move_count: Cell::new(0) }
    }

    /// Alpha-Beta implementation.
    fn inner_compute_move(
        &self,
        board: &Board,
        current_player: Player,
        depth: u8,
        alpha: i32,
        beta: i32

    ) -> Option<BestMove> {
        let mut best_move = None;
        let mut current_alpha = alpha;
        let mut current_beta = beta;
        for (x, y) in GridIterator::new() {
            let opt_board_after_move = board
                .play(current_player, x, y)
                .expect("Unexpected error while computing move.");

            // is the move valid ?
            if let Some(board_after_move) = opt_board_after_move {
                self.move_count.set(self.move_count() + 1);
                if depth == self.depth {
                    // max depth, just evaluate and returns
                    let evaluation = Evaluator::evaluate(&board_after_move, current_player);
                    best_move = BestMove::best_move_for_player(
                        current_player,
                        best_move,
                        Some(BestMove { x, y, evaluation }),
                    );
                    continue;
                }

                // determine the next player, and check if the game is blocked.
                let next_player = if board_after_move.can_player_move(current_player.opponent()) {
                    // the player changes.
                    current_player.opponent()
                } else {
                    if board_after_move.can_player_move(current_player) {
                        // the game is not blocked, but the player does not change.
                        current_player
                    } else {
                        // the game is blocked.
                        let evaluation = Evaluator::evaluate(&board_after_move, current_player);
                        best_move = BestMove::best_move_for_player(
                            current_player,
                            best_move,
                            Some(BestMove { x, y, evaluation }),
                        );
                        continue;
                    }
                };

                let inner_best_move = self
                    .inner_compute_move(&board_after_move, next_player, depth + 1, current_alpha, current_beta)
                    .unwrap();
                let BestMove {
                    x: _,
                    y: _,
                    evaluation,
                } = inner_best_move;
                best_move = BestMove::best_move_for_player(
                    current_player,
                    best_move,
                    Some(BestMove { x, y, evaluation }),
                );
                let best_eval = best_move.as_ref().unwrap().evaluation;
                if current_player == Player::Black {
                    if best_eval >= beta {
                        // beta cut
                        return best_move;
                    }
                    current_alpha = cmp::max(current_alpha, best_eval);
                } else {
                    if best_eval <= alpha {
                        // alpha cut
                        return best_move;
                    }
                    current_beta = cmp::min(current_beta, best_eval);
                }
            }
        }

        return best_move;
    }
}

impl VirtualPlayer for AlphaBeta {
    fn move_count(&self) -> u32 {
        self.move_count.get()
    }

    fn compute_move(&self, board: &Board, me: Player) -> Option<(u8, u8)> {
        let best_move = self.inner_compute_move(board, me, 1, i32::MIN,i32::MAX);

        match best_move {
            Some(move_found) => Some((move_found.x, move_found.y)),
            None => None,
        }
    }
}

/// Evaluator is responsible for the evaluation of the state of a game.
/// No instance is needed, all methods are statics. Evaluator could become
/// configurable later, but now it's rather a naive implementation.
struct Evaluator;

impl Evaluator {
    // game is over and there is a winner.
    const SCORE_MAX: i32 = i32::MAX;
    // game over and no winner.
    const SCORE_DRAW: i32 = 0;
    // bonus if the opponent can't move the next turn.
    const SCORE_OPPONENT_BLOCKED: i32 = 4;

    // Scores according to piece position.
    const SCORE_INSIDE: i32 = 1;
    const SCORE_BORDER: i32 = 4;
    const SCORE_CORNER: i32 = 8;

    /// Returns an evaluation for the given board, when the last move was done
    /// by the given player.
    /// If the evaluation is ...
    /// * positive : Black player is stronger.
    /// * negative : White player is stronger.
    fn evaluate(board: &Board, last_player: Player) -> i32 {
        let status = GameStatus::evaluate_board(board);
        if status.game_over() {
            return match status.winner() {
                Some(winner) => Self::sign_for_player(winner, Self::SCORE_MAX),
                None => Self::SCORE_DRAW,
            };
        }

        let mut corner = 0;
        let mut border = 0;
        let mut other = 0;
        for (x, y, piece) in board.iter() {
            if let Some(player) = piece {
                if Self::corner(x, y) {
                    corner += Self::sign_for_player(player, Self::SCORE_CORNER);
                } else if Self::border(x, y) {
                    border += Self::sign_for_player(player, Self::SCORE_BORDER);
                } else {
                    other += Self::sign_for_player(player, Self::SCORE_INSIDE);
                }
            }
        }

        let mut evaluation = corner + border + other;

        if !status.can_player_move(last_player.opponent()) {
            evaluation += Self::sign_for_player(last_player, Self::SCORE_OPPONENT_BLOCKED);
        }

        evaluation
    }

    /// Change the sign if the given evaluation (or intermediate one) if the
    /// player is White.
    fn sign_for_player(player: Player, evaluation: i32) -> i32 {
        match player {
            Player::Black => evaluation,
            Player::White => -evaluation,
        }
    }

    fn corner(x: u8, y: u8) -> bool {
        (x == 0 || x == 7) && (y == 0 || y == 7)
    }

    fn border(x: u8, y: u8) -> bool {
        x == 0 || x == 7 || y == 0 || y == 7
    }
}

/// BestMove is in internal structure to retuens best move found during
/// game tree exploration.
struct BestMove {
    x: u8,
    y: u8,
    evaluation: i32,
}

impl BestMove {
    /// Choose the best move between the two given, for the given player.
    fn best_move_for_player(
        current_player: Player,
        move_a: Option<BestMove>,
        move_b: Option<BestMove>,
    ) -> Option<BestMove> {
        if move_a.is_none() {
            return move_b;
        }
        if move_b.is_none() {
            return move_a;
        }

        let eval_a = move_a.as_ref().unwrap().normalized_evaluation(current_player);
        let eval_b = move_b.as_ref().unwrap().normalized_evaluation(current_player);
        return if eval_a >= eval_b { move_a } else { move_b };
    }

    /// Returns an evaluation, normalized to be 'greater is better' for the player.
    fn normalized_evaluation(&self, player: Player) -> i32 {
        Evaluator::sign_for_player(player, self.evaluation)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn evaluate_returns_zero_for_equals_forces() {
        let board = Board::new_start();
        assert_eq!(0, Evaluator::evaluate(&board, Player::Black));
    }

    #[test]
    fn evaluate_returns_positive_score_if_black_is_stronger() {
        let board = Board::new_start();
        let board = board.play(Player::Black, 4, 5).unwrap().unwrap();
        assert!(Evaluator::evaluate(&board, Player::Black) > 0);
    }

    #[test]
    fn evaluate_returns_negative_score_if_white_is_stronger() {
        let mut board = Board::new_start();
        board.set_piece(3, 4, Some(Player::White)).unwrap();
        assert!(Evaluator::evaluate(&board, Player::Black) < 0);
    }

    #[test]
    fn minimax_find_a_move() {
        let board = Board::new_start();
        let minimax = Minimax::new(4);
        let best_move = minimax.compute_move(&board, Player::Black);
        assert!(best_move.is_some());
    }

    #[test]
    fn minimax_find_the_best_move() {
        let mut board = Board::new();
        board.set_piece(2, 2, Some(Player::White)).unwrap();
        board.set_piece(3, 2, Some(Player::Black)).unwrap();
        board.set_piece(2, 3, Some(Player::White)).unwrap();
        board.set_piece(3, 3, Some(Player::Black)).unwrap();
        board.set_piece(4, 3, Some(Player::Black)).unwrap();
        let minimax = Minimax::new(1);
        let best_move = minimax.compute_move(&board, Player::White);
        assert_eq!(best_move, Some((5, 3)));
    }

    #[test]
    fn alphabeta_find_a_move() {
        let board = Board::new_start();
        let alphabeta = AlphaBeta::new(4);
        let best_move = alphabeta.compute_move(&board, Player::Black);
        assert!(best_move.is_some());
    }

    #[test]
    fn alphabeta_find_the_best_move() {
        let mut board = Board::new();
        board.set_piece(2, 2, Some(Player::White)).unwrap();
        board.set_piece(3, 2, Some(Player::Black)).unwrap();
        board.set_piece(2, 3, Some(Player::White)).unwrap();
        board.set_piece(3, 3, Some(Player::Black)).unwrap();
        board.set_piece(4, 3, Some(Player::Black)).unwrap();
        let alphabeta = AlphaBeta::new(1);
        let best_move = alphabeta.compute_move(&board, Player::White);
        assert_eq!(best_move, Some((5, 3)));
    }

    /// This test take more time and is only done when the feature flag is activated.
    /// Disabling capture show each 'best' move found, and the move counts per
    /// algorithms.
    #[cfg(feature="alphabetavsminimax")]
    #[test]
    fn alpha_beta_behave_the_same_as_minimax() {
        use super::super::Game;
        let mut game = Game::new();
        let minimax = Minimax::new(4);
        let alpha_beta = AlphaBeta::new(4);
        while !game.game_over() {
            // compore computed moves for this turn.
            let minimax_result = minimax.compute_move(game.board(), game.player().unwrap());
            let alphabeta_result = alpha_beta.compute_move(game.board(), game.player().unwrap());
            assert_eq!(minimax_result, alphabeta_result);
            // play the move... et continue the game
            println!("Move : {:?} / move counts : minimax {} - {} alphabeta", alphabeta_result, minimax.move_count(), alpha_beta.move_count());
            match alphabeta_result {
                Some((x,y)) => game.play(game.player().unwrap(), x, y),
                None => panic!("Unexpected empty move."),
            }.unwrap();
        }
    }
}
