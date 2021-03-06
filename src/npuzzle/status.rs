//! Error handling module

use clap;
use std::cmp::{PartialEq, PartialOrd};

use std::fmt;
use std::io;

pub type Result<T> = std::result::Result<T, PuzzleError>;

#[derive(Debug)]
pub enum PuzzleError {
	Io(io::Error),
	Clap(clap::Error),
	Puzzle(ErrorKind),
	Custom(String),
}

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ErrorKind {
	InvalidInput,
	InvalidNoSize,
	InvalidChar,
	InvalidValue,
	InvalidNoZero,
	InvalidMode, // should never happen technically
	InvalidMove,
	OutOfBounds,
	InvalidPoint,
	EmptyInput,
	Unsolvable,
}

impl ErrorKind {
	fn as_str(&self) -> &str {
		match *self {
			ErrorKind::InvalidInput => "Invalid N-puzzle state: General Error",
			ErrorKind::InvalidNoSize => "Invalid N-puzzle state: No size given",
			ErrorKind::InvalidChar => "Invalid N-puzzle state: Invalid character",
			ErrorKind::InvalidValue => {
				"Invalid N-puzzle state: Invalid value - too high or duplicate"
			}
			ErrorKind::InvalidNoZero => "Invalid N-puzzle state: Invalid value - zero not found",
			ErrorKind::InvalidMode => "Invalid program mode",
			ErrorKind::InvalidMove => "Attempted move is invalid",
			ErrorKind::OutOfBounds => "Position out of bounds",
			ErrorKind::InvalidPoint => "Position is invalid",
			ErrorKind::EmptyInput => "Input is not expected to be empty",
			ErrorKind::Unsolvable => "Given state is unsolvable",
		}
	}

	fn err_name(&self) -> &str {
		match *self {
			ErrorKind::InvalidNoSize
			| ErrorKind::InvalidInput
			| ErrorKind::InvalidChar
			| ErrorKind::InvalidValue
			| ErrorKind::InvalidNoZero
			| ErrorKind::InvalidMode => "InvalidInput",
			ErrorKind::InvalidMove
			| ErrorKind::OutOfBounds
			| ErrorKind::InvalidPoint
			| ErrorKind::EmptyInput => "InvalidParameter",
			ErrorKind::Unsolvable => "Unsolvable",
		}
	}
}

impl fmt::Debug for ErrorKind {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct(self.err_name())
			.field("message", &self.as_str())
			.finish()
	}
}

impl fmt::Display for PuzzleError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match *self {
			PuzzleError::Io(ref err) => err.fmt(f),
			PuzzleError::Clap(ref err) => err.fmt(f),
			PuzzleError::Puzzle(ref err) => write!(f, "Puzzle Error: {:?}", err),
			PuzzleError::Custom(ref err) => write!(f, "Custom Error: {:?}", err),
		}
	}
}

impl From<io::Error> for PuzzleError {
	fn from(f: io::Error) -> Self {
		Self::Io(f)
	}
}

impl From<clap::Error> for PuzzleError {
	fn from(f: clap::Error) -> Self {
		Self::Clap(f)
	}
}
