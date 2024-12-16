use crate::utils::{position::Position};
use std::{collections::HashMap};


#[derive(Debug)]
struct Robot {
    start: Position,
    velocity_x: i32,
    velocity_y: i32,
}


impl Robot {

    fn move_n_within_grid(&self, n: i32, grid: &Position) -> Position {
        let calc = |start: i64, velocity: i64, n: i64, grid: i64| (grid + (start + (velocity * n))).rem_euclid(grid);
        let new_x = calc(self.start.x.try_into().unwrap(), self.velocity_x.into(), n.into(), grid.x.try_into().unwrap());
        let new_y = calc(self.start.y.try_into().unwrap(), self.velocity_y.into(), n.into(), grid.y.try_into().unwrap());
        Position {
            x: usize::try_from(new_x).unwrap(),
            y: usize::try_from(new_y).unwrap(),
        }
    }

}


fn calc_sum_quadrants(robots: Vec<Robot>, gridsize: Position, n: i32) -> i32 {
    let mut quadrants = HashMap::new();
    for r in robots {
        let new_pos = r.move_n_within_grid(n, &gridsize);
        let quad = quadrant(&new_pos, &gridsize);
        // println!("ROBOT: New {:?} {}", &new_pos, quad);
        quadrants.entry(quad).and_modify(|cnt| *cnt += 1).or_insert(1);
    }

    let total = quadrants.iter()
                              .filter(|(k,_)| (0..4).contains(*k))
                              .map(|(_,v)|*v)
                              .product();
    total
}


fn create_robots(input: &str) -> Vec<Robot> {
    let robots: Vec<Robot> = 
    input.lines()
         .map(|line| line.split_once(" ").unwrap())
         .map(|(cur, vel)| {
            let (cur_x, cur_y) = cur.split_once("=").unwrap().1.split_once(",").unwrap();
            let (diff_x, diff_y) = vel.split_once("=").unwrap().1.split_once(",").unwrap();

            Robot {
                start: Position{x: cur_x.parse().unwrap(), y: cur_y.parse().unwrap()},
                velocity_x: diff_x.parse().unwrap(),
                velocity_y: diff_y.parse().unwrap(),
            }
         })
         .collect();
    robots
}


fn quadrant(position: &Position, grid: &Position) -> u8 {
    // 0 | 1
    // --+--  5
    // 2 | 3
    match (grid.x / 2).cmp(&position.x) {
        std::cmp::Ordering::Less => {
            match (grid.y / 2).cmp(&position.y) {
                std::cmp::Ordering::Less => 3,
                std::cmp::Ordering::Equal => 5,
                std::cmp::Ordering::Greater => 1,
            }
        },
        std::cmp::Ordering::Equal => 5,
        std::cmp::Ordering::Greater => {
            match (grid.y / 2).cmp(&position.y) {
                std::cmp::Ordering::Less => 2,
                std::cmp::Ordering::Equal => 5,
                std::cmp::Ordering::Greater => 0,
            }
        },
    }
}


#[cfg(test)]
pub mod day14_tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use crossterm::{QueueableCommand, terminal, cursor, style::{self, Stylize}};
    use image::{Rgb, RgbImage};
    
const IDX_OF_TREE: i32 = 6577;
const IMAGE_FOLDER: &str = "target/day14/";

#[test]
fn example1() {
    // robots' current positions (p) and velocities (v)
    let input = 
"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    let robots = create_robots(input);
    let gridsize = Position{x: 11, y: 7};
    let total = calc_sum_quadrants(robots, gridsize, 100);
    assert_eq!(12, total);
}

#[test]
fn part1() {
    let input = fs::read_to_string("src/day14/input.txt").unwrap();
    let robots = create_robots(&input);
    let gridsize = Position{x: 101, y: 103};
    let total = calc_sum_quadrants(robots, gridsize, 100);
    assert_eq!(229421808, total);
}


#[test]
fn part2() {
    let input = fs::read_to_string("src/day14/input.txt").unwrap();
    let robots = create_robots(&input);
    let gridsize = Position{x: 101, y: 103};
    let mut stdout = std::io::stdout();

    let robot_positions: Vec<Position> = robots.iter()
                                                .map(|r| r.move_n_within_grid(IDX_OF_TREE, &gridsize))
                                                .collect();

    // save positions to file
    match fs::exists(IMAGE_FOLDER) {
        Ok(false) => fs::create_dir(IMAGE_FOLDER).unwrap(),
        _ => {},
    }
    let mut img = RgbImage::from_pixel(gridsize.x.try_into().unwrap(), gridsize.y.try_into().unwrap(), Rgb([0, 0, 0]));
    robot_positions.iter().for_each(|p| img.put_pixel(p.x.try_into().unwrap(), p.y.try_into().unwrap(), Rgb([31, 118, 5])));
    img.save(format!("{}/img{}.png", IMAGE_FOLDER, IDX_OF_TREE)).unwrap();
    
    // show positions in console (shift content up 30 lines to show full image)
    stdout.queue(terminal::Clear(terminal::ClearType::All)).unwrap()
          .queue(cursor::MoveTo(110,0)).unwrap()
          .queue(style::PrintStyledContent(format!("MAP {}", IDX_OF_TREE).magenta())).unwrap();
    for y in 30..gridsize.y {
        for x in 0..gridsize.x {
            stdout.queue(cursor::MoveTo(x.try_into().unwrap(), (y - 30).try_into().unwrap())).unwrap();
            if robot_positions.contains(&Position{x: x, y: y}) {
                stdout.queue(style::PrintStyledContent("█".green())).unwrap();
            } else {
                stdout.queue(style::Print(" ")).unwrap();
            }
        }
    }
    stdout
        .queue(cursor::MoveToNextLine(1)).unwrap()
        .flush().unwrap();

    assert_eq!(6577, IDX_OF_TREE);
}

}
