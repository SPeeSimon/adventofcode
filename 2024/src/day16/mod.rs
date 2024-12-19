use crate::utils::position::Position;
use std::{borrow::BorrowMut, collections::HashMap, str::FromStr};
use crate::utils::{grid::Grid, Move};


const START_DIRECTION: Move = Move::RIGHT;
const START_TILE: char = 'S';
const END_TILE: char = 'E';
const WALL: char = '#';
const TURN_COST: i32 = 1000;


type CrossRoadsMap = HashMap<Position, [Path; 4]>;
type CostMap = HashMap<(Position, Position), i32>;


struct ReindeerOlympic {
    maze: Grid<char>,
    start: Position,
    end: Position,
}


#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Path {
    None,
    Unknown,
    Visited(Position, i32),
}


impl FromStr for ReindeerOlympic {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let grid = Grid::from_str(input).unwrap();
        let start = grid.find_all_positions(|&c| c == START_TILE)[0];
        let end = grid.find_all_positions(|&c| c == END_TILE)[0];
        Ok(ReindeerOlympic {
            maze: grid,
            start: start,
            end: end,
        })
    }
}


impl ReindeerOlympic {

    fn create_crossroad_map(&self) -> CrossRoadsMap {
        let mut crossroads = HashMap::new();
        for point in self.maze.iter_lr_down() {
            if self.maze.at(&point) != Some(&WALL) {
                let sides = point.sides();
                let mut costs = [Path::None; 4];

                for dir in Move::direct_sides() {
                    if sides[dir].is_some_and(|p| self.maze.at(&p) != Some(&WALL)) {
                        costs[dir] = Path::Unknown;
                    } else {
                        costs[dir] = Path::None;
                    }
                }
    
                if (costs[Move::TOP] != Path::None || costs[Move::DOWN] != Path::None) &&
                   (costs[Move::RIGHT] != Path::None || costs[Move::LEFT] != Path::None) {
                    crossroads.insert(point, costs);
                }
            }
        }
        crossroads.entry(self.start).or_insert([Path::Unknown; 4]);
        crossroads.entry(self.end).or_insert([Path::Visited(self.end.clone(), 0); 4]);
        crossroads
    }


    fn fill_crossroad_distances(&self, crossroads: &mut CrossRoadsMap) {
        let crosspoints: Vec<Position> = crossroads.keys().cloned().collect();
        for (&crosspoint, cost_and_distances) in crossroads.borrow_mut().iter_mut() {
            for dir in Move::direct_sides() {
                if let Path::Unknown = cost_and_distances[dir] {
                    cost_and_distances[dir] = check(&self.maze, &crosspoints, &dir, &crosspoint);
                }
            }
        }
    }
}


fn check(grid: &Grid<char>, crossroads: &Vec<Position>, dir: &Move, crosspoint: &Position) -> Path {
    let mut next_crossroad = crosspoint.move_to(&dir);
    let mut count = 1;
    while let Some(check) = next_crossroad {
        if crossroads.contains(&check) {
            return Path::Visited(check, count);
        }
        if !grid.in_grid(&check) {
            break;
        }
        if grid.at(&check) == Some(&WALL) {
            break;
        }
        count += 1;
        next_crossroad = check.move_to(&dir);
    }
    Path::None
}


fn find_route(olympic: &ReindeerOlympic, current: &Position, direction: &Move, cost_to_current: i32, crossroads: &CrossRoadsMap, visited: &mut CostMap) -> Option<i32> {
    if *current == olympic.end {
        return Some(0);
    }
    let current_key = (olympic.start, *current);
    if let Some(&other_route_to_here) = visited.get(&current_key) {
        if other_route_to_here <= cost_to_current {
            return None;
        }
    }

    visited.insert(current_key, cost_to_current);
    let walk = crossroads.get(current).unwrap();
    let mut shortest_path = None;
    for (new_direction, extra_costs) in [(*direction, 0), (direction.turn_left(), TURN_COST), (direction.move_90(), TURN_COST)] {
        if let Path::Visited(next_point, distance_to_next) = walk[new_direction] {
            let new_cost = cost_to_current + distance_to_next + extra_costs;
            if let Some(distance_to_end) = find_route(olympic, &next_point, &new_direction, new_cost, crossroads, visited) {
                let distance = distance_to_next + distance_to_end + extra_costs;
                if shortest_path.is_none_or(|prev_distance| distance < prev_distance) {
                    shortest_path = Some(distance);
                }
            }
        }    
    }

    shortest_path
}



#[cfg(test)]
pub mod day16_tests {
    use super::*;
    use std::{fs, str::FromStr};
    

#[test]
fn example1_1() {
    let input = 
"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
    let olympic = ReindeerOlympic::from_str(input).unwrap();
    let mut crossroads = olympic.create_crossroad_map();
    olympic.fill_crossroad_distances(&mut crossroads);
    let cost = find_route(&olympic,&olympic.start, &START_DIRECTION, 0, &crossroads, &mut CostMap::new());
    let total = cost.unwrap_or_default();
    assert_eq!(7036, total);
}


#[test]
fn example1_2() {
    let input = 
"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    let olympic = ReindeerOlympic::from_str(input).unwrap();
    let mut crossroads = olympic.create_crossroad_map();
    olympic.fill_crossroad_distances(&mut crossroads);
    let cost = find_route(&olympic,&olympic.start, &START_DIRECTION, 0, &crossroads, &mut CostMap::new());
    let total = cost.unwrap_or_default();
    assert_eq!(11048, total);
}


#[test]
fn part1() {
    let input = fs::read_to_string("src/day16/input.txt").unwrap();
    let olympic = ReindeerOlympic::from_str(&input).unwrap();
    let mut crossroads = olympic.create_crossroad_map();
    olympic.fill_crossroad_distances(&mut crossroads);
    let cost = find_route(&olympic,&olympic.start, &START_DIRECTION, 0, &crossroads, &mut CostMap::new());
    let total = cost.unwrap_or_default();
    assert_eq!(115500, total);
}


#[test]
fn example2_1() {
    let input = 
"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
    // find start position
    // position: [TOP, RIGHT, BOTTOM, LEFT]
    //           NONE, NONE, NONE, SOME(next crossroad, cost)

    let olympic = ReindeerOlympic::from_str(input).unwrap();
    let mut crossroads = olympic.create_crossroad_map();
    olympic.fill_crossroad_distances(&mut crossroads);
    let cost = find_route(&olympic,&olympic.start, &START_DIRECTION, 0, &crossroads, &mut CostMap::new());
    // wall tiles on route
    let total = 0;
    assert_eq!(64, total);
}



#[test]
fn example2_2() {
    let input = 
"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    let olympic = ReindeerOlympic::from_str(input).unwrap();
    let mut crossroads = olympic.create_crossroad_map();
    olympic.fill_crossroad_distances(&mut crossroads);
    let cost = find_route(&olympic,&olympic.start, &START_DIRECTION, 0, &crossroads, &mut CostMap::new());
    // wall tiles on route
    let total = 0;
    assert_eq!(64, total);
}


// #[test]
// fn part2() {
//     let input = fs::read_to_string("src/day14/input.txt").unwrap();
//     let robots = create_robots(&input);
//     let gridsize = Position{x: 101, y: 103};
//     let mut stdout = std::io::stdout();

//     let robot_positions: Vec<Position> = robots.iter()
//                                                 .map(|r| r.move_n_within_grid(IDX_OF_TREE, &gridsize))
//                                                 .collect();

//     // save positions to file
//     match fs::exists(IMAGE_FOLDER) {
//         Ok(false) => fs::create_dir(IMAGE_FOLDER).unwrap(),
//         _ => {},
//     }
//     let mut img = RgbImage::from_pixel(gridsize.x.try_into().unwrap(), gridsize.y.try_into().unwrap(), Rgb([0, 0, 0]));
//     robot_positions.iter().for_each(|p| img.put_pixel(p.x.try_into().unwrap(), p.y.try_into().unwrap(), Rgb([31, 118, 5])));
//     img.save(format!("{}/img{}.png", IMAGE_FOLDER, IDX_OF_TREE)).unwrap();
    
//     // show positions in console (shift content up 30 lines to show full image)
//     stdout.queue(terminal::Clear(terminal::ClearType::All)).unwrap()
//           .queue(cursor::MoveTo(110,0)).unwrap()
//           .queue(style::PrintStyledContent(format!("MAP {}", IDX_OF_TREE).magenta())).unwrap();
//     for y in 30..gridsize.y {
//         for x in 0..gridsize.x {
//             stdout.queue(cursor::MoveTo(x.try_into().unwrap(), (y - 30).try_into().unwrap())).unwrap();
//             if robot_positions.contains(&Position{x: x, y: y}) {
//                 stdout.queue(style::PrintStyledContent("█".green())).unwrap();
//             } else {
//                 stdout.queue(style::Print(" ")).unwrap();
//             }
//         }
//     }
//     stdout
//         .queue(cursor::MoveToNextLine(1)).unwrap()
//         .flush().unwrap();

//     assert_eq!(6577, IDX_OF_TREE);
// }

}
