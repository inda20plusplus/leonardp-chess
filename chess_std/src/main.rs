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
	
	game.perform_action(game.move_from_str("a2 a4")?)?;
	game.perform_action(game.move_from_str("a8 a6")?)
		.expect_err("a piece was in the way");
	// game.perform_action(game.move_from_str("a7 a4")?)
	// 	.expect_err("pawn can only move at max 2 steps initially")
	game.perform_action(game.move_from_str("a7 a6")?)?;
	game.perform_action(game.move_from_str("a1 a3")?)?;
	game.perform_action(game.move_from_str("a6 a5")?)?;
	game.perform_action(game.move_from_str("a3 b4")?)
		.expect_err("rook cannot move diagonally");
	game.perform_action(game.move_from_str("a3 d3")?)?;

	game.perform_action(game.move_from_str("e7 e6")?)?;
	game.perform_action(game.move_from_str("e2 e4")?)?;
	game.perform_action(game.move_from_str("f8 a3")?)?;
	game.perform_action(game.move_from_str("b1 a3")?)?;
	game.perform_action(game.move_from_str("d8 e7")?)?;
	game.perform_action(game.move_from_str("e1 e3")?)
		.expect_err("king cannot move 2 steps");
	game.perform_action(game.move_from_str("d1 h5")?)?;


	// game.perform_action(game.move_from_str("b1 c3")?)?;
	// game.perform_action(game.move_from_str("b8 a6")?)?;
	// game.perform_action(game.move_from_str("c3 e4")?)?;
	// game.perform_action(game.move_from_str("a6 b4")?)?;
	// game.perform_action(game.move_from_str("e4 g5")?)?;
	// game.perform_action(game.move_from_str("b4 c2")?)?;
	// game.perform_action(game.move_from_str("g5 f7")?)?;
	println!("{}", game.board.print(BoardPrintStyle::ascii_bordered()));
	println!("{}", game.status_message());

	Ok(())
}
