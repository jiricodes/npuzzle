//! Generic traits for possible grid implmentations
//! will allow 2d vec, graph, linked lists or whatever

use super::status::Result;

pub trait Grid {
	type M; // moves
	type P; // position

	/// To get vector of possible moves
	fn possible_moves(&self) -> Vec<Self::M>;

	/// To make a move aka swap positions
	fn do_move(&mut self, dir: &Self::M) -> Result<()>;

	/// To unmake a move
	fn undo_move(&mut self, dir: &Self::M) -> Result<()>;

	/// To set a value
	fn set_value(&mut self, position: Self::P, value: usize) -> Result<()>;

	/// To get a value
	fn get_value(&self, position: Self::P) -> Result<usize>;

	/// Returns (width, height)
	fn dim(&self) -> (usize, usize);

	/// retrieves data for its internal structure from 2d vector
	/// Consider if single vector is sufficient in use cases
	fn from_2dvec(&mut self, data: Vec<Vec<usize>>) -> Result<()>;
}
