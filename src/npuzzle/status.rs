use clap;
use std::cmp::{PartialEq, PartialOrd};
use std::error::Error;
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

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ErrorKind {
	InvalidInput,
	Unsolvable,
}

impl ErrorKind {
	fn as_str(&self) -> &str {
		match *self {
			ErrorKind::InvalidInput => "Invalid N-puzzle state",
			ErrorKind::Unsolvable => "Given state is unsolvable",
		}
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

impl Error for PuzzleError {}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn basic() {
		// let e = PuzzleError::from(IOErrorType::from(ErrorKind::PermissionDenied));
		// assert_eq!(e, PuzzleError::IOError);
		unimplemented!()
	}
}
