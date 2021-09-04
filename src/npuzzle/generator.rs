use super::grid_traits::{DoMove, PossibleDirs, SetValue, UndoMove};
use super::Result;
use log::*;
use rand::{seq::SliceRandom, thread_rng};
use std::cmp::Eq;
use std::collections::HashSet;
use std::hash::Hash;
use std::iter::Iterator;

pub enum PuzzleType {
    Snail,   // subject given snail spiral solution
    Lines00, // empty at the beginning
    LinesNN, // Empty at the end
}
pub struct Generator<P, G>
where
    P: Iterator,
    G: PossibleDirs<P> + DoMove<P> + UndoMove<P> + SetValue<P> + Hash + Eq + Clone,
{
    grid: G,
    puzzletype: PuzzleType,
}

impl<P, G> Generator<P, G>
where
    P: Iterator,
    G: PossibleDirs<P> + DoMove<P> + UndoMove<P> + SetValue<P> + Hash + Eq + Clone,
{
    pub fn new(grid: G, puzzletype: PuzzleType) -> Self {
        Self { grid, puzzletype }
    }

    pub fn generate(&self, iterations: usize) -> Vec<Vec<usize>> {
        unimplemented!()
    }

    pub fn solution(&self) -> Vec<Vec<usize>> {
        unimplemented!()
    }

    ///
    /// Shuffles the given grid. Keeps track of visited states
    /// to quarantee solution length
    /// This may break the memory if too many iterations?
    ///
    fn shuffle(&mut self, iterations: usize) {
        if iterations == 0 {
            return;
        }
        let mut visited_states: HashSet<G> = HashSet::new();
        'shuffle: for i in 0..iterations {
            let mut possible_moves = self.grid.possible_directions();
            possible_moves.shuffle(&mut thread_rng());
            let mut moved = false;
            'consider: for current_move in possible_moves {
                self.grid.do_move(current_move);
                if !visited_states.contains(&self.grid) {
                    visited_states.insert(self.grid.clone());
                    moved = true;
                    break 'consider;
                } else {
                    self.grid.undodo_move(current_move);
                }
            }
            if moved == false {
                warn!(
                    "all possible from this point already visited {}/{}",
                    i, iterations
                );
                break 'shuffle;
            }
        }
    }
}
