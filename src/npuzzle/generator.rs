use super::grid_traits::Grid;
use super::utils::is_in_bounds;
use log::*;
use rand::{seq::SliceRandom, thread_rng};
use std::cmp::Eq;
use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug)]
pub enum PuzzleType {
    Snail,   // subject given snail spiral solution
    Lines00, // empty at the beginning
    LinesNN, // Empty at the end (n-1, n-1) - ambiguous naming
}

impl PuzzleType {
    pub fn get_template(&self, width: usize, height: usize) -> Vec<Vec<usize>> {
        match self {
            PuzzleType::Snail => snail_template(width, height),
            PuzzleType::Lines00 => lines00_template(width, height),
            PuzzleType::LinesNN => linesnn_template(width, height),
        }
    }
}

/// Sliding puzzle template generators - consider better organisation
fn snail_template(width: usize, height: usize) -> Vec<Vec<usize>> {
    let mut grid: Vec<Vec<usize>> = Vec::with_capacity(height);
    for _ in 0..height {
        grid.push(vec![0; width]);
    }
    // Boundaries
    let mut left_top: (i32, i32) = (0, 0);
    let mut right_bot: (i32, i32) = ((width - 1) as i32, (height - 1) as i32);
    // Direction handlers
    let dir_col: [i32; 4] = [1, 0, -1, 0];
    let dir_row: [i32; 4] = [0, 1, 0, -1];
    let mut di: usize = 0;

    let mut col = left_top.0;
    let mut row = left_top.1;

    for i in 0..(width * height) {
        let val = if i < (width * height - 1) { i + 1 } else { 0 };
        grid[row as usize][col as usize] = val;
        let candidate_col = col + dir_col[di];
        let candidate_row = row + dir_row[di];
        if is_in_bounds(candidate_col, candidate_row, left_top, right_bot) {
            col = candidate_col;
            row = candidate_row;
        } else {
            di = (di + 1) % 4;
            if dir_col[di] + dir_row[di] > 0 {
                left_top.0 += dir_col[di];
                left_top.1 += dir_row[di];
            } else {
                right_bot.0 += dir_col[di];
                right_bot.1 += dir_row[di];
            }
            col += dir_col[di];
            row += dir_row[di];
        }
    }
    grid
}

fn lines00_template(width: usize, height: usize) -> Vec<Vec<usize>> {
    let mut grid: Vec<Vec<usize>> = Vec::with_capacity(height);
    for r in 0..height {
        grid.push(Vec::new());
        for c in 0..width {
            grid[r].push(r * width + c);
        }
    }
    grid
}
fn linesnn_template(width: usize, height: usize) -> Vec<Vec<usize>> {
    let mut grid: Vec<Vec<usize>> = Vec::with_capacity(height);
    for r in 0..height {
        grid.push(Vec::new());
        for c in 0..width {
            grid[r].push((r * width + c + 1) % (width * height));
        }
    }
    grid
}

/// Probably worth reworking to use mut ref for grid rather than cloned
pub struct Generator<G>
where
    G: Grid + Hash + Eq + Clone,
{
    grid: G,
    puzzletype: PuzzleType,
}

impl<G> Generator<G>
where
    G: Grid + Hash + Eq + Clone,
{
    pub fn new(grid: G, puzzletype: PuzzleType) -> Self {
        Self { grid, puzzletype }
    }

    /// wrapper for generating a solution state and shuffling it
    pub fn generate_random(&mut self, iterations: usize) {
        self.generate_solution();
        self.shuffle(iterations);
    }

    /// should create the "default state" aka solution
    pub fn generate_solution(&mut self) {
        let (width, height) = self.grid.dim();
        let grid_2d = self.puzzletype.get_template(width, height);
        self.grid.from_2dvec(grid_2d);
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
            let mut possible_moves = self.grid.possible_moves();
            possible_moves.shuffle(&mut thread_rng());
            let mut moved = false;
            'consider: for current_move in possible_moves {
                self.grid.do_move(&current_move);
                if !visited_states.contains(&self.grid) {
                    visited_states.insert(self.grid.clone());
                    moved = true;
                    break 'consider;
                } else {
                    self.grid.undo_move(&current_move);
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

    pub fn get_grid(&self) -> G {
        self.grid.clone()
    }
}

#[cfg(test)]
mod test {
    use super::super::grid2d::Grid2D;
    use super::*;

    #[test]
    fn test_snail_template() {
        let g = snail_template(5, 5);
        let mut expected: Vec<Vec<usize>> = Vec::with_capacity(5);
        expected.push(vec![1, 2, 3, 4, 5]);
        expected.push(vec![16, 17, 18, 19, 6]);
        expected.push(vec![15, 24, 0, 20, 7]);
        expected.push(vec![14, 23, 22, 21, 8]);
        expected.push(vec![13, 12, 11, 10, 9]);
        assert_eq!(expected, g);
    }

    #[test]
    fn test_lines00_template() {
        let g = lines00_template(5, 5);
        let mut expected: Vec<Vec<usize>> = Vec::with_capacity(5);
        expected.push(vec![0, 1, 2, 3, 4]);
        expected.push(vec![5, 6, 7, 8, 9]);
        expected.push(vec![10, 11, 12, 13, 14]);
        expected.push(vec![15, 16, 17, 18, 19]);
        expected.push(vec![20, 21, 22, 23, 24]);
        assert_eq!(expected, g);
    }

    #[test]
    fn test_sn_template() {
        let g = linesnn_template(5, 5);
        let mut expected: Vec<Vec<usize>> = Vec::with_capacity(5);
        expected.push(vec![1, 2, 3, 4, 5]);
        expected.push(vec![6, 7, 8, 9, 10]);
        expected.push(vec![11, 12, 13, 14, 15]);
        expected.push(vec![16, 17, 18, 19, 20]);
        expected.push(vec![21, 22, 23, 24, 0]);
        assert_eq!(expected, g);
    }

    #[test]
    fn test_grid2d() {
        let g = Grid2D::new(5, 5);
        let mut gen = Generator::new(g, PuzzleType::Snail);
        gen.generate_solution();
        println!("{}", gen.get_grid());
    }
}
