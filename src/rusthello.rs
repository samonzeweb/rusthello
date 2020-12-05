/// An Othello board.
pub struct Board {
    cells: [[Option<Player>; 8]; 8]
}

/// Players.
#[derive(Copy, Clone)]
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
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_creates_empty_board() {
        let board = Board::new();
        board.cells.iter()
                .flatten()
                .for_each(|content|  {
                    assert_eq!(content.is_none(), true);
                })
    }
}