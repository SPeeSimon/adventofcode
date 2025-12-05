use std::{ops::RangeInclusive, str::FromStr};


struct Inventory {
    fresh_ranges: Vec<RangeInclusive<i64>>,
    available_ids: Vec<i64>,
}

impl FromStr for Inventory {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sections = s.split("\n\n");
        let fresh_section = sections.next().ok_or("Fresh section not found")?;
        let available_section = sections.next().ok_or("Available section not found")?;

        let mut fresh_ranges: Vec<RangeInclusive<i64>> = fresh_section
                                                            .lines()
                                                            .filter_map(|line| {
                                                                let (start, end) = line.split_once('-')?;
                                                                Some((start.parse().ok()?, end.parse().ok()?))
                                                            })
                                                            .map(|(start, end)| RangeInclusive::new(start, end))
                                                            .collect();
        fresh_ranges.sort_by(|a, b| a.start().cmp(b.start()));

        let mut available_ids: Vec<i64> = available_section
            .lines()
            .filter_map(|line| line.parse().ok())
            .collect();
        available_ids.sort_by(|a, b| a.cmp(&b));

        Ok(Inventory {
            fresh_ranges,
            available_ids,
        })
    }
}


fn check_ingredients(inventory: &Inventory) -> usize {
    let mut fresh_count = 0;

    for id in inventory.available_ids.iter() {
        let mut is_fresh = false;
        for range in inventory.fresh_ranges.iter() {
            if range.contains(&id) {
                is_fresh = true;
                break;
            }
        }
        if is_fresh {
            fresh_count += 1;
        }
    }

    fresh_count
}


fn fresh_ingredients(inventory: &Inventory) -> i64 {
    let mut count = 0;

    let mut range = None;
    for next_range in inventory.fresh_ranges.iter() {
        match range {
            None => {
                range = Some(next_range.clone());
            },
            Some(cur_range) if *next_range.start() <= cur_range.end() + 1 => {
                range = Some(RangeInclusive::new(*cur_range.start(), *cur_range.end().max(next_range.end())));
            }
            Some(cur_range) => {
                count += cur_range.count() as i64;
                range = Some(next_range.clone());
            }
        }
    }
    if let Some(cur_range) = range {
        count += cur_range.count() as i64;
    }
    count
}



#[cfg(test)]
pub mod day5_tests {
    use super::*;
    use std::fs;

#[test]
fn example1_1() {
    // A list of fresh ingredient ID ranges (inclusive), a blank line, and a list of available ingredient IDs
    let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
// Ingredient ID 1 is spoiled because it does not fall into any range.
// Ingredient ID 5 is fresh because it falls into range 3-5.
// Ingredient ID 8 is spoiled.
// Ingredient ID 11 is fresh because it falls into range 10-14.
// Ingredient ID 17 is fresh because it falls into range 16-20 as well as range 12-18.
// Ingredient ID 32 is spoiled.
// So, in this example, 3 of the available ingredient IDs are fresh.

    let inventory: Inventory = input.parse().unwrap();
    let total = check_ingredients(&inventory);
    assert_eq!(3, total);
}


#[test]
fn part1() {
    let input = fs::read_to_string("src/day5/input1.txt").unwrap();
    let inventory: Inventory = input.parse().unwrap();
    let total = check_ingredients(&inventory);
    assert_ne!(3, total);
    assert_eq!(726, total);
}

#[test]
fn example2_1() {
    // A list of fresh ingredient ID ranges (inclusive), a blank line, and a list of available ingredient IDs
    let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
// IDs that the fresh ingredient ID ranges consider to be fresh: 3-5, 10-14, 16-20, 12-18.
// The ingredient IDs that these ranges consider to be fresh are: 3, 4, 5, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20.
// So, in this example, the fresh ingredient ID ranges consider a total of 14 ingredient IDs to be fresh.
    let inventory: Inventory = input.parse().unwrap();
    let total = fresh_ingredients(&inventory);
    assert_eq!(14, total);
}


#[test]
fn part2() {
    let input = fs::read_to_string("src/day5/input1.txt").unwrap();
    let inventory: Inventory = input.parse().unwrap();
    let total = fresh_ingredients(&inventory);
    assert_ne!(3, total);
    assert_ne!(726, total);
    assert_eq!(354226555270043, total);
}


}
