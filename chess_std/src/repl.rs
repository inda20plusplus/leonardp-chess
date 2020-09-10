
use crate::lib::*;
use std::io::{self, Write, Result as IOResult};

type IOResultPlain = IOResult<()>;
type ShouldExit = bool;


pub struct GameRepl {
	pub game: Game,
	// TODO: use read/write trait (eg. to allow tcp stream instead of stdin/out)
	pub stdout: io::Stdout,
	history: Vec<Game>,
}

impl GameRepl {
	pub fn new(game: Game, stdout: io::Stdout) -> GameRepl {
		GameRepl {
			game,
			stdout,
			history: vec![],
		}
	}
	pub fn connect<I>(&mut self, lines: I) -> IOResultPlain where I: Iterator<Item = String> {
		self.print_board()?;
		writeln!(self.stdout, "Enter moves like 'a2 a3', 'status', 'undo', or 'exit'.")?;
		
		self.prompt()?;

		for line in lines {
			let should_exit = self.handle_line(&line)?;
			if should_exit {
				return Ok(())
			}
		}

		Ok(())
	}
	fn print_board(&mut self) -> IOResultPlain {
		writeln!(self.stdout, "{}", self.game.board.print(BoardPrintStyle::ascii_pretty()))
	}
	fn handle_line(&mut self, line: &str) -> IOResult<ShouldExit> {
		match line.trim() {
			""=> {
				self.print_board()?;
				self.prompt()?;
			},
			"exit"=> return Ok(true),
			"status"=> {
				writeln!(self.stdout, "{}", self.game.status_message())?;
				self.prompt()?;
			},
			"undo"=> {
				if let Some(prev) = self.history.pop() {
					self.game = prev;
					self.prompt()?;
				}
			},
			_=> {
				self.history.push(self.game.clone());
				match self.attempt_move(line) {
					Err(e)=> {
						self.history.pop();
						self.print_error(&e)
					},
					Ok(_)=> self.print_board(),
				}?;
				self.prompt()?;
			},
		};
		Ok(false)
	}
	fn attempt_move(&mut self, line: &str) -> Result<(), String> {
		let action = self.game.move_from_str(line)?;
		self.game.perform_action(action)
	}
	fn prompt(&mut self) -> IOResultPlain {
		write!(self.stdout, "{}> ", self.game.current_player_title())?;
		self.stdout.flush()
	}
	fn print_error(&mut self, err: &str) -> IOResultPlain {
		writeln!(self.stdout, "{}", err)
	}
}
