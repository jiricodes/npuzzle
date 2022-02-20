//! 2D grid representation of the problem
use crate::grid_traits::Grid;
use log::*;
use std::cmp::PartialEq;
use std::fmt;
use std::ops::{Add, Sub};

use crate::utils::status::{ErrorKind, PuzzleError, Result};

#[derive(Debug, Clone, Copy)]
pub enum Direction {
	Up,
	Down,
	Right,
	Left,
}

impl Direction {
	/// Safe
	fn try_from(coords: Coords) -> Result<Self> {
		match coords {
			Coords(1, 0) => Ok(Self::Up),
			Coords(-1, 0) => Ok(Self::Down),
			Coords(0, 1) => Ok(Self::Right),
			Coords(0, -1) => Ok(Self::Left),
			_ => Err(PuzzleError::Puzzle(ErrorKind::InvalidPoint)),
		}
	}

	fn iterator() -> impl Iterator<Item = Direction> {
		[Self::Up, Self::Down, Self::Right, Self::Left]
			.iter()
			.copied()
	}

	fn opposite(self) -> Self {
		match self {
			Self::Up => Self::Down,
			Self::Down => Self::Up,
			Self::Right => Self::Left,
			Self::Left => Self::Right,
		}
	}
}

/// Coordinates struct for Grid2D (row, column)
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Coords(pub i32, pub i32);

impl Add for Coords {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		Self(self.0 + other.0, self.1 + other.1)
	}
}

impl Sub for Coords {
	type Output = Self;

	fn sub(self, other: Self) -> Self {
		Self(self.0 - other.0, self.1 - other.1)
	}
}

impl From<Direction> for Coords {
	fn from(dir: Direction) -> Self {
		match dir {
			Direction::Up => Coords(-1, 0),
			Direction::Down => Coords(1, 0),
			Direction::Right => Coords(0, 1),
			Direction::Left => Coords(0, -1),
		}
	}
}

#[derive(Clone, Debug, Hash, Eq)]
pub struct Grid2D {
	height: usize,
	width: usize,
	data: Vec<Vec<usize>>,
	zero_position: Coords,
}

impl Grid2D {
	pub fn new() -> Self {
		Self {
			width: 0,
			height: 0,
			data: Vec::new(),
			zero_position: Coords(0, 0),
		}
	}

	pub fn with_capacity(height: usize, width: usize) -> Self {
		Self {
			width: width,
			height: height,
			data: vec![Vec::with_capacity(width); height],
			zero_position: Coords(0, 0),
		}
	}

	pub fn in_bounds(&self, coords: Coords) -> bool {
		coords.0 >= 0
			&& self.height > coords.0 as usize
			&& coords.1 >= 0
			&& self.width > coords.1 as usize
	}

	pub fn get_zero(&self) -> Coords {
		self.zero_position
	}
}

impl Grid for Grid2D {
	type M = Direction;
	type P = Coords;

	/// Returns list of possible directions for next move
	fn possible_moves(&self) -> Vec<Direction> {
		let mut res: Vec<Direction> = Vec::new();
		for dir in Direction::iterator() {
			if self.in_bounds(self.zero_position + Coords::from(dir)) {
				res.push(dir)
			}
		}
		res
	}

	/// Does a move - swapping zero with a value in given direction.
	///
	/// This also updates the zero position internally.
	fn do_move(&mut self, dir: &Direction) -> Result<()> {
		let position = self.zero_position + Coords::from(*dir);
		match self.get_value(position) {
			Ok(val) => {
				self.set_value(self.zero_position, val)?;
				self.set_value(position, 0)?;
				self.zero_position = position;
				Ok(())
			}
			Err(_) => Err(PuzzleError::Puzzle(ErrorKind::InvalidMove)),
		}
	}

	/// Undoes a move - swapping zero with a value in opposite of given direction.
	///
	/// This also updates the zero position internally.
	fn undo_move(&mut self, dir: &Direction) -> Result<()> {
		let rev_dir = dir.opposite();
		let position = self.zero_position + Coords::from(rev_dir);
		match self.get_value(position) {
			Ok(val) => {
				self.set_value(self.zero_position, val)?;
				self.set_value(position, 0)?;
				self.zero_position = position;
				Ok(())
			}
			Err(_) => Err(PuzzleError::Puzzle(ErrorKind::InvalidMove)),
		}
	}

	/// Sets a specific position to given value
	fn set_value(&mut self, position: Coords, value: usize) -> Result<()> {
		if self.in_bounds(position) {
			self.data[position.0 as usize][position.1 as usize] = value;
			Ok(())
		} else {
			Err(PuzzleError::Puzzle(ErrorKind::OutOfBounds))
		}
	}

	/// Retrieves value at given position
	fn get_value(&self, position: Coords) -> Result<usize> {
		match self.in_bounds(position) {
			true => Ok(self.data[position.0 as usize][position.1 as usize]),
			false => Err(PuzzleError::Puzzle(ErrorKind::OutOfBounds)),
		}
	}

	/// Retrieves dimensions of the grid
	fn dim(&self) -> (usize, usize) {
		(self.width, self.height)
	}

	/// Transforms 2d vector into grid
	fn from_2dvec(&mut self, data: Vec<Vec<usize>>) -> Result<()> {
		self.data = data;
		self.height = self.data.len();
		if self.height == 0 {
			return Err(PuzzleError::Puzzle(ErrorKind::EmptyInput));
		}
		self.width = self.data[0].len();
		// Checking width consistency and setting zero position
		for r in 0..self.height {
			if self.data[r].len() != self.width {
				return Err(PuzzleError::Puzzle(ErrorKind::InvalidInput));
			}
			for c in 0..self.width {
				if self.data[r][c] == 0 {
					self.zero_position = Coords(r as i32, c as i32);
					return Ok(());
				}
			}
		}
		warn!("No zero found, maybe consider fixing it!");
		Err(PuzzleError::Puzzle(ErrorKind::InvalidNoZero))
	}
}

impl fmt::Display for Grid2D {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if self.data.is_empty() {
			write!(f, "Empty")
		} else {
			let mut longest: usize = 1;
			let mut n = self.width * self.height - 1;
			while n != 0 {
				longest += 1;
				n /= 10;
			}
			let mut out = String::new();
			for row in 0..self.height {
				for col in 0..self.width {
					out.push_str(&format!("{:width$}", self.data[row][col], width = longest));
				}
				out.push_str("\n");
			}
			write!(f, "{}", out)
		}
	}
}

impl PartialEq for Grid2D {
	fn eq(&self, other: &Self) -> bool {
		self.data == other.data
	}
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn basic() {
		let data: Vec<Vec<usize>> = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];
		let mut grid = Grid2D::new();
		assert!(grid.from_2dvec(data.clone()).is_ok());
		assert!(grid.do_move(&Direction::Down).is_ok());
		assert!(grid.do_move(&Direction::Left).is_err());
		assert!(grid.undo_move(&Direction::Down).is_ok());
		assert!(grid.data == data);
		assert_eq!((3, 3), grid.dim());
		let ret = grid.get_value(Coords(1, 1));
		assert!(ret.is_ok());
		assert_eq!(4, ret.unwrap());
		assert!(grid.get_value(Coords(5, 5)).is_err());
		assert!(grid.set_value(Coords(5, 5), 99).is_err());
		assert!(grid.set_value(Coords(0, 0), 99).is_ok());
	}
}
