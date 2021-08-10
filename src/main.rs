#[macro_use]
extern crate clap;
use clap::{App, Arg};

use log::*;

mod npuzzle;
use npuzzle::Board;
use npuzzle::init_logger;



fn main() {
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
        ).arg(
            Arg::with_name("play")
            .long("play")
            .takes_value(false)
            .help("play mode"),
        ).arg(
            Arg::with_name("infile")
            .short("f")
            .long("file")
            .takes_value(true)
            .help("Reads the intial state from given file")
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
    if args.is_present("infile") {
        warn!("Not implemented");
        return;
    }
    let mut board = Board::new(n, n);
    println!("{}", board);
    // dbg!(board);
    board.shuffle(iterations);
    if args.is_present("play") {
        board.play();
    }
}
