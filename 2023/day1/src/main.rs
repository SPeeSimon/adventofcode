use std::{char, fs};



fn first_digit(line: &str) -> char {
    match line.find(char::is_numeric) {
        Some(i) => line.chars().nth(i).unwrap(),
        None => '0',
    }
}

fn last_digit(line: &str) -> char {
    match line.rfind(char::is_numeric) {
        Some(i) => line.chars().nth(i).unwrap(),
        None => '0',
    }
}

fn concat_to_digit(first: char, last: char) -> u32 {
    (first.to_digit(10).unwrap() * 10) + last.to_digit(10).unwrap()
}

fn calibrate_values1(input: &str) -> u32 {
    input.lines()
         .map(|line| concat_to_digit(first_digit(&line), last_digit(&line)))
         .sum()
}

fn calibrate_values2(input: &str) -> u32 {
    input.lines()
         .map(|line| line.replace("one", "o1e")
                                .replace("two", "t2o")
                                .replace( "three", "t3e")
                                .replace( "four", "f4r")
                                .replace( "five", "f5e")
                                .replace( "six", "s6x")
                                .replace( "seven", "s7n")
                                .replace( "eight", "e8t")
                                .replace("nine", "n9e"))
         .map(|line| concat_to_digit(first_digit(&line), last_digit(&line)))
         .sum()
}


// fn calibrate_values3(input: &str) {
//     let VALUE_REPLACEMENTS: Vec<(&str, i32)> = vec![("one", 1),
//                                                     ("two", 2),
//                                                     ("three", 3),
//                                                     ("four", 4),
//                                                     ("five", 5),
//                                                     ("six", 6),
//                                                     ("seven", 7),
//                                                     ("eight", 8),
//                                                     ("nine", 9)];

//     let mut first_index: (usize, i32) = (0, 0);
//     let mut last_index: (usize, i32) = (0, 0);


//     let first = VALUE_REPLACEMENTS.clone().into_iter()
//                       .map(|replc| (input.find(replc.0), replc.1))
//                       .filter(|fnd| fnd.0.is_some())
//                       .min_by(|a,b| a.0.cmp(&b.0));

//     let last = VALUE_REPLACEMENTS.clone().into_iter()
//                       .map(|replc| (input.rfind(replc.0), replc.1))
//                       .filter(|fnd| fnd.0.is_some())
//                       .max_by(|a,b| a.0.cmp(&b.0));

//     for replacement in VALUE_REPLACEMENTS {
//         if let Some(i) = input.find(replacement.0) {
//             if first_index.1 > i.try_into().unwrap() {
//                 first_index = (i, replacement.1);
//             }
//         }
//         if let Some(i) = input.rfind(replacement.0) {
//             if last_index.1 < i.try_into().unwrap() {
//                 last_index = (i, replacement.1);
//             }
//         };
//     }

// }




#[test]
fn example1() {

    let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    for line in input.lines() {
        println!("{} => {:?} = {}", line, [first_digit(&line), last_digit(&line)], concat_to_digit(first_digit(&line), last_digit(&line)));
    }

    println!("total: {}", calibrate_values1(input));
    assert_eq!(142, calibrate_values1(input));
}

#[test]
fn part1() {
    let input = fs::read_to_string("src/input1.txt").unwrap();
    println!("total: {}", calibrate_values1(input.as_str()));
}

#[test]
fn example2() {

    let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    for line in input.lines() {
        let line2 = line.replace("one", "o1e")
                                .replace("two", "t2o")
                                .replace( "three", "t3e")
                                .replace( "four", "f4r")
                                .replace( "five", "f5e")
                                .replace( "six", "s6x")
                                .replace( "seven", "s7n")
                                .replace( "eight", "e8t")
                                .replace("nine", "n9e");
        println!("{} = {} => {:?} = {}", line, line2, [first_digit(&line2), last_digit(&line2)], concat_to_digit(first_digit(&line2), last_digit(&line2)));
    }

    println!("total: {}", calibrate_values2(input));
    assert_eq!(281, calibrate_values2(input));
}


#[test]
fn part2() {
    let input = fs::read_to_string("src/input2.txt").unwrap();
    println!("total: {}", calibrate_values2(input.as_str()));
}
