use rusthello;
use std::io::{self, Write};

fn main() {
    let game = rusthello::Game::new();
    let board = game.board();
    io::stdout()
        .write(rusthello::board_to_ascii(board).as_bytes())
        .unwrap();
}
