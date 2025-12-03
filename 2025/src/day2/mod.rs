use std::{char, ops::RangeInclusive};


fn calculate_invalid_sum(input: &str) -> u64 {
    input.split(',')
            .map(|pair| {
                let (start, end) = pair.split_once('-').unwrap();
                let range: RangeInclusive<i64> = start.parse().unwrap()..=end.parse().unwrap();
                range
            }).map(|range| {
                let invalid_sum: i64 = range
                    .filter(|id| {
                        let id_str = id.to_string();
                        let digits: Vec<char> = id_str.chars().collect();
                        let mid = digits.len() / 2;
                        *(&digits[0..mid].eq(&digits[mid..]))
                    })
                    .sum();
                invalid_sum as u64
            })
            .sum::<u64>()
}


fn calculate_invalid_sum2(input: &str) -> u64 {
    input.split(',')
            .map(|pair| {
                let (start, end) = pair.split_once('-').unwrap();
                let range: RangeInclusive<i64> = start.parse().unwrap()..=end.parse().unwrap();
                range
            }).map(|range| {
                let invalid_sum: i64 = range
                    .filter(|id| {
                        let id_str = id.to_string();
                        let digits: Vec<char> = id_str.chars().collect();
                        let mid = digits.len() / 2;
                        (1..=mid).into_iter()
                            .any(|i| {
                                digits.chunks(i)
                                      .clone()
                                      .reduce(|a, b| if a == b { a } else { &[' ';0] })
                                      .unwrap_or_default()
                                      .is_empty() == false
                            })
                    })
                    // .inspect(|x|println!("has invalid IDs: {:?}", x))
                    .sum();
                invalid_sum as u64
            })
            .sum::<u64>()
}


#[cfg(test)]
pub mod day2_tests {
    use super::*;
    use std::fs;

#[test]
fn example1_1() {
    let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    let total = calculate_invalid_sum(input);
    assert_eq!(1227775554u64, total);
/*
11-22 has two invalid IDs, 11 and 22.
95-115 has one invalid ID, 99.
998-1012 has one invalid ID, 1010.
1188511880-1188511890 has one invalid ID, 1188511885.
222220-222224 has one invalid ID, 222222.
1698522-1698528 contains no invalid IDs.
446443-446449 has one invalid ID, 446446.
38593856-38593862 has one invalid ID, 38593859.
The rest of the ranges contain no invalid IDs.
*/
}


#[test]
fn example2() {
    let input = "11-22";
    let total = calculate_invalid_sum2(input);
    assert_eq!(33, total);
    let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    let total = calculate_invalid_sum2(input);
    assert_eq!(4174379265, total);
/*
11-22 still has two invalid IDs, 11 and 22.
95-115 now has two invalid IDs, 99 and 111.
998-1012 now has two invalid IDs, 999 and 1010.
1188511880-1188511890 still has one invalid ID, 1188511885.
222220-222224 still has one invalid ID, 222222.
1698522-1698528 still contains no invalid IDs.
446443-446449 still has one invalid ID, 446446.
38593856-38593862 still has one invalid ID, 38593859.
565653-565659 now has one invalid ID, 565656.
824824821-824824827 now has one invalid ID, 824824824.
2121212118-2121212124 now has one invalid ID, 2121212121.
Adding up all the invalid IDs in this example produces 4174379265.
*/
}


#[test]
fn part1() {
    // the number of times the dial is left pointing at 0 after any rotation in the sequence
    let input = fs::read_to_string("src/day2/input1.txt").unwrap();
    let total = calculate_invalid_sum(input.as_str());
    assert_eq!(15873079081, total);
}


#[test]
fn part2() {
    // the number of times the dial is left pointing at 0 after any rotation in the sequence
    let input = fs::read_to_string("src/day2/input1.txt").unwrap();
    let total = calculate_invalid_sum2(input.as_str());
    assert_ne!(15873079081, total);
    assert_eq!(22617871034, total);
}

}