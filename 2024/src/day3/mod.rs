use std::fs;
use regex::Regex;


fn calc(input: &str) -> i32 {
    let mut result: i32 = 0;
    for (_, [a,b]) in Regex::new(r"mul\((\d+),(\d+)\)").unwrap().captures_iter(input).map(|c| c.extract()) {
        let first: i32 = a.parse().unwrap();
        let second: i32 = b.parse().unwrap();
        result += first * second;
    }
    result
}


fn calc_with_preinstruction(input: &str) -> i32 {
    let mut result: i32 = 0;
    let mut instruction_enabled = true;
    for capture in Regex::new(r"(?<instr>do|don't|mul)\(((\d+),(\d+))?\)").unwrap().captures_iter(input) {
        match &capture["instr"] {
            "do" => { instruction_enabled = true; },
            "don't" => { instruction_enabled = false; },
            "mul" => {
                if instruction_enabled {
                    let first: i32 = capture[3].parse().unwrap();
                    let second: i32 = capture[4].parse().unwrap();
                    result += first * second;
                }
            },
            _ => {}
        }
    }
    result
}



#[cfg(test)]
pub mod day3_tests {
    use super::*;

#[test]
fn example1() {
    let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    // (2*4 + 5*5 + 11*8 + 8*5)
    assert_eq!(161, calc(input));
}


#[test]
fn part1() {
    let input = fs::read_to_string("src/day3/input.txt").unwrap();
    assert_eq!(164730528, calc(&input));
}


#[test]
fn example2() {
    let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    // (2*4 + 8*5)
    assert_eq!(48, calc_with_preinstruction(input));
}


#[test]
fn part2() {
    let input = fs::read_to_string("src/day3/input.txt").unwrap();
    assert_eq!(70478672, calc_with_preinstruction(&input));
}

}