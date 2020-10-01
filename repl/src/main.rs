mod repl;

use chess_engine::Game;
use repl::GameRepl;
use std::io::{self, BufRead, Result as IOResult};

type IOResultPlain = IOResult<()>;

fn main() -> IOResultPlain {
    let stdout = io::stdout();

    let stdin = io::stdin();
    let stdin = stdin.lock();
    let lines = stdin.lines().map(|x| x.expect("error reading input"));

    let game = Game::new_standard_game();

    let mut repl = GameRepl::new(game, stdout);
    repl.clear_screen = std::env::args().any(|x| x == "--clear-screen");

    repl.connect(lines)?;

    Ok(())
}
