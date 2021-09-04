//! 2D grid representation of the problem
use super::grid_traits::Grid;
use std::cmp::PartialEq;
use std::fmt;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    /// Safe
    fn try_from(coords: Coords) -> Option<Self> {
        match coords {
            Coords(1, 0) => Some(Self::Up),
            Coords(-1, 0) => Some(Self::Down),
            Coords(0, 1) => Some(Self::Right),
            Coords(0, -1) => Some(Self::Left),
            _ => None,
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

// Row then Column? makes sense I guess
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Coords(i32, i32);

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
            Direction::Up => Coords(1, 0),
            Direction::Down => Coords(-1, 0),
            Direction::Right => Coords(0, 1),
            Direction::Left => Coords(0, -1),
        }
    }
}

#[derive(Clone, Debug, Hash, Eq)]
pub struct Grid2D {
    width: usize,
    height: usize,
    data: Vec<Vec<usize>>,
    zero_position: Coords,
}

impl Grid2D {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: Vec::new(),
            zero_position: Coords(0, 0),
        }
    }

    pub fn in_bounds(&self, coords: Coords) -> bool {
        coords.0 >= 0
            && self.height > coords.0 as usize
            && coords.1 >= 0
            && self.width > coords.1 as usize
    }
}

impl Grid for Grid2D {
    type M = Direction;
    type P = Coords;

    fn possible_moves(&self) -> Vec<Direction> {
        let mut res: Vec<Direction> = Vec::new();
        for dir in Direction::iterator() {
            if self.in_bounds(self.zero_position + Coords::from(dir)) {
                res.push(dir)
            }
        }
        res
    }

    fn do_move(&mut self, dir: &Direction) -> bool {
        let position = self.zero_position + Coords::from(*dir);
        if let Some(val) = self.get_value(position) {
            self.set_value(self.zero_position, val);
            self.set_value(position, 0);
            self.zero_position = position;
            true
        } else {
            false
        }
    }

    fn undo_move(&mut self, dir: &Direction) -> bool {
        let rev_dir = dir.opposite();
        let position = self.zero_position + Coords::from(rev_dir);
        if let Some(val) = self.get_value(position) {
            self.set_value(self.zero_position, val);
            self.set_value(position, 0);
            self.zero_position = position;
            true
        } else {
            false
        }
    }

    fn set_value(&mut self, position: Coords, value: usize) -> bool {
        if self.in_bounds(position) {
            self.data[position.0 as usize][position.1 as usize] = value;
            true
        } else {
            false
        }
    }

    fn get_value(&self, position: Coords) -> Option<usize> {
        if self.in_bounds(position) {
            let val = self.data[position.0 as usize][position.1 as usize];
            Some(val)
        } else {
            None
        }
    }

    fn dim(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    fn from_2dvec(&mut self, data: Vec<Vec<usize>>) {
        self.data = data;
    }
}

/// rewrite in style with board.rs display, calculating printing width
impl fmt::Display for Grid2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.data.is_empty() {
            write!(f, "Empty")
        } else {
            let mut out = String::new();
            for i in 0..self.height {
                out.push_str(&format!("{:?}\n", self.data[i]));
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
