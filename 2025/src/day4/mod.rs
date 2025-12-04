use std::collections::HashMap;



fn count_accessible_papers(input: &str) -> usize {
    let paper_positions: Vec<(usize, usize)> = input.lines().enumerate()
                                .flat_map(|(y, line)| {
                                    line.chars().enumerate()
                                        .filter(|(_, ch)| *ch == '@')
                                        .map(move |(x, _)|  (x, y))
                                })
                                .collect();
    let mut accesible_positions = Vec::new();
    let height = input.lines().count();
    let width = input.lines().next().unwrap().chars().count();
    
    for x in 0..width {
        for y in 0..height {
            let mut count = 0;
            if !paper_positions.contains(&(x, y)) {
                continue;
            }
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dy == 0 && dx == 0 {
                        continue;
                    }
                    let ny = y as isize + dy;
                    let nx = x as isize + dx;
                    if paper_positions.contains(&(nx as usize, ny as usize)) {
                        count += 1;
                    }
                }
            }
    
            if count < 4 {
                accesible_positions.push((x, y));
            }
        }
    }
    
    accesible_positions.len()
}


fn count_accessible_papers_with_removal(input: &str) -> usize {
    let mut paper_positions = HashMap::new();
    input.lines().enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate()
                .filter(|(_, ch)| *ch == '@')
                .map(move |(x, _)|  (x, y))
        })
        .for_each(|position| {
            paper_positions.insert(position, 0);
        });
    let original_count = paper_positions.len();

    let keys = paper_positions.keys().cloned().collect::<Vec<(usize, usize)>>();
    paper_positions.iter_mut().for_each(|(k, v)| {
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dy == 0 && dx == 0 {
                        continue;
                    }
                    let nx = k.0 as isize + dx;
                    let ny = k.1 as isize + dy;
                    if keys.contains(&(nx as usize, ny as usize)) {
                        *v += 1;
                    }
                }
            }
    });

    loop {
        let removable_positions: Vec<(usize, usize)> = paper_positions.iter()
                                                                        .filter(|(_, &count)| count < 4)
                                                                        .map(|(&pos, _)| pos)
                                                                        .collect();
        if removable_positions.is_empty() {
            break;
        }
        
        for pos in removable_positions.iter() {
            paper_positions.remove(pos);
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dy == 0 && dx == 0 {
                        continue;
                    }
                    let nx = pos.0 as isize + dx;
                    let ny = pos.1 as isize + dy;
                    if let Some(count) = paper_positions.get_mut(&(nx as usize, ny as usize)) {
                        *count -= 1;
                    }
                }
            }
        }
    }

    original_count - paper_positions.len()
}


#[cfg(test)]
pub mod day4_tests {
    use super::*;
    use std::fs;

#[test]
fn example1_1() {
    let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
    // fewer than four rolls of paper in the eight adjacent positions
// In this example, there are 13 rolls of paper that can be accessed by a forklift (marked with x):
// ..xx.xx@x.
// x@@.@.@.@@
// @@@@@.x.@@
// @.@@@@..@.
// x@.@@@@.@x
// .@@@@@@@.@
// .@.@.@.@@@
// x.@@@.@@@@
// .@@@@@@@@.
// x.x.@@@.x.
    let total = count_accessible_papers(input);
    assert_eq!(13, total);
}


#[test]
fn part1() {
    let input = fs::read_to_string("src/day4/input1.txt").unwrap();
    let total = count_accessible_papers(input.as_str());
    assert_eq!(1376, total);
}


#[test]
fn example2_1() {
    let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
// Once a roll of paper can be accessed by a forklift, it can be removed
    let total = count_accessible_papers_with_removal(input);
    assert_ne!(13, total);
    assert_eq!(43, total);
}


#[test]
fn part2() {
    let input = fs::read_to_string("src/day4/input1.txt").unwrap();
    let total = count_accessible_papers_with_removal(input.as_str());
    assert_ne!(1376, total);
    assert_eq!(8587, total);
}


}
