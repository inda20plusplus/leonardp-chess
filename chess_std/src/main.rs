mod lib;

fn main() {
	let game = lib::Game::new();
	let txt = game.board.print(lib::PrintStyle::Ascii);
  println!("{}", txt);
}
