use crate::utils::{grid::Grid, Move};
use crate::utils::position::Position;


struct Track {
    value: char,
    position: Position,
}


#[cfg(test)]
pub mod day14_tests {
    use super::*;
    use std::{fs, str::FromStr};
    

#[test]
fn example1s() {
    let input = 
"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

// result:
// ########
// #....OO#
// ##.....#
// #.....O#
// #.#O@..#
// #...O..#
// #...O..#
// ########


// The Goods Positioning System (GPS) coordinate of a box = 100 * distance from the top edge of the map + distance from the left edge of the map
// So, the box shown below has a distance of 1 from the top edge of the map and 4 from the left edge of the map, resulting in a GPS coordinate of 100 * 1 + 4 = 104.
// #######
// #...O..
// #......
// sum of all boxes' GPS coordinates after the robot finishes moving.
    let (grid, movements) = input.split_once("\n\n").unwrap();
    let mut warehouse = Grid::from_str(grid).unwrap();
    let mut robot_position = warehouse.find_all_positions(|c| *c == '@')[0];
    execute_movements(movements, &mut warehouse, robot_position);
    warehouse.print();
    let total = goods_positioning_all(&warehouse);
    assert_eq!(2028, total);
}


fn execute_movements(movements: &str, warehouse: &mut Grid<char>, mut robot_position: Position) {
    for mv in movements.chars() {
        if let Some(robot_moving_direction) = Move::from_char(&mv) {
            if let Some(new_position) = move_position(warehouse, &Track { value: '@', position: robot_position }, &robot_moving_direction) {
                robot_position = new_position;
            }
        }
    }
}


fn goods_positioning_all(warehouse: &Grid<char>) -> usize {
    warehouse.find_all_positions(|&c| c == 'O').iter()
             .map(|good_position| goods_positioning_system(good_position))
             .sum()
}


fn goods_positioning_system(item: &Position) -> usize {
    100 * item.y + item.x
}


fn move_position(warehouse: &mut Grid<char>, item: &Track, mv: &Move) -> Option<Position> {
    if let Some(next_position) = item.position.move_to(mv) {
        match warehouse.at(&next_position).unwrap() {
            '.' => {
                warehouse.set(&item.position, '.');
                warehouse.set(&next_position, item.value);
                return Some(next_position);
            },
            '#' => {
                // cannot move
            },
            'O' => {
                // try and move box
                if let Some(moved_next) = move_position(warehouse, &Track { value: 'O', position: next_position }, mv) {
                    warehouse.set(&item.position, '.');
                    warehouse.set(&next_position, item.value);
                    return Some(next_position);    
                }
            }, 
            _ => {}
        }
    }
    None
}


#[test]
fn example1() {
    let input = 
"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

// result:
// ##########
// #.O.O.OOO#
// #........#
// #OO......#
// #OO@.....#
// #O#.....O#
// #O.....OO#
// #O.....OO#
// #OO....OO#
// ##########

    let (grid, movements) = input.split_once("\n\n").unwrap();
    let mut warehouse = Grid::from_str(grid).unwrap();
    let mut robot_position = warehouse.find_all_positions(|c| *c == '@')[0];
    execute_movements(movements, &mut warehouse, robot_position);
    warehouse.print();
    let total = goods_positioning_all(&warehouse);
    assert_eq!(10092, total);
}


#[test]
fn part1() {
    let input = fs::read_to_string("src/day15/input.txt").unwrap();
    let (grid, movements) = input.split_once("\n\n").unwrap();
    let mut warehouse = Grid::from_str(grid).unwrap();
    let mut robot_position = warehouse.find_all_positions(|c| *c == '@')[0];
    execute_movements(movements, &mut warehouse, robot_position);
    let total = goods_positioning_all(&warehouse);
    assert_eq!(1398947, total);
}



#[test]
fn example2() {
    let input = 
"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

// result:
// ####################
// ##[].......[].[][]##
// ##[]...........[].##
// ##[]........[][][]##
// ##[]......[]....[]##
// ##..##......[]....##
// ##..[]............##
// ##..@......[].[][]##
// ##......[][]..[]..##
// ####################

    let (grid, movements) = input.split_once("\n\n").unwrap();
    let mut warehouse = Grid::from_str(grid).unwrap();
    let mut robot_position = warehouse.find_all_positions(|c| *c == '@')[0];
    execute_movements(movements, &mut warehouse, robot_position);
    warehouse.print();
    let total = goods_positioning_all(&warehouse);
    assert_eq!(10092, total);
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
