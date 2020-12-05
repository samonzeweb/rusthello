/// An Othello board.
pub struct Board {
    cells: [[Option<Player>; 8]; 8]
}

/// Players.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Player {
    Black,
    White
}

impl Board {
    /// Creates an empty board.
    pub fn new() -> Board {
        Board{
            cells: [[None; 8]; 8]
        }
    }

    /// Creates a new board ready to start a game
    pub fn new_start() -> Board {
        let mut board = Self::new();
        board.set_piece(3, 3, Some(Player::White)).unwrap();
        board.set_piece(4, 4, Some(Player::White)).unwrap();
        board.set_piece(3, 4, Some(Player::Black)).unwrap();
        board.set_piece(4, 3, Some(Player::Black)).unwrap();
        board
    }

    /// Set the content of a board cell
    pub fn set_piece(&mut self, x: usize, y: usize, piece: Option<Player>) -> Result<(), String> {
        self.check_coordonates(x, y)?;
        self.cells[x][y] = piece;
        Ok(())
    }

    //// Get the content of a board cell
    pub fn get_piece(&self, x: usize, y:usize) -> Result<Option<Player>, String> {
        self.check_coordonates(x, y)?;
        Ok(self.cells[x][y])
    }

    fn check_coordonates(&self, x: usize, y: usize) -> Result<(), String> {
        if x > 7 || y > 7 {
            Err(format!("the given coordinates are out of range"))
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_creates_empty_board() {
        let board = Board::new();
        board.cells.iter()
                .flatten()
                .for_each(|piece|  {
                    assert_eq!(piece.is_none(), true);
                })
    }

    #[test]
    fn new_start_creates_a_ready_to_play_board() {
        let board = Board::new_start();
        for (x, columns)  in board.cells.iter().enumerate() {
            for(y, piece) in columns.iter().enumerate() {
                if x < 3 || x > 4 || y < 3 || y > 4 {
                    assert_eq!(piece.is_none(), true);
                }
                else if x == y {
                    assert_eq!(*piece, Some(Player::White));
                }
                else {
                    assert_eq!(*piece, Some(Player::Black));
                }
            }
        }
    }

    #[test]
    fn set_piece() {
        let mut board = Board::new();
        board.set_piece(1,2, Some(Player::Black)).unwrap();
        assert_eq!(board.cells[1][2], Some(Player::Black))
    }

    #[test]
    fn get_piece() {
        let mut board = Board::new();
        board.cells[3][4] = Some(Player::White);
        let piece = board.get_piece(3,4).unwrap();
        assert_eq!(piece, Some(Player::White))
    }
}