use std::num::ParseIntError;
use std::str::FromStr;

use super::position::Position;
use super::Move;

#[derive(Debug)]
pub struct Grid<T> {
    pub grid: Vec<T>,
    pub width: usize,
    pub height: usize,
}


impl FromStr for Grid<char> {
    type Err = ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let gird_without_newline = input.lines().flat_map(|line|line.chars()).collect();
        Ok(Grid {
            grid: gird_without_newline,
            height: input.lines().count(),
            width: input.lines().next().unwrap_or_default().len(),
        })
    }

}



impl<T> Grid<T> {

    pub fn find_all_positions<'a, P>(&'a self, predicate: P) -> Vec<Position>
        where P: Fn(&T) -> bool
    {
        self.grid.iter().enumerate()
                 .filter(|(_idx, val)| predicate(val))
                 .map(|(idx, _)| self.position_of(idx))
                 .collect()
    }


    pub fn distances(&self, position: &Position) -> (usize, usize, usize, usize) {
        (
            position.x, // to left
            self.width - position.x, // to right
            position.y, // to top
            self.height - position.y, // to bottom
        )
    }

    pub fn position_of(&self, index: usize) -> Position {
        Position::from(index, self)
    }

    pub fn at(&self, position: &Position) -> Option<&T> {
        self.grid.get(position.to_index(self))
    }

    pub fn in_grid(&self, position: &Position) -> bool {
        position.x < self.width && position.y < self.height
    }

    pub fn max_to(&self, position: &Position, direction: &Move) -> usize {
        match direction {
            Move::TOP => position.y,
            Move::TOP_RIGHT => position.y.min(self.distances(position).1),
            Move::RIGHT => self.distances(position).1,
            Move::DOWN_RIGHT => {
                let distances = self.distances(position);
                distances.1.min(distances.3)
            },
            Move::DOWN => self.distances(position).3,
            Move::DOWN_LEFT => {
                let distances = self.distances(position);
                distances.0.min(distances.3)
            },
            Move::LEFT => self.distances(position).0,
            Move::TOP_LEFT => {
                let distances = self.distances(position);
                distances.0.min(distances.2)
            },
        }
    }


    pub fn positions_to(&self, position: &Position, direction: &Move, size: usize) -> Vec<Position> {
        let mut result = Vec::new();

        for add in 1..size {
            if let Some(y) = direction.move_y(position.y, add) {
                if let Some(x) = direction.move_x(position.x, add) {
                    result.push( Position{x: x, y: y} );
                }
            }
        }
        result.retain(|pos| pos.x < self.width && pos.y <= self.height);
        result
    }


    pub fn positions_to_inclusive(&self, position: &Position, direction: &Move, size: usize) -> Vec<Position> {
        let mut result = Vec::new();

        for add in 0..size {
            if let Some(y) = direction.move_y(position.y, add) {
                if let Some(x) = direction.move_x(position.x, add) {
                    result.push( Position{x: x, y: y} );
                }
            }
        }

        result.retain(|pos| pos.x < self.width && pos.y <= self.height);
        result
    }


    pub fn set(&mut self, position: &Position, value: T) {
        let idx = position.to_index(self);
        if idx < self.grid.len() {
            self.grid[idx] = value;
        }
    }

    // fn walk_to(&self, position: &Position, direction: &Move) -> impl Iterator<Item = (&Position, Option<&T>)> {
    //     self.positions_to(position, direction, self.max_to(position, direction))
    //         .iter()
    //         .map(|pos| (pos, self.at(pos)))
    // }

}
