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
	// info!("{}", launcher);
	// let mut board: Board = if args.is_present("infile") {
	// 	Board::from_file(args.value_of("infile").unwrap())?
	// } else {
	// 	Board::new(n, n)
	// };
	// if !args.is_present("infile") {
	// 	board.shuffle(iterations);
	// }
	// println!("{}", board);
	// board.show_solution();
	// if args.is_present("play") {
	// 	board.play();
	// }
	return Ok(());
}
