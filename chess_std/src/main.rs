mod lib;
use lib::*;

fn main() -> Result<(), String> {
	let mut game = Game::new();
	println!("{}", game.board.print(BoardPrintStyle::ascii_pretty()));
	
	// game.current_player.color
	// game.state

	// game.perform_action(Action::CastlingA)
	// game.perform_action(Action::CastlingH)
	// game.perform_action(Action::Promotion {orign, target, kind: PieceKind})
	
	game.perform_action(game.move_from_str("b1 c3")?)?;
	game.perform_action(game.move_from_str("b8 a6")?)?;
	game.perform_action(game.move_from_str("c3 e4")?)?;
	game.perform_action(game.move_from_str("a6 b4")?)?;
	game.perform_action(game.move_from_str("e4 g5")?)?;
	game.perform_action(game.move_from_str("b4 c2")?)?;
	// game.perform_action(game.move_from_str("g5 f7")?)?;
	println!("{}", game.board.print(BoardPrintStyle::ascii_pretty()));
	println!("{}", game.status_message());

	Ok(())
}
