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
	

	game.perform_action(game.move_from_str("a2 a5")?)
			.expect_err("pawn can only move at max 2 steps initially");

	game.perform_action(game.move_from_str("a2 a4")?)?; // 2 steps
	game.perform_action(game.move_from_str("b7 b6")?)?; // 1 steps

	game.perform_action(game.move_from_str("a4 a6")?)
			.expect_err("pawn 2 step move only allowed from start pos");
	game.perform_action(game.move_from_str("a4 a3")?)
			.expect_err("no backwards");
	game.perform_action(game.move_from_str("a4 b5")?)
			.expect_err("no diagonal forward without capture (en_passant)");

	game.perform_action(game.move_from_str("a4 a5")?)?;
	game.perform_action(game.move_from_str("b6 a5")?)?; // diagonal capture

	game.perform_action(game.move_from_str("d2 d3")?)?; // white dummy move
	game.perform_action(game.move_from_str("a5 a4")?)?;
	game.perform_action(game.move_from_str("b2 b4")?)?;
	game.perform_action(game.move_from_str("a4 b3")?)?; // en_passant
	game.perform_action(game.move_from_str("b4 b5")?)
			.expect_err("captured using en_passant in previous turn");
	game.perform_action(game.move_from_str("c2 c3")?)?; // en_passant made possible
	game.perform_action(game.move_from_str("h7 h5")?)?; // forfeit en_passant move
	game.perform_action(game.move_from_str("h2 h4")?)?;
	game.perform_action(game.move_from_str("b3 c2")?)
			.expect_err("en_passant only valid immediately after becoming possible");

	game.perform_action(game.move_from_str("h5 h4")?)
			.expect_err("pawn cannot capture forward");

	println!("{}", game.board.print(BoardPrintStyle::ascii_pretty()));
	println!("{}", game.status_message());

	Ok(())
}
