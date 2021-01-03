# rusthello

This project is archived, it is now included in https://github.com/samonzeweb/rusthello-wasm .

## Purpose

Rusthello is an implementation of the Othello (or Reversi) game,
whose goal is to practice a little [Rust](https://www.rust-lang.org/).

It's a learning project, some parts are quick & dirty, tests are sometimes limited,
and some features are used for the sole purpose of... using them (`Cell`, feature flag, ...).

## Setup

Install Rust toolchain if needed : https://www.rust-lang.org/tools/install

Fetch the source code :

```
git clone https://github.com/samonzeweb/rusthello.git
cd rusthello
```

### Build and run

Run :

```
cargo build --release
```

The standalone executable will be here : `target/release/rusthello`.

To execute the game you have to give your color, and the depth of exploration for
the virtual player (deeper = slower), ie :

```
target/release/rusthello black 6
```

Usage :

```
Usage : ./rusthello color depth
  color : 'black' or 'white'
  depth : 4 .. 10 (more than 8 could be slow)
```

### Run in debug mode

Exemple :

```
cargo run -- black 6
```

### Test

Install the Rust toolchain and clone the repository as described in the build part.

Fast tests : `cargo test`

All tests (slower) : `cargo test --features alphabetavsminimax`

All tests, showing stats for the minimax vs alphabeta one : `cargo test --features alphabetavsminimax -- --nocapture`
