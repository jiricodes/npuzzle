#[macro_use]
extern crate clap;

use log::*;

mod logger;
use logger::init_logger;

mod launcher;
use launcher::{Launcher, PuzzleMode};

use npuzzle_lib::board::Board;
use npuzzle_lib::utils::status::{ErrorKind, PuzzleError, Result};

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
