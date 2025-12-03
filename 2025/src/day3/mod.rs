

fn find_largest_joltage(bank: &str) -> u64 {
    let batteries: Vec<u32> = bank.chars().map(|x| x.to_digit(10).unwrap()).collect();

    (1..batteries.len()).into_iter()
        .map(|i| {
            let left = &batteries[0..i];
            let right = &batteries[i..];
            let joltage = left.iter().max().unwrap_or(&0) * 10 + right.iter().max().unwrap_or(&0);
            joltage
        })
        .max()
        .unwrap_or(0) as u64
}


fn find_largest_joltage12(bank: &str) -> u64 {
    let mut batteries: Vec<u32> = bank.chars().map(|x| x.to_digit(10).unwrap()).collect();
    
    let max = batteries.iter().take(batteries.len() - 12).max().unwrap();
    let remove_before_idx = batteries.iter().position(|x| *x == *max).unwrap_or_default();
    if remove_before_idx > 0 {
        batteries.drain(0..remove_before_idx);
    }
    
    let mut idx = 0;
    'check: while batteries.len() > 12 {
        while idx + 1 < batteries.len() {
            if batteries[idx] < batteries[idx + 1] {
                batteries.remove(idx);
                idx -= 1;
                continue 'check;
            }
            idx += 1;
        }
        batteries.pop();
    }

    let joltage: u64 = batteries.iter().fold(0u64, |acc, &x| acc * 10 + x as u64);
    joltage
}



#[cfg(test)]
pub mod day3_tests {
    use super::*;
    use std::fs;

#[test]
fn example1_1() {
    let input = "987654321111111
811111111111119
234234234234278
818181911112111";

    let total = input.lines() 
         .map(|line| {
             find_largest_joltage(line)
         })
         .inspect(|x| println!("Bank has max joltage: {:?}", x))
         .sum();
// In 987654321111111, you can make the largest joltage possible, 98, by turning on the first two batteries.
// In 811111111111119, you can make the largest joltage possible by turning on the batteries labeled 8 and 9, producing 89 jolts.
// In 234234234234278, you can make 78 by turning on the last two batteries (marked 7 and 8).
// In 818181911112111, the largest joltage you can produce is 92.
// The total output joltage is the sum of the maximum joltage from each bank, so in this example, the total output joltage is 98 + 89 + 78 + 92 = 357.
    assert_eq!(357u64, total);
}


#[test]
fn part1() {
    let input = fs::read_to_string("src/day3/input1.txt").unwrap();
        let total = input.lines() // bank
         .map(|line| {
             find_largest_joltage(line)
         })
         .sum();
    assert_eq!(17524u64, total);
}


#[test]
fn example2_1() {
    let input = "987654321111111
811111111111119
234234234234278
818181911112111";
    
    let total = input.lines()
         .map(|line| {
             find_largest_joltage12(line)
         })
         .inspect(|x| println!("Bank has max joltage: {:?}", x))
         .sum();
// In 987654321111111, the largest joltage can be found by turning on everything except some 1s at the end to produce 987654321111.
// In the digit sequence 811111111111119, the largest joltage can be found by turning on everything except some 1s, producing 811111111119.
// In 234234234234278, the largest joltage can be found by turning on everything except a 2 battery, a 3 battery, and another 2 battery near the start to produce 434234234278.
// In 818181911112111, the joltage 888911112111 is produced by turning on everything except some 1s near the front.
// The total output joltage is now much larger: 987654321111 + 811111111119 + 434234234278 + 888911112111 = 3121910778619.
    assert_eq!(3121910778619u64, total);
}


#[test]
fn part2() {
    let input = fs::read_to_string("src/day3/input1.txt").unwrap();
        let total = input.lines()
         .map(|line| {
             find_largest_joltage12(line)
         })
         .sum();
    assert_ne!(17524u64, total);
    assert_ne!(171959539665906u64, total); // too low
    assert_eq!(173848577117276u64, total);
}

}
