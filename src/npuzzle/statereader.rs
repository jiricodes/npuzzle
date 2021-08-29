use super::Board;
use super::Result;
use std::fs::File;
use std::io::prelude::*;

pub fn read_input_state_from_file(filename: &str) -> Result<Board> {
	let mut file = File::open(filename)?;
	let mut contents = String::new();
	file.read_to_string(&mut contents)?;
	let board = Board::from_str(&contents)?;
	Ok(board)
}
