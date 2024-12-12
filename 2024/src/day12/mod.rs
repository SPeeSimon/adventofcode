use std::collections::BTreeSet;
use std::{collections::HashSet, str::FromStr};
use crate::utils::{grid::Grid, position::Position, Move};


fn get_sorter(dir: &Move) -> impl FnMut(&&Position, &&Position) -> std::cmp::Ordering {
    match dir {
        Move::TOP   => |a: &&Position, b: &&Position| a.y.cmp(&b.y).then(a.x.cmp(&b.x)),
        Move::DOWN  => |a: &&Position, b: &&Position| a.y.cmp(&b.y).then(a.x.cmp(&b.x)),
        Move::RIGHT => |a: &&Position, b: &&Position| a.x.cmp(&b.x).then(a.y.cmp(&b.y)),
        Move::LEFT  => |a: &&Position, b: &&Position| a.x.cmp(&b.x).then(a.y.cmp(&b.y)),
        _ => todo!()
    }
}


fn is_adjecent(prev: &Position, next: &Position, facing_direction: &Move) -> bool {
    match facing_direction {
        Move::TOP   => prev.y == next.y && prev.x.abs_diff(next.x) == 1,
        Move::DOWN  => prev.y == next.y && prev.x.abs_diff(next.x) == 1,
        Move::RIGHT => prev.x == next.x && prev.y.abs_diff(next.y) == 1,
        Move::LEFT  => prev.x == next.x && prev.y.abs_diff(next.y) == 1,
        _ => todo!()
    }
}


fn find_connected_with_same_value(grid: &Grid<char>, position: &Position) -> HashSet<Position> {
    let mut result = HashSet::new();
    let mut check_neighbours = BTreeSet::new();
    let value = grid.at(position);
    result.insert(position.clone());
    check_neighbours.insert(position.clone());

    
    while let Some(pos) = check_neighbours.pop_first() {
        for next_pos in pos.sides() {
            if let Some(p) = next_pos {
                if !result.contains(&p) &&
                    grid.in_grid(&p) &&
                    grid.at(&p).eq(&value) {
                        check_neighbours.insert(p);
                        result.insert(p);
                }
            }
        }
    }
    result
}


fn count_fence(region: &HashSet<Position>) -> usize {
    let mut outer_limits = 0;
    for pos in region {
        for dir in [Move::TOP, Move::RIGHT, Move::DOWN, Move::LEFT] {
            if pos.move_to(&dir).filter(|p| region.contains(p)).is_none() {
                outer_limits += 1;
            }
        }
    }
    outer_limits
}


fn count_fence_sides(region: &HashSet<Position>) -> usize {
    let mut sum = 0;
    for dir in [Move::TOP, Move::RIGHT, Move::DOWN, Move::LEFT] {
        let match_fence_dir = get_edges_facing(region, &dir);
        let fence_cnt = 
        match_fence_dir.iter()
                       .fold((None, 0), |res: (Option<&Position>, usize), cur| {
                            if let Some(prev) = res.0 {
                                if is_adjecent(prev, cur, &dir) {
                                    return (Some(cur), res.1);
                                }
                            }
                            (Some(cur), res.1 + 1)
                       }).1;
        sum += fence_cnt;
    }

    sum
}


fn get_edges_facing<'a>(region: &'a HashSet<Position>, facing_direction: &Move) -> Vec<&'a Position> {
    let mut match_fence_dir = Vec::new();
    for pos in region {
        if pos.move_to(&facing_direction).filter(|p| region.contains(p)).is_none() {
            match_fence_dir.push(pos);
        }
    }
    match_fence_dir.sort_by(get_sorter(&facing_direction));
    match_fence_dir
}


fn calculate_fencing_region(grid: &Grid<char>) -> usize {
    let mut visited: Vec<Position> = Vec::new();
    let mut total = 0;

    for position in grid.iter_lr_down() {
        if visited.contains(&position) {
            continue;
        }
        let connected_grid = find_connected_with_same_value(&grid, &position);
        let fence_size = count_fence(&connected_grid);
        connected_grid.iter().for_each(|&g| visited.push(g));
        total += connected_grid.len() * fence_size;
    }
    total
}


fn calculate_fencing_region_bulk(grid: &Grid<char>) -> usize {
    let mut visited: Vec<Position> = Vec::new();
    let mut total = 0;

    for position in grid.iter_lr_down() {
        if visited.contains(&position) {
            continue;
        }
        let connected_grid = find_connected_with_same_value(&grid, &position);
        let fence_size = count_fence_sides(&connected_grid);
        connected_grid.iter().for_each(|&g| visited.push(g));
        total += connected_grid.len() * fence_size;
    }
    total
}



#[cfg(test)]
pub mod day12_tests {
    use super::*;
    use std::fs;


#[test]
fn example1() {
    let input = 
"AAAA
BBCD
BBCC
EEEC";
    // area: number of garden plots
    // perimeter: number of sides
    // price: area * perimeter
    // total_price = sum price
    // A: 4, 10 = 40
    // B: 4, 8 = 32
    // C: 4, 10 = 40
    // D: 1, 4 = 4
    // E: 3, 8 = 24
    let grid = Grid::from_str(input).unwrap();
    let total = calculate_fencing_region(&grid);
    assert_eq!(140, total);
}


#[test]
fn example1_2() {
    let input = 
"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
    // X: 1, 4 = 4
    // X: 1, 4 = 4
    // X: 1, 4 = 4
    // X: 1, 4 = 4
    // O: 21, 36 = 756
    let grid = Grid::from_str(input).unwrap();
    let total = calculate_fencing_region(&grid);
    assert_eq!(772, total);
}


#[test]
fn example1_3() {
    let input = 
"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
    // R: 12 * 18 = 216.
    // I: 4 * 8 = 32.
    // C: 14 * 28 = 392.
    // F: 10 * 18 = 180.
    // V: 13 * 20 = 260.
    // J: 11 * 20 = 220.
    // C: 1 * 4 = 4.
    // E: 13 * 18 = 234.
    // I: 14 * 22 = 308.
    // M: 5 * 12 = 60.
    // S: 3 * 8 = 24.
    let grid = Grid::from_str(input).unwrap();
    let total = calculate_fencing_region(&grid);
    assert_eq!(1930, total);
}


#[test]
fn part1() {
    let input = fs::read_to_string("src/day12/input.txt").unwrap();
    let grid = Grid::from_str(&input).unwrap();
    let total = calculate_fencing_region(&grid);
    assert_eq!(1550156, total);
}


#[test]
fn example2() {
    let input = 
"AAAA
BBCD
BBCC
EEEC";
    // area: number of garden plots
    // perimeter: number of sides
    // price: area * perimeter
    // total_price = sum price
    // A: 4, 4 = 16
    // B: 4, 4 = 16
    // C: 4, 8 = 32
    // D: 1, 4 = 4
    // E: 3, 4 = 4
    let grid = Grid::from_str(input).unwrap();
    let total = calculate_fencing_region_bulk(&grid);
    assert_eq!(80, total);
}


#[test]
fn example2_2() {
    let input = 
"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
    let grid = Grid::from_str(input).unwrap();
    let total = calculate_fencing_region_bulk(&grid);
    assert_eq!(436, total);
}


#[test]
fn example2_3() {
    let input = 
"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
    // E = 17 * 12 = 204
    // X = 4 * 4 = 16
    // X = 4 * 4 = 16
    let grid = Grid::from_str(input).unwrap();
    let total = calculate_fencing_region_bulk(&grid);
    assert_eq!(236, total);
}


#[test]
fn example2_4() {
    let input = 
"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";
    let grid = Grid::from_str(input).unwrap();
    let total = calculate_fencing_region_bulk(&grid);
    assert_eq!(368, total);
}


#[test]
fn example2_5() {
    let input = 
"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
    let grid = Grid::from_str(input).unwrap();
    let total = calculate_fencing_region_bulk(&grid);
    assert_eq!(1206, total);
}


#[test]
fn part2() {
    let input = fs::read_to_string("src/day12/input.txt").unwrap();
    let grid = Grid::from_str(&input).unwrap();
    let total = calculate_fencing_region_bulk(&grid);
    assert_ne!(1550156, total);
    assert_eq!(946084, total);
}

}
