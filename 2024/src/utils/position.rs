use std::{fmt::write, path::Display};
use super::Move;


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}


impl std::ops::Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}


impl Position {

   pub fn from<T>(index: usize, grid: &super::grid::Grid<T>) -> Position {
    Position {
        x: index.checked_rem(grid.width).unwrap(),
        y: index / (grid.width),
    }
   }

   pub fn from_width(index: usize, grid_width: usize) -> Position {
    Position {
        x: index.checked_rem(grid_width).unwrap(),
        y: index / grid_width,
    }
   }

   pub fn to_index<T>(&self, grid: &super::grid::Grid<T>) -> usize {
        self.y * grid.width + self.x
   }

   pub fn move_to(&self, direction: &Move) -> Option<Position> {
        Some(Position {
            x: direction.move_x(self.x, 1)?,
            y: direction.move_y(self.y, 1)?,
        })
   }

   pub fn sides(&self) -> [Option<Position>;4] {
        [
            self.move_to(&Move::TOP),
            self.move_to(&Move::RIGHT),
            self.move_to(&Move::DOWN),
            self.move_to(&Move::LEFT),
        ]
   }

   pub fn distance_to(&self, other: &Position) -> (i32, i32) {
        let diff_x: i32 = other.x as i32 - self.x as i32;
        let diff_y: i32 = other.y as i32 - self.y as i32;
        (diff_x, diff_y)
   }

}


impl std::fmt::Display for Position {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.x, self.y)
    }

}