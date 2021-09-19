#[macro_use]
extern crate clap;

use log::*;

mod npuzzle;
use npuzzle::init_logger;
use npuzzle::launcher::PuzzleMode;
use npuzzle::Board;
use npuzzle::Launcher;
use npuzzle::{ErrorKind, PuzzleError, Result};

fn main() -> Result<()> {
	// Initialize logger - default to debug level, lets adjust it with arguments later
	init_logger();
	let launcher = Launcher::new();
	info!("{}", launcher);
	let mut board: Board = launcher.get_board()?;
	// consider using some kind of handler struct for this
	// eg. Game::handle(board, mode)
	match launcher.get_mode() {
		PuzzleMode::Generate => println!("{}", board.as_output_string()),
		PuzzleMode::Play => board.play(),
		PuzzleMode::Solve => println!("Solver not implemented!"),
		_ => return Err(PuzzleError::Puzzle(ErrorKind::InvalidMode)),
	}

	return Ok(());
}
