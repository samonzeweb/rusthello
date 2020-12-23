use super::{Board, Player};

const ROW_REPARATOR: &str = "  +---+---+---+---+---+---+---+---+\n";
const LETTERS: &str = "    A   B   C   D   E   F   G   H\n";

/// Builds an ascii representation of a board.
pub fn board_to_ascii(board: &Board) -> String {
    let mut ascii = String::new();
    ascii.push_str(LETTERS);
    for y in 0..8 {
        ascii.push_str(ROW_REPARATOR);
        ascii.push_str(format!("{} ", y + 1).as_str());
        for x in 0..8 {
            let piece = board.get_piece(x, y).unwrap();
            ascii.push_str(cell_to_ascii(piece));
        }
        ascii.push_str("|\n")
    }
    ascii.push_str(ROW_REPARATOR);

    ascii
}

fn cell_to_ascii(piece: Option<Player>) -> &'static str {
    match piece {
        None => "|   ",
        Some(Player::Black) => "| X ",
        Some(Player::White) => "| O ",
    }
}

#[cfg(test)]
mod tests {
    use crate::rusthello;

    use super::*;
    use rusthello::Board;

    #[test]
    fn board_to_ascii_produce_ascii_representation_of_a_board() {
        // The dots aren't parts of the expected board representation.
        // They're purpose is to manage alignment, there are removed
        // before the comparison.
        let expected = "    A   B   C   D   E   F   G   H\n\
                             . +---+---+---+---+---+---+---+---+\n\
                             1 |   |   |   |   |   |   |   |   |\n\
                             . +---+---+---+---+---+---+---+---+\n\
                             2 |   |   |   |   |   |   |   |   |\n\
                             . +---+---+---+---+---+---+---+---+\n\
                             3 |   |   |   |   |   |   |   |   |\n\
                             . +---+---+---+---+---+---+---+---+\n\
                             4 |   |   |   | O | X |   |   |   |\n\
                             . +---+---+---+---+---+---+---+---+\n\
                             5 |   |   |   | X | O |   |   |   |\n\
                             . +---+---+---+---+---+---+---+---+\n\
                             6 |   |   |   |   |   |   |   |   |\n\
                             . +---+---+---+---+---+---+---+---+\n\
                             7 |   |   |   |   |   |   |   |   |\n\
                             . +---+---+---+---+---+---+---+---+\n\
                             8 |   |   |   |   |   |   |   |   |\n\
                             . +---+---+---+---+---+---+---+---+\n";

        let expected = expected.replace(".", " ");
        let board = Board::new_start();
        let ascii = board_to_ascii(&board);
        assert_eq!(ascii, expected);
    }
}
