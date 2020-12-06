use crate::{Board, Player};

const ROW_REPARATOR: &str = "+---+---+---+---+---+---+---+---+\n";

/// Build an ascii representation of a board
pub fn board_to_ascii(board: &Board) -> String {
    let mut ascii = String::new();
    for y in 0..8 {
        ascii.push_str(ROW_REPARATOR);
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
        Some(Player::White) => "| O "
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn board_to_ascii_produce_ascii_representation_of_a_board() {
        let expected = "+---+---+---+---+---+---+---+---+\n\
                             |   |   |   |   |   |   |   |   |\n\
                             +---+---+---+---+---+---+---+---+\n\
                             |   |   |   |   |   |   |   |   |\n\
                             +---+---+---+---+---+---+---+---+\n\
                             |   |   |   |   |   |   |   |   |\n\
                             +---+---+---+---+---+---+---+---+\n\
                             |   |   |   | O | X |   |   |   |\n\
                             +---+---+---+---+---+---+---+---+\n\
                             |   |   |   | X | O |   |   |   |\n\
                             +---+---+---+---+---+---+---+---+\n\
                             |   |   |   |   |   |   |   |   |\n\
                             +---+---+---+---+---+---+---+---+\n\
                             |   |   |   |   |   |   |   |   |\n\
                             +---+---+---+---+---+---+---+---+\n\
                             |   |   |   |   |   |   |   |   |\n\
                             +---+---+---+---+---+---+---+---+\n";

        let board = Board::new_start();
        let ascii = board_to_ascii(&board);
        assert_eq!(ascii, expected);        
    }
}