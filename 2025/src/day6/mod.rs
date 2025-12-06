use core::num;
use std::{collections::HashMap, str::{Chars, FromStr}};


struct MathWorksheet {
    numbers: Vec<Vec<i32>>,
    operations: Vec<char>,
}


impl FromStr for MathWorksheet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut numbers = Vec::new();
        let mut operations = Vec::new();

        for line in s.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            if line.chars().all(|c| !c.is_digit(10) || c.is_whitespace()) {
                for c in line.chars() {
                    if c == '+' || c == '*' {
                        operations.push(c);
                    }
                }
            } else {
                let row: Vec<i32> = line
                    .split_whitespace()
                    .filter_map(|num_str| num_str.parse::<i32>().ok())
                    .collect();
                numbers.push(row);
            }
        }

        assert!(numbers.first().unwrap().len() == operations.len(), "Mismatch between numbers and operations");

        Ok(MathWorksheet { numbers, operations })
    }
}


impl MathWorksheet {
    pub fn get_calc_data(&self, column: usize) -> (char, Vec<i32>) {
        let mut numbers = Vec::new();
        for row in &self.numbers {
            numbers.push(row[column]);
        }
        (self.operations[column], numbers)
    }


    pub fn parse_and_calculate(input: &str) -> u64 {
        let mut lines: Vec<String> = Vec::new();
        let mut total = 0;

        for line in input.lines() {
            if line.contains(|c: char| !c.is_ascii_digit() && !c.is_whitespace()) {
                continue;
            }
            for (index, char) in line.char_indices() {
                match lines.get_mut(index) {
                    None => lines.push(char.to_string()),
                    Some(l) => l.push(char),
                }
            }
        }

        let mut operations = input.lines().last().unwrap_or_default().chars().filter(|c|!c.is_whitespace());
        let mut line_iter = lines.iter();
        while let Some(op) = operations.next() {
            let mut result = Vec::new();
            loop {
                match line_iter.next() {
                    None => break,
                    Some(a) if a.trim().len() == 0 => break,
                    Some(nr) => result.push(nr.trim().parse().unwrap()),
                }
            }
            match op {
                '+' => {
                    let col_sum: u64 = result.iter().sum();
                    // println!("line result = {} {:?}", col_sum, result);
                    total += col_sum;
                }
                '*' => {
                    let col_product: u64 = result.iter().product();
                    // println!("line result = {} {:?}", col_product, result);
                    total += col_product;
                }
                _ => panic!("Unknown operation"),
            }
        }

        total
    }
}


fn perform_calculation(worksheet: &MathWorksheet) -> u64 {
    let num_columns = worksheet.numbers[0].len();
    let mut result = 0u64;

    for col in 0..num_columns {
        let (op, nums) = worksheet.get_calc_data(col);
        match op {
            '+' => {
                let col_sum: u64 = nums.iter().map(|&n| n as u64).sum();
                result += col_sum;
            }
            '*' => {
                let col_product: u64 = nums.iter().map(|&n| n as u64).product();
                result += col_product;
            }
            _ => panic!("Unknown operation"),
        }
    }
    result
}



#[cfg(test)]
pub mod day6_tests {
    use super::*;
    use std::fs;

#[test]
fn example1_1() {
    let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
// 123 * 45 * 6 = 33210
// 328 + 64 + 98 = 490
// 51 * 387 * 215 = 4243455
// 64 + 23 + 314 = 401
// grand total is 33210 + 490 + 4243455 + 401 = 4277556
    let worksheet: MathWorksheet = input.parse().unwrap();
    let total = perform_calculation(&worksheet);
    assert_eq!(4277556, total);
}


#[test]
fn part1() {
    let input = fs::read_to_string("src/day6/input1.txt").unwrap();
    let worksheet: MathWorksheet = input.parse().unwrap();
    let total = perform_calculation(&worksheet);
    assert_ne!(4277556, total);
    assert_eq!(4693159084994, total);
}


#[test]
fn example2_1() {
    let input = 
"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
// Reading the problems right-to-left one column at a time
// The rightmost problem is 4 + 431 + 623 = 1058
// The second problem from the right is 175 * 581 * 32 = 3253600
// The third problem from the right is 8 + 248 + 369 = 625
// Finally, the leftmost problem is 356 * 24 * 1 = 8544
// Now, the grand total is 1058 + 3253600 + 625 + 8544 = 3263827.
    let total = MathWorksheet::parse_and_calculate(input);
    assert_ne!(4277556, total);
    assert_eq!(3263827, total);
}


#[test]
fn part2() {
    let input = fs::read_to_string("src/day6/input1.txt").unwrap();
    let total = MathWorksheet::parse_and_calculate(input.as_str());
    assert_ne!(4277556, total);
    assert_ne!(4693159084994, total);
    assert_eq!(11643736116335, total);
}


}
