use rusthello::{board_to_ascii, Game};
use std::io::{self, Write};

enum Choice {
    Quit,
    Move { x: u8, y: u8 },
}

fn main() {
    let mut game = Game::new();
    while !game.game_over() {
        let mut choice: Option<Choice> = None;
        let mut bad_response = false;
        while choice.is_none() || bad_response {
            println!();
            println!("{}", board_to_ascii(game.board()));
            display_game_status(&game);
            if bad_response {
                println!("Previous response was invalid, let try again.")
            }
            bad_response = false;
            choice = read_choice();

            match choice {
                None => bad_response = true,
                Some(Choice::Quit) => return,
                Some(Choice::Move { x, y }) => {
                    if let Err(_) = game.play(game.player().unwrap(), x, y) {
                        bad_response = true
                    }
                }
            }
        }
    }
    display_game_status(&game);
}

fn display_game_status(game: &Game) {
    let (black_pieces, white_pieces) = game.count_pieces();
    println!("Black {} - {} White", black_pieces, white_pieces);

    if game.game_over() {
        println!("The game is over !");
        if let Some(winner) = game.winner() {
            println!("And the winner is : {}.", winner);
        } else {
            println!("The game ends in a draw.");
        }
        return;
    }

    let player = game.player().expect("Unexpected None player");
    if game.opponent_is_blocked() {
        println!(
            "The turn does not chance as {} can't move.",
            player.opponent()
        );
    }

    println!("It's the turn of {}.", player);
}

fn read_choice() -> Option<Choice> {
    println!("What's you're move ? (ex : A1 ou Q to quit)");
    print!("> ");
    io::stdout().flush().unwrap();
    let response = read_string();

    parse_response(response)
}

fn parse_response(s: String) -> Option<Choice> {
    let s = s.to_uppercase();
    if s == "Q" {
        return Some(Choice::Quit);
    }

    if s.len() != 2 {
        return None;
    }
    let mut s_chars = s.chars();
    let x = s_chars.next().unwrap() as i8 - 65; // 'A' = 65
    let y = s_chars.next().unwrap() as i8 - 49; // '1' = 49

    if x < 0 || x > 7 || y < 0 || y > 7 {
        return None;
    }

    Some(Choice::Move {
        x: x as u8,
        y: y as u8,
    })
}

fn read_string() -> String {
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("Unable to read user input.");
    trim_newline(&mut s);

    s.trim().to_string()
}

fn trim_newline(s: &mut String) {
    while s.ends_with('\n') || s.ends_with('\r') {
        s.pop();
    }
}
