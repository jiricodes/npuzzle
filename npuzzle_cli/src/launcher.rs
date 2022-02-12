//! Handles launching appropriate modes based on cmd line input
//!

use clap::{App, Arg};
use log::*;

use std::fmt;

use super::Board;
use super::Result;

#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum PuzzleMode {
    Play,
    Solve,
    Generate,
}

impl PuzzleMode {
    pub fn as_str_array() -> &'static [&'static str] {
        &["play", "solve", "generate"]
    }

    pub fn from_str(mode: &str) -> Option<Self> {
        match mode {
            "play" => Some(Self::Play),
            "solve" => Some(Self::Solve),
            "generate" => Some(Self::Generate),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match *self {
            Self::Play => "play",
            Self::Solve => "solve",
            Self::Generate => "generate",
            _ => "unknown",
        }
    }
}

impl fmt::Display for PuzzleMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug)]
pub struct Launcher {
    mode: PuzzleMode,
    size: Option<usize>,
    iters: Option<usize>,
    input_file: Option<String>,
}

impl Launcher {
    pub fn new() -> Self {
        let args = App::new(crate_name!())
            .author(crate_authors!())
            .version(crate_version!())
            .about("N-puzzle solver. Project within ecole 42 curriculum at Hive Helsinki")
            .arg(
                Arg::with_name("size")
                    .short("n")
                    .takes_value(true)
                    .conflicts_with("infile")
                    .value_name("SIZE")
                    .required_unless("infile")
                    .conflicts_with("infile")
                    .help("Size of the puzzle"),
            )
            .arg(
                Arg::with_name("iterations")
                    .short("i")
                    .value_name("NUM")
                    .long("iterations")
                    .takes_value(true)
                    .required_unless("infile")
                    .conflicts_with("infile")
                    .help("Shuffle iterations"),
            )
            .arg(
                Arg::with_name("mode")
                    .long("mode")
                    .value_name("MODE")
                    .short("m")
                    .default_value("solve")
                    .possible_values(PuzzleMode::as_str_array())
                    .takes_value(true)
                    .help("Play mode"),
            )
            .arg(
                Arg::with_name("infile")
                    .short("f")
                    .long("file")
                    .takes_value(true)
                    .value_name("FILE")
                    .required_unless("size")
                    .conflicts_with("size")
                    .help("Reads the intial state from given file"),
            )
            .get_matches();
        let size = if args.is_present("size") {
            Some(value_t_or_exit!(args, "size", usize))
        } else {
            None
        };
        let iters = if args.is_present("iterations") {
            Some(value_t_or_exit!(args, "iterations", usize))
        } else {
            None
        };
        let input_file = if args.is_present("infile") {
            Some(String::from(args.value_of("infile").unwrap()))
        } else {
            None
        };
        // Will panic if mode is not supported
        // should be handled by clap's default values before this happens tho
        let mode = PuzzleMode::from_str(args.value_of("mode").unwrap()).unwrap();
        Self {
            mode,
            size,
            iters,
            input_file,
        }
    }

    /// Gets game board based on parsed arguments
    /// panics if both size and input_file are None
    /// or if size is Some and iterations is None
    pub fn get_board(&self) -> Result<Board> {
        if self.input_file.is_some() {
            Board::from_file(self.input_file.as_ref().unwrap())
        } else {
            let n = self.size.unwrap();
            let mut board = Board::new(n, n);
            board.shuffle(self.iters.unwrap());
            Ok(board)
        }
    }

    pub fn get_mode(&self) -> PuzzleMode {
        self.mode
    }
}

impl fmt::Display for Launcher {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "N-puzzle launcher:\nMode: {}\n", self.mode)?;
        write!(f, "Size: {:?}\n", self.size)?;
        write!(f, "Iterations: {:?}\n", self.iters)?;
        write!(f, "Iput File: {:?}\n", self.input_file)?;
        Ok(())
    }
}
