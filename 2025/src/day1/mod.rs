use std::{char, fs, ops::RangeInclusive};


const _DIAL: RangeInclusive<i32> = 0..=99;
const DIAL_START_POSITION: i32 = 50;


fn rotate(current_position: i32, rotation: &str) -> i32 {
    let (direction, amount_str) = rotation.split_at(1);
    let amount: i32 = amount_str.parse().unwrap();

    match direction {
        "R" => {
            let new_position = current_position + amount;
            new_position % 100
        },
        "L" => {
            let new_position = current_position + 100 - (amount % 100);
            new_position % 100
        },
        _ => panic!("Invalid rotation direction"),
    }
}

fn rotate_pass_zero(current_position: i32, rotation: &str) -> (i32, i32) {
    let (direction, amount_str) = rotation.split_at(1);
    let amount: i32 = amount_str.parse().unwrap();

    match direction {
        "R" => {
            let new_position = current_position + amount;
            (new_position % 100, ((new_position-1) / 100))
        },
        "L" => {
            let diff = amount % 100;

            if diff <= current_position {
                return (current_position - diff, amount / 100);
            } else {
                let passes = if current_position == 0 {0} else {1} + (amount / 100);
                let new_position = (current_position + 100 - diff) % 100;
                return (new_position, passes);
            }

            // let new_position = current_position + 100 - (amount % 100);
            // // 5-10; 95
            // if new_position > current_position && current_position != 0 {
            //     return (new_position % 100, (amount - (current_position + 1)) / 100 + 1);
            // }
            // (new_position % 100, amount / 100)
        },
        _ => panic!("Invalid rotation direction"),
    }
}



#[cfg(test)]
pub mod day1_tests {
    use super::*;

#[test]
fn example1_1() {
    let result = rotate(11, "R8");
    assert_eq!(19, result);
    let result = rotate(19, "L19");
    assert_eq!(0, result);
    let result = rotate(5, "L10");
    assert_eq!(95, result);
    let result = rotate(95, "R5");
    assert_eq!(0, result);
    // the number of times the dial is left pointing at 0 after any rotation in the sequence
}

#[test]
fn example1_2() {
    let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    let mut position = DIAL_START_POSITION;
    let mut zero_count = 0;
    input.lines().for_each(|line| {
        position = rotate(position, line);
        if position == 0 {
            zero_count += 1;
        }
        println!("The dial is rotated {} to point at {}", line, position);
    });
    let result = zero_count;
// The dial starts by pointing at 50.
// The dial is rotated L68 to point at 82.
// The dial is rotated L30 to point at 52.
// The dial is rotated R48 to point at 0.
// The dial is rotated L5 to point at 95.
// The dial is rotated R60 to point at 55.
// The dial is rotated L55 to point at 0.
// The dial is rotated L1 to point at 99.
// The dial is rotated L99 to point at 0.
// The dial is rotated R14 to point at 14.
// The dial is rotated L82 to point at 32.
    assert_eq!(3, result);
}

#[test]
fn part1() {
    // the number of times the dial is left pointing at 0 after any rotation in the sequence
    let input = fs::read_to_string("src/day1/input1.txt").unwrap();
    let mut position = DIAL_START_POSITION;
    let mut zero_count = 0;
    input.lines().for_each(|line| {
        position = rotate(position, line);
        if position == 0 {
            zero_count += 1;
        }
        println!("The dial is rotated {} to point at {}", line, position);
    });
    let result = zero_count;
    assert_eq!(1023, result);
}


// password method 0x434C49434B
#[test]
fn example2_1() {
    assert_eq!((50,10), rotate_pass_zero(50, "R1000"));
    assert_eq!((50,10), rotate_pass_zero(50, "L1000"));
    assert_eq!((30,1), rotate_pass_zero(50, "R80"));
    assert_eq!((95,1), rotate_pass_zero(50, "L55"));
    assert_eq!((5,0), rotate_pass_zero(0, "R5"));
    assert_eq!((95,0), rotate_pass_zero(0, "L5"));
    assert_eq!((0,0), rotate_pass_zero(95, "R5"));
    assert_eq!((0,0), rotate_pass_zero(5, "L5"));
}

#[test]
fn example2_2() {
    let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    let mut position = DIAL_START_POSITION;
    let mut zero_count = 0;
    input.lines().for_each(|line| {
        let rotate_adjustment = rotate_pass_zero(position, line);
        position = rotate_adjustment.0;
        if position == 0 {
            zero_count += 1;
            zero_count += rotate_adjustment.1;
        } else {
            zero_count += rotate_adjustment.1;
        }
        println!("The dial is rotated {} to point at {} (at zero {})", line, position, rotate_adjustment.1);
    });
    let result = zero_count;
// The dial starts by pointing at 50.
// The dial is rotated L68 to point at 82; during this rotation, it points at 0 once.
// The dial is rotated L30 to point at 52.
// The dial is rotated R48 to point at 0.
// The dial is rotated L5 to point at 95.
// The dial is rotated R60 to point at 55; during this rotation, it points at 0 once.
// The dial is rotated L55 to point at 0.
// The dial is rotated L1 to point at 99.
// The dial is rotated L99 to point at 0.
// The dial is rotated R14 to point at 14.
// The dial is rotated L82 to point at 32; during this rotation, it points at 0 once.
    assert_ne!(9, result);
    assert_eq!(6, result);
}

#[test]
fn part2() {
    // the number of times the dial is left pointing at 0 after any rotation in the sequence
    let input = fs::read_to_string("src/day1/input1.txt").unwrap();
    let mut position = DIAL_START_POSITION;
    let mut zero_count = 0;
    input.lines().for_each(|line| {
        let rotate_adjustment = rotate_pass_zero(position, line);
        position = rotate_adjustment.0;
        if position == 0 {
            zero_count += 1;
            zero_count += rotate_adjustment.1;
        } else {
            zero_count += rotate_adjustment.1;
        }
        println!("The dial is rotated {} to point at {} (at zero {})", line, position, rotate_adjustment.1);
    });
    let result = zero_count;
    assert_ne!(1023, result);
    assert_ne!(5900, result); // high
    assert_ne!(4876, result); // low
    assert_eq!(5899, result);
}


}