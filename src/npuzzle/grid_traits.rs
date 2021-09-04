//! Generic traits for possible grid implmentations
//! will allow 2d vec, graph, linked lists or whatever
//!
//! Consider having PuzzleErrors as return values instead of bools
use std::iter::Iterator;

/// To get vector of possible moves
pub trait PossibleDirs<T: Iterator> {
    fn possible_directions(&self) -> Vec<T>;
}

/// To make a move aka swap positions
/// Output is a success indicator
pub trait DoMove<T> {
    fn do_move(&mut self, movement: T) -> bool;
}

/// To unmake a move
/// Output is a success indicator
pub trait UndoMove<T> {
    fn undodo_move(&mut self, movement: T) -> bool;
}

/// To set a value
/// bool for success
pub trait SetValue<T> {
    fn set_value(&mut self, position: T, value: usize) -> bool;
}

// /// retrieves position of the zero (empty)
// pub trait ZeroPosition<T> {
//     fn get_zero_position(&self) -> T;
// }
