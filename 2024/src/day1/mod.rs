use std::fs;


pub fn parse_input_pair(input: &str) -> (Vec<u64>, Vec<u64>) {
    let mut left = Vec::<u64>::new();
    let mut right = Vec::<u64>::new();

    input.lines()
         .filter(|line| line.len() > 0)
         .map(|line| line.split_whitespace())
         .map(|mut line| (line.next().unwrap().parse().unwrap(), line.next().unwrap().parse().unwrap()))
         .for_each(|(l, r): (u64, u64)| {
            left.push(l);
            right.push(r);
         });

    left.sort();
    right.sort();
    (left, right)
}


pub fn calc_difference(input: &str) -> u64 {
    let (left, right) = parse_input_pair(input);
    let total_diff = left.iter()
                         .zip(right.iter())
                         .map(|(&l, &r)| r.abs_diff(l))
                         .sum();
    total_diff
}


fn count_items(list: &Vec<u64>, item: u64) -> u64 {
    list.iter()
        .filter(|&l| *l == item)
        .count() as u64
}


pub fn calc_similarity(input: &str) -> u64 {
    let (left, right) = parse_input_pair(input);
    let total_diff = left.iter()
                         .map(|&l| l.checked_mul(count_items(&right, l)).unwrap())
                         .sum();
    total_diff
}


#[cfg(test)]
pub mod day1_tests {
    use super::*;

#[test]
fn example1() {
    let input =
"3   4
4   3
2   5
1   3
3   9
3   3";
    let total_diff = calc_difference(input);
    println!("total diff: {}", total_diff);
    assert_eq!(11, total_diff);
}


#[test]
fn part1() {
    let input = fs::read_to_string("src/day1/input1.txt").unwrap();
    let total_diff = calc_difference(&input);
    println!("total diff: {}", total_diff);
    assert_eq!(1506483, total_diff);
}


#[test]
fn example2() {
    let input =
"3   4
4   3
2   5
1   3
3   9
3   3";
    let total_diff = calc_similarity(input);
    println!("total diff: {}", total_diff);
    assert_eq!(31, total_diff);
}



#[test]
fn part2() {
    let input = fs::read_to_string("src/day1/input1.txt").unwrap();
    let total_diff = calc_similarity(&input);
    println!("total diff: {}", total_diff);
    assert_eq!(23126924, total_diff);
}

}