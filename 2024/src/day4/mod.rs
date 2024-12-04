use std::{collections::{HashSet}, fs, path::Display};
use itertools::Itertools;


#[derive(Debug, Eq, PartialEq, Hash)]
enum Move {
    TOP,
    TOP_RIGHT,
    RIGHT,
    DOWN_RIGHT,
    DOWN,
    DOWN_LEFT,
    LEFT,
    TOP_LEFT,
}


#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
}


#[derive(Debug)]
struct Grid<'a> {
    grid: &'a str,
    width: usize,
    height: usize,
}


impl Position {

   fn from(index: usize, grid_width: usize) -> Position {
    Position {
        x: index.checked_rem(grid_width).unwrap(),
        y: index / (grid_width),
    }
   }

   fn to_index(&self, grid_width: usize) -> usize {
        self.y * grid_width + self.x
   }

}


impl <'a> Grid<'a> {

    fn distances(&self, position: &Position) -> (usize, usize, usize, usize) {
        (
            position.x, // to left
            self.width - position.x, // to right
            position.y, // to top
            self.height - position.y, // to bottom
        )
    }

    fn position_of(&self, index: usize) -> Position {
        Position::from(index, self.width)
    }

    fn at(&self, position: &Position) -> Option<char> {
        self.grid.chars().nth(position.to_index(self.width))
    }

    fn max_to(&self, position: &Position, direction: &Move) -> usize {
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

    fn positions_to(&self, position: &Position, direction: &Move, size: usize) -> Vec<Position> {
        let mut result = Vec::new();
        match direction {
            Move::TOP => {
                for add in 0..size {
                    if let Some(y) = position.y.checked_sub(add) {
                        result.push( Position{x: position.x, y: y} );
                    }
                }
            },
            Move::TOP_RIGHT => {
                for add in 0..size {
                    if let Some(y) = position.y.checked_sub(add) {
                        result.push( Position{x: position.x + add, y: y} );
                    }
                }
            },
            Move::RIGHT => {
                for add in 0..size {
                    result.push( Position{x: position.x + add, y: position.y} );
                }
            },
            Move::DOWN_RIGHT => {
                for add in 0..size {
                    result.push( Position{x: position.x + add, y: position.y + add} );
                }
            },
            Move::DOWN => {
                for add in 0..size {
                    result.push( Position{x: position.x, y: position.y + add} );
                }
            },
            Move::DOWN_LEFT => {
                for add in 0..size {
                    if let Some(x) = position.x.checked_sub(add) {
                        result.push( Position{x: x, y: position.y + add} );
                    }
                }
            },
            Move::LEFT => {
                for add in 0..size {
                    if let Some(x) = position.x.checked_sub(add) {
                        result.push( Position{x: x, y: position.y} );
                    }
                }
            },
            Move::TOP_LEFT => {
                for add in 0..size {
                    if let Some(y) = position.y.checked_sub(add) {
                        if let Some(x) = position.x.checked_sub(add) {
                            result.push( Position{x: x, y: y} );
                        }
                    }
                }
            },
        }
        
        result.retain(|pos| pos.x < self.width && pos.y <= self.height);
        result
    }

}


fn count_matches_xmas(grid: &Grid) -> usize {
    const XMAS: &str = "XMAS";
    const SIZE: usize = XMAS.len();
    let mut total = 0;

    for start in grid.grid.match_indices("X") {
        let x_position = grid.position_of(start.0);

        for dir in [Move::TOP, Move::TOP_RIGHT, Move::RIGHT, Move::DOWN_RIGHT, Move::DOWN, Move::DOWN_LEFT, Move::LEFT, Move::TOP_LEFT] {
            let is_match = grid.positions_to(&x_position, &dir, SIZE).iter()
                                     .map(|pos| grid.at(pos).unwrap_or_default())
                                     .eq(XMAS.chars());
            if is_match {
                total += 1;
            }
        }
    }
    total
}


fn count_matches_mas(grid: &Grid) -> usize {
    const MAS: &str = "MAS";
    let mut total = 0;

    for start in grid.grid.match_indices("A") {
        let x_position = grid.position_of(start.0);
        let mut x_match = HashSet::new();

        for dir in [(Move::TOP_RIGHT, Move::DOWN_LEFT), (Move::TOP_LEFT, Move::DOWN_RIGHT), (Move::DOWN_LEFT, Move::TOP_RIGHT), (Move::DOWN_RIGHT, Move::TOP_LEFT)] {
            let is_match = grid.positions_to(&x_position, &dir.0, 2).last().
                                        iter()
                                        .chain([&x_position].iter()
                                                            .chain(grid.positions_to(&x_position, &dir.1, 2).last()
                                                            .iter())
                                        ).map(|pos| grid.at(pos).unwrap_or_default())
                                        .eq(MAS.chars());
            if is_match {
                x_match.insert(dir.0);
                x_match.insert(dir.1);
            }
        }

        if x_match.len() == 4 {
            total += 1;
        }

    }
    total
}



#[cfg(test)]
pub mod day3_tests {
    use super::*;

#[test]
fn example1() {
    let input = 
"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    
    const XMAS: &str = "XMAS";
    const SIZE: usize = XMAS.len();

    let width = input.lines().next().unwrap().len() + 1;
    let grid = Grid {
        grid: input,
        width: width,
        height: input.len() / width,
    };

    let mut count = 1;
    let mut total = 0;

    let p = Position{x:9, y:3};
    println!("{:?} => {:?}", &p, grid.positions_to(&p, &Move::TOP_RIGHT, SIZE));

    println!(r"   Position   x:  , y:     |   >   |   ˅   |   /   |   \   |   <   |   ^   |   \   |   /   | ");
    for start in input.match_indices("X") {
        let x_position = Position::from(start.0, width);
        // let rev_hor = input[..start.0+1].chars().rev().take(SIZE).eq(XMAS.chars());

        for dir in [Move::TOP, Move::TOP_RIGHT, Move::RIGHT, Move::DOWN_RIGHT, Move::DOWN, Move::DOWN_LEFT, Move::LEFT, Move::TOP_LEFT] {
            let txt = grid.positions_to(&x_position, &dir, SIZE).iter()
                                    .map(|pos| input.chars().nth(pos.to_index(grid.width)).unwrap_or_default())
                                    .join("");
            
            let is_match = grid.positions_to(&x_position, &dir, SIZE).iter()
                                     .map(|pos| input.chars().nth(pos.to_index(grid.width)).unwrap_or_default())
                                     .eq(XMAS.chars());

            if is_match {
                total += 1;
            }
            // println!("{:?} - {:?} = {}", &x_position, &dir, txt);
        }
        // println!("{:?}", grid.positions_to(&x_position, Move::TOP, SIZE));

        // let remainder = &input[start.0..];
        // let passed = &input[..start.0+1];

        // let right = XMAS.chars().zip(remainder.chars().take(grid.max_to(&x_position, &Move::RIGHT))).take_while(|(l,r)| l.eq(r)).count() == SIZE;
        // let left = XMAS.chars().zip(passed.chars().rev().take(grid.max_to(&x_position, &Move::LEFT))).take_while(|(l,r)| l.eq(r)).count() == SIZE;

        // let down_left = XMAS.chars().zip(remainder.chars().step_by(width - 1 ).take(grid.max_to(&x_position, &Move::DOWN_LEFT))).take_while(|(l,r)| l.eq(r)).count() == SIZE;
        // let down  = XMAS.chars().zip(remainder.chars().step_by(width).take(grid.max_to(&x_position, &Move::DOWN))).take_while(|(l,r)| l.eq(r)).count() == SIZE;
        // let down_right = XMAS.chars().zip(remainder.chars().step_by(width + 1).take(grid.max_to(&x_position, &Move::DOWN_RIGHT))).take_while(|(l,r)| l.eq(r)).count() == SIZE;

        // let up_left = XMAS.chars().zip(passed.chars().rev().step_by(width - 1).take(grid.max_to(&x_position, &Move::TOP_LEFT))).take_while(|(l,r)| l.eq(r)).count() == SIZE;
        // let up  = XMAS.chars().zip(passed.chars().rev().step_by(width).take(grid.max_to(&x_position, &Move::TOP))).take_while(|(l,r)| l.eq(r)).count() == SIZE;
        // let up_right = XMAS.chars().zip(passed.chars().rev().step_by(width + 1).take(grid.max_to(&x_position, &Move::TOP_RIGHT))).take_while(|(l,r)| l.eq(r)).count() == SIZE;

        // let size = [right, down, down_left, down_right, left, up, up_left, up_right].iter().filter(|&a| *a).count();
        // println!("{:2} {:?} | {right:5} | {down:5} | {down_left:5} | {down_right:5} | {left:5} | {up:5} | {up_left:5} | {up_right:5} | {}", count, x_position, size);
        // count += 1;
        // total += size;
    }

    assert_eq!(18, total);
}


#[test]
fn part1() {
    let input = fs::read_to_string("src/day4/input.txt").unwrap();
    let width = input.lines().next().unwrap().len() + 1;
    let grid = Grid {
        grid: input.as_str(),
        width: width,
        height: input.len() / width,
    };

    assert_eq!(2401, count_matches_xmas(&grid));
}


#[test]
fn example2() {
    let input = 
"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    
    let width = input.lines().next().unwrap().len() + 1;
    let grid = Grid {
        grid: input,
        width: width,
        height: input.len() / width,
    };
    assert_eq!(9, count_matches_mas(&grid));
}


#[test]
fn part2() {
    let input = fs::read_to_string("src/day4/input.txt").unwrap();
    let width = input.lines().next().unwrap().len() + 1;
    let grid = Grid {
        grid: input.as_str(),
        width: width,
        height: input.len() / width,
    };
    assert_eq!(1822, count_matches_mas(&grid));
}

}