use std::{collections::HashSet, fs, str::FromStr};
use std::collections::HashMap;
use itertools::Itertools;
use crate::utils::{grid::Grid, position::Position};


const TRAILHEAD: char = '0';
const TRAIL_FIRST_STEP: char = '1';
const PEAK: char = '9';



fn create_trail_grid(input: &str) -> (Grid<char>, Vec<Position>) {
    let grid = Grid::from_str(input).unwrap();
    let trail_start_positions = grid.find_all_positions(|c| TRAILHEAD.eq(c));
    (grid, trail_start_positions)
}


fn find_trails(trail_start_positions: Vec<Position>, grid: Grid<char>) -> HashMap<Position, Vec<Position>> {
    let mut trails = HashMap::new();
    trail_start_positions.iter()
                         .for_each(|trail_start| { trails.insert(trail_start.clone(), vec![trail_start.clone()]);});

    for lvl in TRAIL_FIRST_STEP..=PEAK {
        trails.iter_mut().for_each(|(_, lvls)| {
            let mut next_steps : Vec<Position> = lvls.drain(..)
                                        .flat_map(|p| p.sides())
                                        .filter_map(|p| p)
                                        .filter(|p| grid.at(p).eq(&Some(&lvl)))
                                        .filter(|p| grid.in_grid(p))
                                        .unique()
                                        .collect();
            lvls.append(&mut next_steps);
        });
    }
    trails
}


fn create_next_positions(grid: &Grid<char>, trail: &Vec<Position>, next_lvl: &char) -> Vec<Vec<Position>>
{
    trail.last().iter()
            .flat_map(|l| l.sides())
            .filter_map(|p| p)
            .filter(move |p| grid.at(p).eq(&Some(next_lvl)))
            .filter(|p| grid.in_grid(p))
            .map(|next| {
                let mut new_trail = trail.clone();
                new_trail.push(next);
                new_trail
            })
            .collect()
}


fn find_trails_rating(trail_start_positions: Vec<Position>, grid: Grid<char>) -> HashMap<Position, usize> {
    let mut trails: HashMap<Position, HashSet<Vec<Position>>> = HashMap::new();
    trail_start_positions.iter()
                         .map(|start_pos| (start_pos, HashSet::new()))
                         .map(|mut trail| {
                            trail.1.insert(vec![trail.0.clone()]);
                            trail
                        })
                         .for_each(|(trail_start, trail)| {
                            trails.insert(trail_start.clone(), trail);
                        });

    for lvl in TRAIL_FIRST_STEP..=PEAK {
        for (_, trails_per_start) in trails.iter_mut() {
            let trail_next: Vec<Vec<Position>> = 
                    trails_per_start.drain()
                                    .flat_map(|trail| {
                                        let trails_for_next_step = create_next_positions(&grid, &trail, &lvl);
                                        trails_for_next_step
                                    })
                                    .collect();
            trail_next.iter().for_each(|t| {trails_per_start.insert(t.to_owned());});
        }
    }

    let mut trail_ratings = HashMap::new();
    trails.iter().for_each(|(start, unique_ways)| { trail_ratings.insert(*start, unique_ways.len());});
    trail_ratings
}



#[cfg(test)]
pub mod day10_tests {
    use super::*;


#[test]
fn example1() {
    let input = 
"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
    // longest hiking trail
    // even, gradual, uphill slope, always increases, never diagonal
    // trail score = number of 9-height reachable from a trail
    let (grid, trails_start) = create_trail_grid(input);
    let trails = find_trails(trails_start, grid);

    println!("# of trails: {}", trails.len());
    trails.iter().for_each(|(k,v)| println!("Trail @{k}={}  // {:?}", v.len(), v));
    
    let total = trails.values().map(|t| t.len() as i32).sum();
    assert_eq!(5 + 6 + 5 + 3 + 1 + 3 + 5 + 3 + 5, total);
}


#[test]
fn example1_1() {
    let input = 
"..90..9
...1.98
...2..7
6543456
765.987
876....
987....";
    let (grid, trails_start) = create_trail_grid(input);
    let trails = find_trails(trails_start, grid);
    let total = trails.values().map(|t| t.len() as i32).sum();
    assert_eq!(4, total);
}


#[test]
fn part1() {
    let input = fs::read_to_string("src/day10/input.txt").unwrap();
    let (grid, trails_start) = create_trail_grid(&input);
    let trails = find_trails(trails_start, grid);
    let total = trails.values().map(|t| t.len() as i32).sum();
    assert_ne!(632, total); // too high
    assert_eq!(629, total);
}


#[test]
fn example2_1() {
    let input = 
".....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9....";
    let (grid, trails_start) = create_trail_grid(input);
    let trails = find_trails_rating(trails_start, grid);

    println!("# of trails: {}", trails.len());
    trails.iter().for_each(|(k,v)| println!("Trail @{k}={}", v));
    
    let total = trails.values().map(|t| *t as i32).sum();
    assert_eq!(3, total);
}


#[test]
fn example2_2() {
    let input = 
"..90..9
...1.98
...2..7
6543456
765.987
876....
987....";
    // rating = unique paths to top
    let (grid, trails_start) = create_trail_grid(input);
    let trails = find_trails_rating(trails_start, grid);

    println!("# of trails: {}", trails.len());
    trails.iter().for_each(|(k,v)| println!("Trail @{k}={}", v));
    
    let total = trails.values().map(|t| *t as i32).sum();
    assert_eq!(13, total);
}


#[test]
fn example2_3() {
    let input = 
"012345
123456
234567
345678
4.6789
56789.";
    let (grid, trails_start) = create_trail_grid(input);
    let trails = find_trails_rating(trails_start, grid);
    let total = trails.values().map(|t| *t as i32).sum();
    assert_eq!(227, total);
}


#[test]
fn example2_4() {
    let input = 
"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
    let (grid, trails_start) = create_trail_grid(input);
    let trails = find_trails_rating(trails_start, grid);
    let total = trails.values().map(|t| *t as i32).sum();
    assert_eq!(20 + 24 + 10 + 4 + 1 + 4 + 5 + 8 + 5, total);
    assert_eq!(81, total);
}


#[test]
fn part2() {
    let input = fs::read_to_string("src/day10/input.txt").unwrap();
    let (grid, trails_start) = create_trail_grid(&input);
    let trails = find_trails_rating(trails_start, grid);
    let total = trails.values().map(|t| *t as i32).sum();
    assert_ne!(1292, total); // too high
    assert_eq!(1242, total);
}

}
