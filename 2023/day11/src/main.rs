use std::{str::FromStr, num::ParseIntError, collections::HashMap, fs, fmt::{Display, Write}};
use itertools::Itertools;


const COSMIC_EXPANSION: i32 = 1000000; // =2 (part 1) | =1000000 (part 2)


fn main() {
    println!("Hello, world!");
}


type Position = (i32,i32);


#[derive(Debug)]
struct Universe {
    galaxies: Vec<Position>,
    width: i32,
    heigth: i32,
}


impl FromStr for Universe {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut found_galaxies: Vec<Position> = Vec::new();
        let mut width: i32 = 0;
        let mut height: i32 = 0;

        for line in s.lines() {
            let mut found_galaxy = false;
            for (index, point) in line.char_indices() {
                if point != '.' {
                    found_galaxies.push((height, index.try_into().unwrap()));
                    found_galaxy = true;
                }
                width = width.max(index.try_into().unwrap());
            }

            if !found_galaxy {
                height += COSMIC_EXPANSION; // expand height
            } else {
                height += 1;
            }
        }

        // expand width
        let mut galaxies = Vec::new();
        let mut new_width = 0;

        for w in 0..width+1 {
            let mut has_galaxy = false;
            for g in found_galaxies.iter() {
                if g.1 == w {
                    galaxies.push((g.0, new_width));
                    has_galaxy = true;
                }
            }
            
            if !has_galaxy {
                new_width += COSMIC_EXPANSION;
            } else {
                new_width += 1;
            }
        }

        Ok(Universe { galaxies: galaxies, width: new_width, heigth: height })
    }
}


impl Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for x in 0..self.heigth {
            for y in 0..self.width {
                if self.galaxies.contains(&(x,y)) {
                    f.write_char('#').unwrap();
                } else {
                    f.write_char('.').unwrap();
                }
            }
            f.write_str("\n").unwrap();
        }
        Ok(())
    }
}


type GalaxyDistance = (Position, Position);
fn add_unique_distance(mut acc: HashMap<GalaxyDistance, u64>, p: (Position, Position, u32)) -> HashMap<GalaxyDistance, u64> {
    if !acc.contains_key(&(p.1, p.0)) {
        acc.insert((p.0, p.1), p.2.into());
    }
    acc
}


fn calculate_distance(point1: &Position, point2: &Position) -> u32 {
    point1.0.abs_diff(point2.0) + point1.1.abs_diff(point2.1)
}


fn find_shortest_path(input: &str) -> u64 {
    let universe = Universe::from_str(input).unwrap();
    // println!("universe:\n{:?}\n", universe);

    let galaxy_distances: HashMap<(Position, Position), u64> = universe.galaxies.iter().permutations(2)
            .map(|p| (*p[0], *p[1], calculate_distance(p[0], p[1])))
            .fold(HashMap::new(), add_unique_distance);

    // galaxy_distances.iter().for_each(|p| println!("  {:?} = {}", p.0, p.1));
    let total = galaxy_distances.values().into_iter().map(|p| p).sum::<u64>();
    total
}


#[test]
fn example1() {
    let input =
"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    // rows or columns with no galaxies expand: 2x
    // column: 3
    // row: 2

    // 5-9 = 9
    // 1-7 = 15
    // 3-6 = 17
    // 8-9 = 5
    assert_eq!(2, COSMIC_EXPANSION, "Cosmic expansion not set correctly for this part");
    assert_eq!(374, find_shortest_path(input));
}


#[test]
fn part1() {
    assert_eq!(2, COSMIC_EXPANSION, "Cosmic expansion not set correctly for this part");
    let input = fs::read_to_string("src/input.txt").unwrap();
    let result = find_shortest_path(input.as_str());
    println!("distance: \x1b[32m{}\x1b[0m", result);
    assert_eq!(9648398, result);
}


#[test]
fn example2() {
    let input =
"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    // rows or columns with no galaxies expand: 1000000
    assert_eq!(1000000, COSMIC_EXPANSION, "Cosmic expansion not set correctly for this part");
    assert_eq!(82000210, find_shortest_path(input));
}


#[test]
fn part2() {
    assert_eq!(1000000, COSMIC_EXPANSION, "Cosmic expansion not set correctly for this part");
    let input = fs::read_to_string("src/input.txt").unwrap();
    let result = find_shortest_path(input.as_str());
    println!("distance: \x1b[32m{}\x1b[0m", result);
    assert_eq!(618800410814, result);
}
