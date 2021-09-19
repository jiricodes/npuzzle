#[macro_use]
extern crate clap;
use clap::{App, Arg};

use log::*;

mod npuzzle;
use npuzzle::init_logger;
use npuzzle::Board;
use npuzzle::Launcher;
use npuzzle::Result;

fn main() -> Result<()> {
	// Initialize logger - default to debug level, lets adjust it with arguments later
	init_logger();
	let launcher = Launcher::new();
	info!("{}", launcher);
	let mut board: Board = launcher.get_board()?;
	println!("Current state:\n{}", board);
	board.show_solution();
	// if args.is_present("play") {
	// 	board.play();
	// }
	return Ok(());
}
