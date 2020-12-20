use super::board::*;
use super::game_status::*;

pub trait VirtualPlayer {
    fn compute_move(&self, board: &Board, me: Player) -> Option<(u8, u8)>;
}

struct BestMove {
    x: u8,
    y: u8,
    evaluation: i32,
}

pub struct Minimax {
    depth: u8,
}

impl Minimax {
    pub fn new(depth: u8) -> Self {
        Self { depth }
    }

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
                if depth == self.depth {
                    // max depth, juste evaluate and returns
                    let evaluation = Evaluator::evaluate(&board_after_move, current_player);
                    return Self::best_move_for_player(
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
                        return Self::best_move_for_player(
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
                return Self::best_move_for_player(
                    current_player,
                    best_move,
                    Some(BestMove { x, y, evaluation }),
                );
            }

            // it's not a valid move, just return the current best move.
            best_move
        })
    }

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

        let eval_a =
            Evaluator::sign_for_player(current_player, move_a.as_ref().unwrap().evaluation);
        let eval_b =
            Evaluator::sign_for_player(current_player, move_b.as_ref().unwrap().evaluation);
        return if eval_a > eval_b { move_a } else { move_b };
    }
}

impl VirtualPlayer for Minimax {
    fn compute_move(&self, board: &Board, me: Player) -> Option<(u8, u8)> {
        let best_move = self.inner_compute_move(board, me, 1);

        match best_move {
            Some(move_found) => Some((move_found.x, move_found.y)),
            None => None,
        }
    }
}

// TODO
// pub struct AlphaBeta;
// impl VirtualPlayer for AlphaBeta {}

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

    fn sign_for_player(player: Player, count: i32) -> i32 {
        match player {
            Player::Black => count,
            Player::White => -count,
        }
    }

    fn corner(x: u8, y: u8) -> bool {
        (x == 0 || x == 7) && (y == 0 || y == 7)
    }

    fn border(x: u8, y: u8) -> bool {
        x == 0 || x == 7 || y == 0 || y == 7
    }
}

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
}
