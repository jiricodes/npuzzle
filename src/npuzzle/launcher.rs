//! Handles launching appropriate modes based on cmd line input
//!

use clap::{App, Arg};
use log::*;

use std::fmt;

#[non_exhaustive]
#[derive(Debug)]
enum PuzzleMode {
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
    size: usize,
    iters: usize,
    input_file: Option<String>,
}

impl Launcher {
    pub fn new() -> Self {
        let args = App::new(crate_name!())
            .arg(
                Arg::with_name("size")
                    .short("n")
                    .takes_value(true)
                    .default_value("3")
                    .conflicts_with("infile")
                    .value_name("SIZE")
                    .help("Size of the puzzle"),
            )
            .arg(
                Arg::with_name("iterations")
                    .short("i")
                    .default_value("10")
                    .value_name("NUM")
                    .long("iterations")
                    .takes_value(true)
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
                    .help("Reads the intial state from given file"),
            )
            .get_matches();
        let size = value_t_or_exit!(args, "size", usize);
        let iters = value_t_or_exit!(args, "iterations", usize);
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
}

impl fmt::Display for Launcher {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "N-puzzle launcher:\nMode: {}", self.mode)?;
        write!(f, "Size: {}", self.size)?;
        write!(f, "Iterations: {}", self.iters)?;
        if self.input_file.is_some() {
            write!(f, "Iput File: {}", self.input_file.as_ref().unwrap())?;
        }
        Ok(())
    }
}
