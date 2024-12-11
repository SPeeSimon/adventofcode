use std::ops::Mul;
use std::collections::HashMap;


struct Cache {
    cache: HashMap<(i64, i32), i64>,
    cache_entries: i32,
}


impl Cache {
    fn new() -> Self {
        Cache { cache: HashMap::new(), cache_entries: 5 }
    }

    fn cache_for(&self, blink: i32) -> bool {
        blink % self.cache_entries == 0
    }

    fn check_entry(&self, blink: i32, stone: i64) -> Option<&i64> {
        if self.cache_for(blink) {
            return self.cache.get(&(stone, blink));
        }
        None
    }

    fn cache_entry(&mut self, blink: i32, stone: i64, value: i64) {
        if self.cache_for(blink) {
            self.cache.insert((stone, blink), value);
        }
    }
}


fn split_on_blink(stone: i64) -> Vec<i64> {
    let num_of_digits = 1 + stone.ilog10();
    if num_of_digits % 2 == 0 {
        let split_by = 10_i64.pow((num_of_digits/2).try_into().unwrap());
        vec![stone / split_by, stone % split_by]
    } else {
        vec![stone.mul(2024)]
    }
}


fn reshuffle_on_blink(blink: i32, line_of_stones: Vec<i64>) -> Vec<i64> {
    if blink <= 0 {
        return line_of_stones;
    }

    let new_line_of_stones = line_of_stones.iter()
                                                    .flat_map(|&digit| {
                                                        if digit == 0 {
                                                            return vec![1];
                                                        }
                                                        split_on_blink(digit)
                                                    })
                                                    .collect();
    // println!(""After {blink} blinks remaining: {:?}", new_line_of_stones);
    reshuffle_on_blink(blink - 1, new_line_of_stones)
}


fn count_stones_on_blink_with_cache(blink: i32, line_of_stones: Vec<i64>, cache: &mut Cache) -> i64 {
    let mut total = 0;
    
    if blink <= 0 {
        return line_of_stones.len().try_into().unwrap();
    }

    for stone in line_of_stones {
        if let Some(cached_value) = cache.check_entry(blink, stone) {
            total += cached_value;
            continue;
        }

        let total_of_entry = if stone == 0 {
                                    count_stones_on_blink_with_cache(blink - 1, vec![1], cache)
                                  }
                                  else if stone == 1 {
                                    count_stones_on_blink_with_cache(blink - 1, vec![2024], cache)
                                  }
                                  else {
                                    count_stones_on_blink_with_cache(blink - 1, split_on_blink(stone), cache)
                                  };
        cache.cache_entry(blink, stone, total_of_entry);
        total += total_of_entry;
    }

    total
}


fn count_stones_on_blink(blink: i32, mut line_of_stones: Vec<i64>) -> i64 {
    let mut cache = Cache::new();
    line_of_stones.sort();
    count_stones_on_blink_with_cache(blink, line_of_stones, &mut cache)
}



#[cfg(test)]
pub mod day11_tests {
    use std::fs;
    use super::*;


#[test]
fn example1() {
    let input = "0 1 10 99 999";
    // stone: change number || split + all other shift
    // engraved 0 > 1, even digits > 2 stones (left 1/2, right 1/2 => 1000 => 10 & 0), replace by new (old # * 1024)
    let line_of_stones: Vec<i64> = input.split_whitespace().map(|nr| nr.parse().unwrap()).collect();
    let reshuffeled_stones = reshuffle_on_blink(1, line_of_stones.clone());
    assert_eq!(vec![1, 2024, 1, 0, 9, 9, 2021976], reshuffeled_stones);
    assert_eq!(7, reshuffeled_stones.len());
    assert_eq!(7, count_stones_on_blink(1, line_of_stones));
}



#[test]
fn example1_2() {
    let input = "125 17";
    let line_of_stones: Vec<i64> = input.split_whitespace().map(|nr| nr.parse().unwrap()).collect();
    assert_eq!(vec![253000, 1, 7], reshuffle_on_blink(1, line_of_stones.clone()));
    assert_eq!(vec![253, 0, 2024, 14168], reshuffle_on_blink(2, line_of_stones.clone()));
    assert_eq!(vec![512072, 1, 20, 24, 28676032], reshuffle_on_blink(3, line_of_stones.clone()));
    assert_eq!(vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032], reshuffle_on_blink(4, line_of_stones.clone()));
    assert_eq!(vec![1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32], reshuffle_on_blink(5, line_of_stones.clone()));
    assert_eq!(vec![2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6, 0, 3, 2], reshuffle_on_blink(6, line_of_stones.clone()));
}


#[test]
fn example1_3() {
    let input = "125 17";
    let line_of_stones: Vec<i64> = input.split_whitespace().map(|nr| nr.parse().unwrap()).collect();
    assert_eq!(55312, reshuffle_on_blink(25, line_of_stones.clone()).len());
    assert_eq!(55312, count_stones_on_blink(25, line_of_stones));
}


#[test]
fn part1() {
    let input = fs::read_to_string("src/day11/input.txt").unwrap();
    let line_of_stones: Vec<i64> = input.split_whitespace().map(|nr| nr.parse().unwrap()).collect();
    let total = reshuffle_on_blink(25, line_of_stones.clone()).len();
    assert_eq!(182081, total);
    assert_eq!(182081, count_stones_on_blink(25, line_of_stones));
}


#[test]
fn part2() {
    let input = fs::read_to_string("src/day11/input.txt").unwrap();
    let line_of_stones: Vec<i64> = input.split_whitespace().map(|nr| nr.parse().unwrap()).collect();
    let total = count_stones_on_blink(75, line_of_stones);
    assert_eq!(216318908621637, total);
}

}
