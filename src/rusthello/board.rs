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

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Player::Black => f.write_str("Black"),
            Player::White => f.write_str("White"),
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

    /// Returns an iterator on the board.
    /// The iterator will returns all cells positions and their contents.
    pub fn iter(self: &Board) -> BoardIterator {
        BoardIterator::new(self)
    }

    /// All possible directions to capture opponent pieces.
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

    /// Checks if the given player can move to the given coordinates.
    /// It's faster than play as it does just the bare minimum.
    pub fn is_move_valid(&self, player: Player, x: u8, y: u8) -> Result<bool, String> {
        Self::check_coordinates(x, y)?;

        // Only moves targeting empty cells are valids.
        if self.cells[x as usize][y as usize] != None {
            return Ok(false);
        }

        let other_player = player.opponent();

        for direction in Self::ALL_DIRECTIONS.iter() {
            if let Some(_) = self.can_capture(other_player, x, y, *direction) {
                return Ok(true);
            }
        }

        // No capture is possible in any direction : invalid move
        Ok(false)
    }

    /// Checks if a capture is possible for a given move and a given direction.
    /// Returns a CellsNavigator ready to capture all opponent pieces backward.
    fn can_capture(
        &self,
        opponent: Player,
        x: u8,
        y: u8,
        direction: (i8, i8),
    ) -> Option<CellsNavigator> {
        let mut navigator = CellsNavigator::new((x, y), direction).unwrap();
        let mut found_other_on_path = false;
        let mut can_capture = false;
        for position in &mut navigator {
            let piece = self.cells[position.0 as usize][position.1 as usize];
            match piece {
                // Not a valid move.
                None => break,
                // Perhaps a valid move.
                Some(p) if p == opponent => found_other_on_path = true,
                // If player passes over opponent's pieces and reach a cell containing
                // one of his pieces, he can capture opponent's pieces (hence it's a valid move).
                Some(_) => {
                    can_capture = found_other_on_path;
                    break;
                }
            }
        }
        if can_capture {
            navigator.reverse();
            return Some(navigator);
        }

        None
    }

    /// Plays at the given position for the given player.
    /// If the move is valid a new Board is returned, else None.
    pub fn play(&self, player: Player, x: u8, y: u8) -> Result<Option<Board>, String> {
        Self::check_coordinates(x, y)?;

        // Only moves targeting empty cells are valids.
        if self.cells[x as usize][y as usize] != None {
            return Ok(None);
        }

        // Explores the 8 possible directions and try to capture opponent pieces.
        // If at least one capture is possible, the move is valid.
        let mut new_board = self.clone();
        let other_player = player.opponent();
        let mut valid_move = false;
        for direction in Self::ALL_DIRECTIONS.iter() {
            if let Some(navigator) = self.can_capture(other_player, x, y, *direction) {
                // Let's capture opponent's pieces going backward.
                valid_move = true;
                for position in navigator {
                    // reverse iteration stop at move position
                    if position == (x, y) {
                        break;
                    }
                    new_board.cells[position.0 as usize][position.1 as usize] = Some(player);
                }
            }
        }

        if valid_move {
            new_board.cells[x as usize][y as usize] = Some(player);
            Ok(Some(new_board))
        } else {
            Ok(None)
        }
    }

    /// Cheks if a given player can move in at least one position.
    pub fn can_player_move(&self, player: Player) -> bool {
        for (x, y) in GridIterator::new() {
            let can_move = self.is_move_valid(player, x, y).unwrap();
            if can_move {
                return true;
            }
        }

        false
    }

    /// Count the pieces on the board.
    /// It returns a tuple with black pieces count as the first item,
    /// and white pieces count as the second.
    pub fn count_pieces(&self) -> (u8, u8) {
        let mut black_pieces = 0;
        let mut white_pieces = 0;
        for (_, _, piece) in self.iter() {
            match piece {
                Some(Player::Black) => black_pieces += 1,
                Some(Player::White) => white_pieces += 1,
                _ => (),
            }
        }

        (black_pieces, white_pieces)
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

/// Implements an iterator on the board wich returns
/// each position of the board and its content.
#[derive(Debug)]
pub struct BoardIterator<'a> {
    board: &'a Board,
    grid_iterator: GridIterator,
}

impl<'a> BoardIterator<'a> {
    fn new(board: &'a Board) -> Self {
        BoardIterator {
            board,
            grid_iterator: GridIterator::new(),
        }
    }
}

impl Iterator for BoardIterator<'_> {
    type Item = (u8, u8, Option<Player>);

    fn next(&mut self) -> Option<Self::Item> {
        let current_position = self.grid_iterator.next();
        match current_position {
            None => None,
            Some((x, y)) => Some((x, y, self.board.get_piece(x, y).unwrap())),
        }
    }
}

/// An iterator over a 8x8 grid
#[derive(Debug)]
pub struct GridIterator {
    x: u8,
    y: u8,
}

impl GridIterator {
    pub fn new() -> Self {
        GridIterator { x: 0, y: 0 }
    }
}

impl Iterator for GridIterator {
    type Item = (u8, u8);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y > 7 {
            return None;
        }

        let item: Self::Item = (self.x, self.y);
        self.x += 1;
        if self.x > 7 {
            self.x = 0;
            self.y += 1;
        }

        Some(item)
    }
}

/// Iterator to navigate from a start position upto the limit of a board in
/// a given direction.
/// The start position is excluded from the iteration.
/// The iterator can be reversed to go backward.
#[derive(Debug)]
struct CellsNavigator {
    current_position: (i8, i8),
    direction: (i8, i8),
}

impl CellsNavigator {
    fn new(start: (u8, u8), direction: (i8, i8)) -> Result<CellsNavigator, String> {
        let (x, y) = start;
        let (dx, dy) = direction;

        Board::check_coordinates(x, y)?;

        if !(-1..=1).contains(&dx) || !(-1..=1).contains(&dy) {
            return Err(format!(
                "the given direction is out of range : ({}, {})",
                dx, dy
            ));
        }

        Ok(CellsNavigator {
            current_position: (x as i8, y as i8),
            direction: direction,
        })
    }

    fn reverse(&mut self) {
        self.direction = (-self.direction.0, -self.direction.1);
    }
}

impl Iterator for CellsNavigator {
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
    fn bord_iterator_returns_cells_contents() {
        let mut board = Board::new();
        board.set_piece(1, 0, Some(Player::Black)).unwrap();
        let mut iterator = board.iter();
        assert_eq!(iterator.next(), Some((0, 0, None)));
        assert_eq!(iterator.next(), Some((1, 0, Some(Player::Black))));
    }

    #[test]
    fn bord_iterator_iterate_over_all_cells() {
        let board = Board::new();
        let (mut i, mut j) = (0, 0);
        let mut count = 0;
        for (x, y, _) in board.iter() {
            count += 1;
            assert_eq!(x, i);
            assert_eq!(y, j);
            i += 1;
            if i > 7 {
                i = 0;
                j += 1;
            }
        }
        assert_eq!(count, 64);
    }

    #[test]
    fn is_move_valid_returns_none_for_non_empty_cell() {
        let board = Board::new_start();
        // cell already occupied by a white piece
        let is_valid = board.is_move_valid(Player::Black, 3, 3).unwrap();
        assert!(!is_valid);
        // cell already occupied by a black piece
        let is_valid = board.is_move_valid(Player::Black, 3, 4).unwrap();
        assert!(!is_valid);
    }

    #[test]
    fn is_valid_move_returns_true_for_a_valid_one() {
        let board = Board::new_start();
        let is_valid = board.is_move_valid(Player::Black, 4, 5).unwrap();
        assert!(is_valid);
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
    fn count_players_pieces() {
        let mut board = Board::new_start();
        board.set_piece(0, 0, Some(Player::White)).unwrap();
        let (black, white) = board.count_pieces();
        assert_eq!(black, 2);
        assert_eq!(white, 3)
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
    fn grid_iterator_generates_all_coordonates() {
        let mut cells = [false; 64];
        let grid_iterator = GridIterator::new();
        for (x, y) in grid_iterator {
            cells[(x + y * 8) as usize] = true;
        }
        assert!(cells.iter().all(|flag| *flag));
    }

    #[test]
    fn cell_navigation() {
        let mut cn = CellsNavigator::new((3, 3), (1, -1)).unwrap();
        assert_eq!(cn.next(), Some((4, 2)));
        assert_eq!(cn.next(), Some((5, 1)));
        assert_eq!(cn.next(), Some((6, 0)));
        assert_eq!(cn.next(), None);
    }

    #[test]
    fn cell_navigation_reverse() {
        let mut cn = CellsNavigator::new((3, 3), (1, -1)).unwrap();
        assert_eq!(cn.next(), Some((4, 2)));
        cn.reverse();
        assert_eq!(cn.next(), Some((3, 3)));
    }
}
