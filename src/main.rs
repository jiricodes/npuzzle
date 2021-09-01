#[macro_use]
extern crate clap;
use clap::{App, Arg};

use log::*;

mod npuzzle;
use npuzzle::init_logger;
use npuzzle::Board;
use npuzzle::Result;

fn main() -> Result<()> {
	let args = App::new("npuzzle")
		.arg(
			Arg::with_name("size")
				.short("n")
				.takes_value(true)
				.help("Size of the puzzle"),
		)
		.arg(
			Arg::with_name("iterations")
				.short("i")
				.long("iterations")
				.takes_value(true)
				.help("Shuffle iterations"),
		)
		.arg(
			Arg::with_name("play")
				.long("play")
				.takes_value(false)
				.help("play mode"),
		)
		.arg(
			Arg::with_name("infile")
				.short("f")
				.long("file")
				.takes_value(true)
				.help("Reads the intial state from given file"),
		)
		.get_matches();
	// Initialize logger - default to debug level, lets adjust it with arguments later
	init_logger();
	let n = if args.is_present("size") {
		value_t_or_exit!(args, "size", usize)
	} else {
		3
	};
	info!("Npuzzle size is {}", n);
	let iterations = if args.is_present("iterations") {
		value_t_or_exit!(args, "iterations", usize)
	} else {
		10
	};
	info!("Shuffle iterations set to {}", iterations);
	let mut board: Board = if args.is_present("infile") {
		Board::from_file(args.value_of("infile").unwrap())?
	} else {
		Board::new(n, n)
	};
	if !args.is_present("infile") {
		board.shuffle(iterations);
	}
	println!("{}", board);
	board.show_solution();
	if args.is_present("play") {
		board.play();
	}
	return Ok(());
}
