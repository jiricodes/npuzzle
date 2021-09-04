//! Generic traits for possible grid implmentations
//! will allow 2d vec, graph, linked lists or whatever
//!
//! Consider having PuzzleErrors as return values instead of bools

pub trait Grid {
    type M; // moves
    type P; // position

    /// To get vector of possible moves
    fn possible_moves(&self) -> Vec<Self::M>;
    /// To make a move aka swap positions
    /// Output is a success indicator
    fn do_move(&mut self, dir: &Self::M) -> bool;

    /// To unmake a move
    /// Output is a success indicator
    fn undo_move(&mut self, dir: &Self::M) -> bool;

    /// To set a value
    /// bool for success
    fn set_value(&mut self, position: Self::P, value: usize) -> bool;

    /// To get a value
    fn get_value(&self, position: Self::P) -> Option<usize>;

    /// Returns (width, height)
    fn dim(&self) -> (usize, usize);

    /// retrieves data for its internal structure from 2d vector
    /// Consider if single vector is sufficient in use cases
    fn from_2dvec(&mut self, data: Vec<Vec<usize>>);
}
