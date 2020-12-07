use rusthello;
use std::io::{self, Write};

fn main() {
    let board = rusthello::Board::new_start();
    io::stdout()
        .write(rusthello::board_to_ascii(&board).as_bytes())
        .unwrap();
}
