use super::utils::is_in_bounds;
use super::{ErrorKind, PuzzleError, Result};
use log::*;
use rand::{seq::SliceRandom, thread_rng};
use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use text_io::read;

#[derive(Debug, Hash)]
pub struct Board {
    width: usize,
    height: usize,
    data: Vec<usize>,
    solution: Vec<usize>,
    zero_position: usize,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        let mut new_board = Self {
            width: width,
            height: height,
            data: vec![0; width * height],
            solution: vec![0; width * height],
            zero_position: 0,
        };
        new_board.fill_board();
        new_board.solution = new_board.data.clone();
        new_board
    }

    fn get_index(&self, col: usize, row: usize) -> usize {
        col + row * self.width
    }

    fn is_xy_in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32
    }

    fn set_coords(&mut self, x: i32, y: i32, val: usize) {
        if self.is_xy_in_bounds(x, y) {
            let i = self.get_index(x as usize, y as usize);
            self.data[i] = val;
            if val == 0 {
                self.set_zero_index(i);
            }
        }
    }

    fn set_zero_xy(&mut self, x: i32, y: i32) {
        if self.is_xy_in_bounds(x, y) {
            self.zero_position = self.get_index(x as usize, y as usize);
        }
    }

    fn set_zero_index(&mut self, i: usize) {
        if i < self.width * self.height {
            self.zero_position = i;
        }
    }

    fn fill_board(&mut self) {
        // Boundaries
        let mut left_top: (i32, i32) = (0, 0);
        let mut right_bot: (i32, i32) = ((self.width - 1) as i32, (self.height - 1) as i32);
        // Direction handlers
        let dir_col: [i32; 4] = [1, 0, -1, 0];
        let dir_row: [i32; 4] = [0, 1, 0, -1];
        let mut di: usize = 0;

        let mut col = left_top.0;
        let mut row = left_top.1;

        for i in 0..(self.width * self.height) {
            let val = if i < (self.width * self.height - 1) {
                i + 1
            } else {
                0
            };
            self.set_coords(col, row, val);
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
    }

    pub fn possible_directions(&self) -> Vec<usize> {
        let mut res: Vec<usize> = Vec::new();
        // Up
        if self.zero_position >= self.width {
            res.push(self.zero_position - self.width)
        }
        // Down
        if self.zero_position < self.width * (self.height - 1) {
            res.push(self.zero_position + self.width)
        }
        // Left
        if self.zero_position % self.width > 0 {
            res.push(self.zero_position - 1)
        }
        // Right
        if self.zero_position % self.width < (self.width - 1) {
            res.push(self.zero_position + 1)
        }
        res
    }

    pub fn shuffle(&mut self, steps: usize) {
        if steps == 0 {
            return;
        }
        let mut visited_states: HashSet<Vec<usize>> = HashSet::new();
        for i in 0..steps {
            let mut possible_moves = self.possible_directions();
            possible_moves.shuffle(&mut thread_rng());
            let mut moved = false;
            'consider: for current_move in possible_moves {
                self.data.swap(self.zero_position, current_move);
                if !visited_states.contains(&self.data) {
                    visited_states.insert(self.data.clone());
                    self.zero_position = current_move;
                    moved = true;
                    break 'consider;
                } else {
                    self.data.swap(self.zero_position, current_move);
                }
            }
            if moved == false {
                panic!(
                    "all possible from this point already visited {}/{}",
                    i, steps
                );
            }
        }
    }

    fn get_move(&mut self, key: &str) -> Option<usize> {
        match key {
            "up" | "u" => {
                if self.zero_position >= self.width {
                    Some(self.zero_position - self.width)
                } else {
                    None
                }
            }
            "down" | "d" => {
                if self.zero_position < self.width * (self.height - 1) {
                    Some(self.zero_position + self.width)
                } else {
                    None
                }
            }
            "left" | "l" => {
                if self.zero_position % self.width > 0 {
                    Some(self.zero_position - 1)
                } else {
                    None
                }
            }
            "right" | "r" => {
                if self.zero_position % self.width < (self.width - 1) {
                    Some(self.zero_position + 1)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// would be better outside board strucy
    pub fn play(&mut self) {
        print!("\x1B[2J\x1B[1;1H");
        println!("{}", self);
        while self.data != self.solution {
            let input: String = read!("{}\n");
            match input.as_str() {
                "exit" => {
                    return;
                }
                "help" => {
                    println!("Possible options: up / u, down / d, left / l, right / r, exit, help")
                }
                "solution" => {
                    self.show_solution();
                    continue;
                }
                _ => {
                    if let Some(move_to) = self.get_move(input.as_str()) {
                        self.data.swap(move_to, self.zero_position);
                        self.zero_position = move_to;
                    } else {
                        continue;
                    }
                }
            }
            print!("\x1B[2J\x1B[1;1H");
            println!("{}", self);
        }
    }

    pub fn show_solution(&self) {
        let mut longest: usize = 0;
        let mut n = self.width * self.height - 1;
        while n != 0 {
            longest += 1;
            n /= 10;
        }
        let mut res = String::new();
        for i in 0..self.width * self.height {
            res.push_str(&format!("{:width$} ", self.solution[i], width = longest));
            if i % self.width == self.width - 1 {
                res.push_str("\n");
            }
        }
        println!("Expected Solution:\n{}", res);
    }

    pub fn from_str(input: &str) -> Result<Board> {
        let mut board = Board {
            width: 0,
            height: 0,
            data: Vec::new(),
            solution: Vec::new(),
            zero_position: 0,
        };
        let lines = input.split("\n");
        for raw_line in lines {
            let line = clean_line(raw_line);
            let words: Vec<&str> = line.split_whitespace().collect();
            if words.len() == 0 {
                continue;
            } else if words.len() == 1 && board.width == 0 {
                if words[0].chars().all(char::is_numeric) {
                    board.width = words[0].parse::<usize>().unwrap();
                    board.height = board.width;
                } else {
                    return Err(PuzzleError::Puzzle(ErrorKind::InvalidChar));
                }
            } else if board.width != 0 && words.len() == board.width {
                for word in words {
                    if word.chars().all(char::is_numeric) {
                        let val = word.parse::<usize>().unwrap();
                        if val < board.width * board.width && !board.data.contains(&val) {
                            board.data.push(val);
                        } else {
                            return Err(PuzzleError::Puzzle(ErrorKind::InvalidValue));
                        }
                    } else {
                        return Err(PuzzleError::Puzzle(ErrorKind::InvalidChar));
                    }
                }
            } else {
                return Err(PuzzleError::Puzzle(ErrorKind::InvalidNoSize));
            }
        }
        if let Some(zero_pos) = board.data.iter().position(|&x| x == 0) {
            board.zero_position = zero_pos;
        } else {
            return Err(PuzzleError::Puzzle(ErrorKind::InvalidNoZero));
        }
        let solution_board = Board::new(board.width, board.height);
        board.solution = solution_board.solution.clone();
        info!("Board read sucessfully from the file");
        return Ok(board);
    }

    pub fn from_file(filename: &str) -> Result<Board> {
        let mut file = File::open(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        // while width and height == 0 search for size in the string
        Board::from_str(&contents)
    }

    pub fn as_output_string(&self) -> String {
        format!(
            "# by github.com/jiricodes/npuzzle\n{}\n{}",
            self.width, *self
        )
    }
}

fn clean_line(input_string: &str) -> &str {
    if let Some(comment) = input_string.find("#") {
        &input_string[0..comment]
    } else {
        input_string
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut longest: usize = 0;
        let mut n = self.width * self.height - 1;
        while n != 0 {
            longest += 1;
            n /= 10;
        }
        let mut res = String::new();
        for i in 0..self.width * self.height {
            res.push_str(&format!("{:width$} ", self.data[i], width = longest));
            if i % self.width == self.width - 1 {
                res.push_str("\n");
            }
        }
        write!(f, "{}", res)
    }
}
