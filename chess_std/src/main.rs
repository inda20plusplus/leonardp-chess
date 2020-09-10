mod repl;
mod lib;

use repl::GameRepl;
use crate::lib::*;
use std::io::{self, BufRead, Result as IOResult};

type IOResultPlain = IOResult<()>;

fn main() -> IOResultPlain {

	let stdout = io::stdout();

	let stdin = io::stdin();
	let stdin = stdin.lock();
	let lines = stdin.lines().map(|x| x.expect("error reading input"));

	let game = Game::new_standard_game();

	let mut repl = GameRepl {
		game,
		stdout,
	};

	repl.connect(lines)?;

	Ok(())

	// let mut game = Game::new_5x5_empty();
	// game.add_pieces_from_str("Ra5 Kc5 Re5", game.player_black_index());
	// game.add_pieces_from_str("Pa4 Pb4 Pc4 Pd4 Pe4", game.player_black_index());
	// game.add_pieces_from_str("Ra1 Kd1 Re1", game.player_white_index());
	// println!("{}", game.board.print(BoardPrintStyle::ascii_pretty()));
	
	// game.current_player.color
	// game.state

	// game.perform_action(Action::CastlingA)
	// game.perform_action(Action::CastlingH)
	// game.perform_action(Action::Promotion {orign, target, kind: PieceKind})

	// Ok(())
}
