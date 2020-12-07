use std::fmt;

/// Othello players.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Player {
    Black,
    White,
}

impl Player {
    /// Returns the opponent of the player.
    pub fn opponent(self) -> Player {
        if self == Player::Black {
            Player::White
        } else {
            Player::Black
        }
    }
}

/// An Othello board, implementing moves.
/// Board does not implement game workflow.
#[derive(Debug, Copy, Clone)]
pub struct Board {
    cells: [[Option<Player>; 8]; 8],
}

impl Board {
    /// Creates an empty board.
    pub fn new() -> Board {
        Board {
            cells: [[None; 8]; 8],
        }
    }

    /// Creates a new board ready to start a game.
    pub fn new_start() -> Board {
        let mut board = Self::new();
        board.set_piece(3, 3, Some(Player::White)).unwrap();
        board.set_piece(4, 4, Some(Player::White)).unwrap();
        board.set_piece(3, 4, Some(Player::Black)).unwrap();
        board.set_piece(4, 3, Some(Player::Black)).unwrap();
        board
    }

    /// Sets the content of a board cell.
    pub fn set_piece(&mut self, x: u8, y: u8, piece: Option<Player>) -> Result<(), String> {
        Self::check_coordinates(x, y)?;
        self.cells[x as usize][y as usize] = piece;
        Ok(())
    }

    //// Gets the content of a board cell.
    pub fn get_piece(&self, x: u8, y: u8) -> Result<Option<Player>, String> {
        Self::check_coordinates(x, y)?;
        Ok(self.cells[x as usize][y as usize])
    }

    fn check_coordinates(x: u8, y: u8) -> Result<(), String> {
        if x > 7 || y > 7 {
            Err(format!(
                "the given coordinates are out of range : ({}, {})",
                x, y
            ))
        } else {
            Ok(())
        }
    }

    /// Play at the given position for the given player.
    /// If the move is valid a new Board is returned, else None.
    pub fn play(&self, player: Player, x: u8, y: u8) -> Result<Option<Board>, String> {
        Self::check_coordinates(x, y)?;

        // Only moves targeting empty cells are valids.
        if self.cells[x as usize][y as usize] != None {
            return Ok(None);
        }

        const ALL_DIRECTIONS: [(i8, i8); 8] = [
            (0, -1),
            (1, -1),
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
        ];

        // Explores the 8 possible directions and try to capture opponent pieces.
        // If at least one capture is possible, the move is valid.
        let mut new_board = self.clone();
        let other_player = player.opponent();
        let mut valid_move = false;
        for (_, direction) in ALL_DIRECTIONS.iter().enumerate() {
            let mut navigator = CellsNavigation::new((x, y), *direction).unwrap();
            let mut found_other_on_path = false;
            let mut can_capture = false;
            for position in &mut navigator {
                let piece = self.cells[position.0 as usize][position.1 as usize];
                match piece {
                    // Not a valid move.
                    None => break,
                    // Perhaps a valid move.
                    Some(p) if p == other_player => found_other_on_path = true,
                    // If player passes over opponent's pieces and reach a cell containing
                    // one of his pieces, he can capture opponent's pieces (hence it's a valid move).
                    Some(_) => {
                        can_capture = found_other_on_path;
                        break;
                    }
                }
            }

            // The current direction does not allow a capture.
            if !can_capture {
                continue;
            }

            // Let's capture opponent's pieces going backward.
            valid_move = true;
            navigator.reverse();
            for position in &mut navigator {
                // reverse iteration stop at move position
                if position == (x, y) {
                    break;
                }
                new_board.cells[position.0 as usize][position.1 as usize] = Some(player);
            }
        }

        if valid_move {
            new_board.cells[x as usize][y as usize] = Some(player);
            Ok(Some(new_board))
        } else {
            Ok(None)
        }
    }
}

impl fmt::Display for Board {
    /// Builds an ascii representation of the board. Not a fancy one,
    /// just enough to see what it looks like.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..=7 {
            for x in 0..=7 {
                let piece = self.get_piece(x, y).unwrap();
                let piece_representation = match piece {
                    None => " ",
                    Some(Player::Black) => "X",
                    Some(Player::White) => "O",
                };
                f.write_str(piece_representation)?;
            }
            f.write_str(".\n")?;
        }
        Ok(())
    }
}

/// Iterator to navigate from a start position upto the limit of a board.
/// The start position is excluded from the iteration.
/// The iterator can be reversed to go backward.
#[derive(Debug)]
struct CellsNavigation {
    current_position: (i8, i8),
    direction: (i8, i8),
}

impl CellsNavigation {
    fn new(start: (u8, u8), direction: (i8, i8)) -> Result<CellsNavigation, String> {
        let (x, y) = start;
        let (dx, dy) = direction;

        Board::check_coordinates(x, y)?;

        if !(-1..=1).contains(&dx) || !(-1..=1).contains(&dy) {
            return Err(format!(
                "the given direction is out of range : ({}, {})",
                dx, dy
            ));
        }

        Ok(CellsNavigation {
            current_position: (x as i8, y as i8),
            direction: direction,
        })
    }

    fn reverse(&mut self) {
        self.direction = (-self.direction.0, -self.direction.1);
    }
}

impl Iterator for CellsNavigation {
    type Item = (u8, u8);

    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = self.current_position;
        let (dx, dy) = self.direction;
        let (x, y) = (x + dx, y + dy);
        if x < 0 || x > 7 || y < 0 || y > 7 {
            None
        } else {
            self.current_position = (x, y);
            Some((x as u8, y as u8))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_creates_empty_board() {
        let board = Board::new();
        board.cells.iter().flatten().for_each(|piece| {
            assert_eq!(piece.is_none(), true);
        })
    }

    #[test]
    fn new_start_creates_a_ready_to_play_board() {
        let board = Board::new_start();
        for (x, columns) in board.cells.iter().enumerate() {
            for (y, piece) in columns.iter().enumerate() {
                if x < 3 || x > 4 || y < 3 || y > 4 {
                    assert_eq!(piece.is_none(), true);
                } else if x == y {
                    assert_eq!(*piece, Some(Player::White));
                } else {
                    assert_eq!(*piece, Some(Player::Black));
                }
            }
        }
    }

    #[test]
    fn set_piece() {
        let mut board = Board::new();
        board.set_piece(1, 2, Some(Player::Black)).unwrap();
        assert_eq!(board.cells[1][2], Some(Player::Black))
    }

    #[test]
    fn get_piece() {
        let mut board = Board::new();
        board.cells[3][4] = Some(Player::White);
        let piece = board.get_piece(3, 4).unwrap();
        assert_eq!(piece, Some(Player::White))
    }

    #[test]
    fn play_invalid_move_if_cell_not_empty() {
        let board = Board::new_start();
        // cell already occupied by a white piece
        let result_after_move = board.play(Player::Black, 3, 3).unwrap();
        assert!(result_after_move.is_none());
        // cell already occupied by a black piece
        let result_after_move = board.play(Player::Black, 3, 4).unwrap();
        assert!(result_after_move.is_none());
    }

    #[test]
    fn play_execute_simple_move() {
        let board = Board::new_start();
        let result_after_move = board.play(Player::Black, 4, 5).unwrap();
        assert!(result_after_move.is_some());
        let board_after_move = result_after_move.unwrap();
        assert_eq!(
            board_after_move.get_piece(4, 5).unwrap(),
            Some(Player::Black)
        );
        assert_eq!(
            board_after_move.get_piece(4, 4).unwrap(),
            Some(Player::Black)
        );
    }

    #[test]
    fn play_execute_move_capuring_pieces_in_all_directions() {
        // The test board is made of :
        // * an outer square of black pieces
        // * an inner square of white pieces
        // * an empty cell a the center of both squares, at position (2, 2)
        let mut board = Board::new();
        for x in 0..=4 {
            for y in 0..=4 {
                if x == 0 || x == 4 || y == 0 || y == 4 {
                    board.set_piece(x, y, Some(Player::Black)).unwrap()
                } else if x != 2 || y != 2 {
                    board.set_piece(x, y, Some(Player::White)).unwrap()
                }
            }
        }

        let result_after_move = board.play(Player::Black, 2, 2).unwrap();
        assert!(result_after_move.is_some());
        let board_after_move = result_after_move.unwrap();

        for x in 0..=7 {
            for y in 0..=7 {
                if x < 5 && y < 5 {
                    assert_eq!(
                        board_after_move.get_piece(x, y).unwrap(),
                        Some(Player::Black)
                    );
                } else {
                    assert_eq!(board_after_move.get_piece(x, y).unwrap(), None);
                }
            }
        }
    }

    #[test]
    fn fmt_build_a_board_representation() {
        let board = Board::new_start();
        let mut expected = String::new();
        let empty_line = "        .\n";
        expected.push_str(empty_line);
        expected.push_str(empty_line);
        expected.push_str(empty_line);
        expected.push_str("   OX   .\n");
        expected.push_str("   XO   .\n");
        expected.push_str(empty_line);
        expected.push_str(empty_line);
        expected.push_str(empty_line);
        assert_eq!(format!("{}", board), expected);
    }

    #[test]
    fn cell_navigation() {
        let mut cn = CellsNavigation::new((3, 3), (1, -1)).unwrap();
        assert_eq!(cn.next(), Some((4, 2)));
        assert_eq!(cn.next(), Some((5, 1)));
        assert_eq!(cn.next(), Some((6, 0)));
        assert_eq!(cn.next(), None);
    }

    #[test]
    fn cell_navigation_reverse() {
        let mut cn = CellsNavigation::new((3, 3), (1, -1)).unwrap();
        assert_eq!(cn.next(), Some((4, 2)));
        cn.reverse();
        assert_eq!(cn.next(), Some((3, 3)));
    }
}
