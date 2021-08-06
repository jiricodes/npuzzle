#[macro_use]
extern crate clap;

use rand::{seq::SliceRandom, thread_rng};
use std::collections::HashSet;
use std::fmt;
use text_io::read;

use clap::{App, Arg};

#[derive(Debug, Hash)]
struct Board {
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
                    "all possible from this point already wisited {}/{}",
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

    pub fn play(&mut self) {
		print!("\x1B[2J\x1B[1;1H");
        println!("{}", self);
        while self.data != self.solution {
            let input: String = read!("{}\n");
            match input.as_str() {
                "exit" => {
                    return;
                },
                "help" => {
                    println!("Possible options: up / u, down / d, left / l, right / r, exit, help")
                },
				"solution" => {
					self.show_solution();
					continue;
				},
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

	fn show_solution(&self) {
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
		println!("Solution:\n{}", res);
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

// inclusive
#[inline]
pub fn is_in_bounds(x: i32, y: i32, left_top: (i32, i32), right_bot: (i32, i32)) -> bool {
    x >= left_top.0 && x <= right_bot.0 && y >= left_top.1 && y <= right_bot.1
}

fn main() {
    let args = App::new("npuzzle")
        .arg(
            Arg::with_name("size")
                .short("n")
                .takes_value(true)
                .help("Size of the puzzle"),
        )
        .arg(
            Arg::with_name("iterations")
                .short("i")
                .long("iterations")
                .takes_value(true)
                .help("Shuffle iterations"),
        )
        .get_matches();
    let n = if args.is_present("size") {
        value_t_or_exit!(args, "size", usize)
    } else {
        3
    };
    let iterations = if args.is_present("iterations") {
        value_t_or_exit!(args, "iterations", usize)
    } else {
        10
    };
    let mut board = Board::new(n, n);
    println!("{}", board);
    // dbg!(board);
    board.shuffle(iterations);
    board.play();
}
