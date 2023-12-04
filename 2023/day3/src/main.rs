use std::{char, fs, fmt::{Display, write}, collections::{HashMap, HashSet}};


fn main(){}


#[derive(PartialEq, Debug)]
enum FieldValue {
    IGNORE,
    DIGIT(char),
    ADJACENT_DIGIT(char),
    ENGINE_PART(char),
    GEAR(char),
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Coordinate {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct GridData<T> {
    location: Coordinate,
    width: i32,
    value: T,
}


impl Coordinate {
    fn around(&self) -> Vec<Coordinate> {
        [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1,1)].map(|diff| Coordinate{x: self.x + diff.0, y: self.y + diff.1}).to_vec()
    }
}


struct FieldValueList<'a>(&'a Vec<FieldValue>);

impl Display for FieldValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldValue::IGNORE => f.write_str("."),
            FieldValue::DIGIT(d) => f.write_fmt(format_args!("{}", d)),
            FieldValue::ADJACENT_DIGIT(d) => f.write_fmt(format_args!("\x1b[93m{}\x1b[0m", d)),
            FieldValue::ENGINE_PART(p) => f.write_fmt(format_args!("{}", p)),
            FieldValue::GEAR(p) => f.write_fmt(format_args!("\x1b[93m{}\x1b[0m", p)),
        }
    }
}

impl Display for FieldValueList<'_ > {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for v in self.0 {
            write!(f, "{}", v);
        }
        Ok(())
    }
}


fn line_to_symbols(line: &str) -> Vec<FieldValue> {
    line.chars()
        .map(|c| {
            match c {
                '.' => FieldValue::IGNORE,
                '0'..='9' => FieldValue::DIGIT(c),
                c => FieldValue::ENGINE_PART(c),
            }
        }).collect()
}


fn bounding_box(x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut bbox = Vec::new();
    for mx in [0, 1, 2] {
        for my in [0, 1, 2] {
            let coord_x = x + mx - 1;
            let coord_y = y + my - 1;
            if coord_x >= 0 && coord_y >= 0 {
                bbox.push((coord_x, coord_y));
            }
        }
    }
    return bbox;
}


#[derive(Debug)]
struct EnginePartNumber {
    pub number: u32,
    pub indexes: Vec<usize>,
}


impl From<Vec<(usize, char)>> for EnginePartNumber {
    fn from(input: Vec<(usize, char)>) -> Self {
        let mut nr = 0;
        let mut indexes = Vec::new();

        for ci in input.into_iter() {
            nr = (nr * 10) + ci.1.to_digit(10).unwrap_or_default();
            indexes.push(ci.0);
        }

        EnginePartNumber { number: nr, indexes: indexes }
    }
}



// id, x, y, width, heigth=1


fn is_symbol(c: char) -> bool {
    c != '.' || !c.is_numeric()
}


fn find_numbers(line: &str) -> Vec<EnginePartNumber> {
    let mut result: Vec<EnginePartNumber> = Vec::new();
    let mut number_indexes = Vec::new();

    for nr in line.char_indices() {
        if nr.1.is_numeric() {
            number_indexes.push(nr);
        } else if !number_indexes.is_empty() {
            result.push( number_indexes.clone().into() );
            number_indexes.clear();
        }
    }
    return result;
}


fn find_numbers_with_adjecency(input: Vec<&str>) -> Vec<(bool, u32)> {
    let mut part_indicators = Vec::new();
    let mut symbol_grid = Vec::new();

    // input > symbols
    for val_loc in input.into_iter().enumerate() {
        let x = val_loc.0; // line_nr
        let data = line_to_symbols(val_loc.1);
        data.iter().enumerate()
                   .filter(|val| matches!(*val.1, FieldValue::ENGINE_PART(_)))
                   .for_each(|val| part_indicators.push((x, val.0)));
        symbol_grid.push(data);
    }

    // digit > ignored digit
    part_indicators.iter()
                   .flat_map(|coord| bounding_box(coord.0, coord.1))
                   .for_each(|coord| {
                       let target = &symbol_grid[coord.0][coord.1];
                       match target {
                           FieldValue::DIGIT(c) => {
                            symbol_grid[coord.0][coord.1] = FieldValue::ADJACENT_DIGIT(*c);
                           }
                           _ => {}
                       }
                   })
               ;
    
    symbol_grid.iter().for_each(|l| println!("{}", FieldValueList(l)));

    // only digits
    let mut result = Vec::new();
    for line in symbol_grid {
        let mut is_adjecent = false;
        let mut digit = 0;
        for x in 0..line.len() {
            match line[x] {
                FieldValue::ADJACENT_DIGIT(d) => {
                    is_adjecent = true;
                    digit = (digit * 10) + d.to_digit(10).unwrap_or_default();
                }
                FieldValue::DIGIT(d) => {
                    digit = (digit * 10) + d.to_digit(10).unwrap_or_default();
                }
                _ => {
                    if digit > 0 {
                        result.push((is_adjecent, digit));
                    }
                    is_adjecent = false;
                    digit =  0;
                }
            }
        }
        if digit > 0 {
            result.push((is_adjecent, digit));
        }
    }

    return result;
}


fn count_digits_in_bbox(x: usize, y: usize, grid: &Vec<Vec<FieldValue>>) -> i32 {
    let mut cnt_digits = 0;
    for bbox in bounding_box(x, y) {
      if matches!(&grid[bbox.0][bbox.1], FieldValue::DIGIT(_)) {
          cnt_digits += 1;
      }
    }
    return cnt_digits;
}

fn find_gears(input: Vec<&str>) -> Vec<u32> {
    let mut gear_indicators: HashMap<Coordinate, Vec<u32>> = HashMap::new();
    let mut symbol_grid = Vec::new();

    // input > symbols
    for val_loc in input.into_iter().enumerate() {
        let x = val_loc.0; // line_nr
        let data = line_to_symbols(val_loc.1);
        data.iter().enumerate()
                   .filter(|val| matches!(*val.1, FieldValue::ENGINE_PART('*')) )
                   .for_each(|val| { 
                        gear_indicators.insert(Coordinate{x: x as i32, y: val.0 as i32}, Vec::new());
                    });
        symbol_grid.push(data);
    }

    // part > gear
    // gear_indicators.iter()
    //                .for_each(|coord| {
    //                     if count_digits_in_bbox(coord.0, coord.1, &symbol_grid) > 1 {
    //                         for (x,y) in bounding_box(coord.0, coord.1) {
    //                             let target = &symbol_grid[x][y];
    //                             match target {
    //                                 FieldValue::ENGINE_PART(c) => {
    //                                      symbol_grid[x][y] = FieldValue::GEAR(*c);
    //                                 },
    //                                 FieldValue::DIGIT(c) => {
    //                                     symbol_grid[x][y] = FieldValue::ADJACENT_DIGIT(*c);
    //                                    },        
    //                                 _ => {},
    //                             }
    //                         }
    //                     }
    //                });

    // [Symbol(x,y),List]
    // find digits

    // only digits
    // let mut result = Vec::new();
    for x in 0..symbol_grid.len() {
        let line = &symbol_grid[x];
        let mut digit = 0;
        let mut location = None;
        let mut width = 0;
        let mut next_to_symbol = HashSet::new();
        // x,y + width
        for y in 0..line.len() {
            match line[y] {
                FieldValue::DIGIT(d) => {
                    digit = (digit * 10) + d.to_digit(10).unwrap_or_default();
                    width += 1;
                    if location.is_none() {
                        location = Some(Coordinate{ x: x as i32, y: y as i32});
                    }
                    
                    let pos = Coordinate{ x: x as i32, y: y as i32};
                    pos.around().iter()
                                                        .filter(|c| gear_indicators.contains_key(c))
                                                        .for_each(|c| {next_to_symbol.insert(c.clone());});
                }
                _ => {
                    if digit > 0 {
                        // append_to_gear_indicators(
                        //     location.unwrap(), width, digit, gear_indicators
                        // );
                        next_to_symbol.iter().for_each(|g| {
                            gear_indicators.entry(*g).and_modify(|e| e.push(digit));
                        });
                        // result.push(GridData{ location: location.unwrap(), width: width, value: (next_to_symbol, digit) });
                    }
                    digit = 0;
                    width = 0;
                    location = None;
                    next_to_symbol.clear();
                }
            }
        }
        if digit > 0 {
            next_to_symbol.iter().for_each(|g| {
                gear_indicators.entry(*g).and_modify(|e| e.push(digit));
            });
            // append_to_gear_indicators(
            //     location.unwrap(), width, digit, gear_indicators
            // );
            // result.push(GridData{ location: location.unwrap(), width: width, value: (next_to_symbol, digit) });
        }
    }

    gear_indicators.values()
                    .filter(|digits| digits.len() == 2)
                    .map(|digits| digits.first().unwrap() * digits.last().unwrap())
                    .collect()
    // println!("{:?}", rv );

    // return 0;
}






#[test]
fn example1() {

    let input: Vec<&str> = 
"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..".lines().collect();

    let result = find_numbers_with_adjecency(input);

    let non_adjecent: Vec<u32> = result.iter().filter(|val| !val.0).map(|val| val.1).collect();
    let adjecent: Vec<u32> = result.iter().filter(|val| val.0).map(|val| val.1).collect();
    let sum_adjecent: u32 = adjecent.iter().sum();

    // println!("values: {:?}", &result);
    assert_eq!(114, non_adjecent[0]);
    assert_eq!(58, non_adjecent[1]);
    assert_eq!(4361, sum_adjecent);
}


#[test]
fn part1() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let result = find_numbers_with_adjecency(input.lines().collect());

    let non_adjecent: Vec<u32> = result.iter().filter(|val| !val.0).map(|val| val.1).collect();
    let adjecent: Vec<u32> = result.iter().filter(|val| val.0).map(|val| val.1).collect();
    let sum_adjecent: u32 = adjecent.iter().sum();

    println!("{:?}", non_adjecent);
    println!("sum of all parts: {}", sum_adjecent);
    assert_ne!(544359, sum_adjecent);
    assert!(544359 < sum_adjecent);
    assert_ne!(1134686, sum_adjecent);
    assert!(1134686 > sum_adjecent);
    assert_eq!(546563, sum_adjecent);
}

#[test]
fn example2() {
    let input = 
"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    let result = find_gears(input.lines().collect());
    let total: u32 = result.iter().sum();

    println!("found gears: {:?}", result);
    println!("total: {}", total);
    assert_eq!(467835, total);
}


#[test]
fn part2() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let result = find_gears(input.lines().collect());
    let total: u32 = result.iter().sum();

    println!("found gears: {:?}", result);
    println!("total: {}", total);
}
