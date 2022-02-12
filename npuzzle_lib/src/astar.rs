//! Generic implementation of A* for sliding puzzle
//!
//! Should do following:
//! 	- generic heuristic
//! 	- track complexity in time
//! 	- track complexity in size
//! 	- number of moves between states
//! 	- final "path" / set of moves
//! 	- solvability
//!

use super::grid_traits::Grid;

pub struct Astar<G: Grid> {
	start: G,
	destination: G,
	time_complex: usize,
	size_complex: usize,
	path: Vec<G>,
}
