use rusthello::{AlphaBeta, Game, Player, VirtualPlayer, board_to_ascii};
use std::{
    char, env,
    io::{self, Write},
    process,
};

enum Choice {
    Quit,
    Move { x: u8, y: u8 },
}

fn main() {
    let (human, depth) = parge_args();
    let computer = Box::new(AlphaBeta::new(depth)) as Box<dyn VirtualPlayer>;

    let mut game = Game::new();
    while !game.game_over() {
        if game.player().unwrap() == human {
            let mut valid_move = false;
            while !valid_move {
                match get_choice_from_player(&game) {
                    Choice::Quit => return,
                    Choice::Move { x, y } => {
                        if let Ok(_) = game.play(game.player().unwrap(), x, y) {
                            valid_move = true
                        }
                    }
                }
            }
        } else {
            display_game_status(&game);
            println!("Computer is thinking...");
            let (x, y) = computer
                .compute_move(&game.board(), human.opponent())
                .expect("The computer can't produce a move.");
            game.play(human.opponent(), x, y).unwrap();
            println!("Computer played at {}", readable_coordinates(x, y));
        }
    }
    display_game_status(&game);
}

fn parge_args() -> (Player, u8) {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        print_usage_and_exit();
    }

    let player_str = args[1].trim().to_ascii_lowercase();
    let player = match player_str.as_str() {
        "black" => Player::Black,
        "white" => Player::White,
        _ => print_usage_and_exit(),
    };

    match args[2].parse::<u8>() {
        Ok(depth) => {
            if depth < 4 || depth > 10 {
                print_usage_and_exit();
            }
            return (player, depth);
        }
        Err(_) => {
            print_usage_and_exit();
        }
    };
}

fn print_usage_and_exit() -> ! {
    println!("Usage : {} color depth", env::args().nth(0).unwrap());
    println!("  color : 'black' or 'white'");
    println!("  depth : 4 .. 10 (more than 8 could be slow)");
    process::exit(1);
}

fn display_game_status(game: &Game) {
    println!("------------------------------------------------------------");
    println!("{}", board_to_ascii(game.board()));
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
            "The turn does not change as {} can't move.",
            player.opponent()
        );
    }

    println!("It's the turn of {}.", player);
}

fn readable_coordinates(x: u8, y: u8) -> String {
    let letter = char::from_u32('A' as u32 + x as u32).unwrap();
    let digit = y + 1;

    format!("({}, {})", letter, digit)
}

fn get_choice_from_player(game: &Game) -> Choice {
    let mut choice: Option<Choice> = None;
    let mut bad_response = false;
    while choice.is_none() || bad_response {
        display_game_status(&game);
        if bad_response {
            println!("Previous response was invalid, let try again.")
        }
        choice = read_choice();
        bad_response = choice.is_none();
    }

    choice.unwrap()
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
