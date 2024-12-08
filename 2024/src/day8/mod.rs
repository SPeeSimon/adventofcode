use std::{collections::HashMap, fs, ops::{Add, Mul}};
use itertools::Itertools;
use crate::utils::{grid::Grid, position::Position};


fn get_antenna_map(grid: &Grid<char>) -> HashMap<char, Vec<Position>> {
    let mut antenna_map = HashMap::new();

    for (idx, point) in grid.grid.iter().enumerate() {
        match point {
            '.' => {},
            '\n' => {},
            antenna => {
                let antenna_position = Position::from(idx, grid);
                antenna_map.entry(*antenna).or_insert(Vec::new())
                           .push(antenna_position);
            }
        }
    }
    antenna_map
}



fn find_antinodes(antenna_map: &HashMap<char, Vec<Position>>, grid: &Grid<char>) -> Vec<Position> {
    let mut antinode = Vec::new();
    
    for (_antenna, locations) in antenna_map {
        for permutation in locations.iter().permutations(2) {
            match permutation[..] {
                [first, second, ..] => {
                    let (diff_x, diff_y) = first.distance_to(second);
                    if diff_x != 0 && diff_y != 0 {
                        if let Ok(top_x) = (second.x as i32 + diff_x).try_into() {
                            if let Ok(top_y) = (second.y as i32 + diff_y).try_into() {
                                antinode.push(Position{x: top_x, y: top_y});
                            }
                        }
                    }
                },
                _ => {},
            }
        }
    }
    
    antinode.sort();
    antinode.dedup();
    antinode.retain(|p| grid.in_grid(p));
    antinode
}


fn find_antinodes_with_resonate(antenna_map: &HashMap<char, Vec<Position>>, grid: &Grid<char>) -> Vec<Position> {
    let mut antinode = Vec::new();
    
    for (_antenna, locations) in antenna_map {
        for permutation in locations.iter().permutations(2) {
            match permutation[..] {
                [first, second, ..] => {
                    let (diff_x, diff_y) = first.distance_to(second);
                    add_antinodes(grid, diff_x, diff_y, second, &mut antinode, grid.width.max(grid.height));
                },
                _ => {},
            }
        }
    }
    
    antinode.sort();
    antinode.dedup();
    antinode.retain(|p| grid.in_grid(p));
    antinode
}


fn add_antinodes(grid: &Grid<char>, diff_x: i32, diff_y: i32, second: &Position, antinode: &mut Vec<Position>, counter: usize) {
    for i in 0..counter {
        if let Ok(top_x) = diff_x.mul(i as i32).add(second.x as i32).try_into() {
            if let Ok(top_y) = diff_y.mul(i as i32).add(second.y as i32).try_into() {
                let next_position = Position{x: top_x, y: top_y};
                if !grid.in_grid(&next_position) {
                    break;
                }
                antinode.push(next_position);
            }
        }
    }
}




#[cfg(test)]
pub mod day8_tests {
    use std::str::FromStr;
    use super::*;

#[test]
fn example1() {
    let input = 
"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    let grid = Grid::from_str(&input).unwrap();
    let antenna_map = get_antenna_map(&grid);
    let antinode = find_antinodes(&antenna_map, &grid);
    let total = antinode.len();
    assert_eq!(14, total);
}


#[test]
fn part1() {
    let input = fs::read_to_string("src/day8/input.txt").unwrap();
    let grid = Grid::from_str(&input).unwrap();
    let antenna_map = get_antenna_map(&grid);
    let antinode = find_antinodes(&antenna_map, &grid).len();
    let total = antinode;
    assert_ne!(380, total); // too high
    assert_ne!(371, total); // too high
    assert_eq!(367, total); 
}



#[test]
fn example2_sample() {
    let input = 
"T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........";

    let grid = Grid::from_str(&input).unwrap();
    let antenna_map = get_antenna_map(&grid);
    let antinode = find_antinodes_with_resonate(&antenna_map, &grid);
    let total = antinode.len();
    assert_eq!(9, total);
}



#[test]
fn example2() {
    let input = 
"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    let grid = Grid::from_str(&input).unwrap();
    let antenna_map = get_antenna_map(&grid);
    let antinode = find_antinodes_with_resonate(&antenna_map, &grid);
    let total = antinode.len();
    assert_eq!(34, total);
}


#[test]
fn part2() {
    let input = fs::read_to_string("src/day8/input.txt").unwrap();
    let grid = Grid::from_str(&input).unwrap();
    let antenna_map = get_antenna_map(&grid);
    let antinode = find_antinodes_with_resonate(&antenna_map, &grid).len();
    let total = antinode;
    assert_eq!(1285, total); 
}
}
