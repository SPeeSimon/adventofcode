use std::fs;


struct Report {
    level: Vec<i32>,
}


enum Outcome {
    Safe_InCrease(i32),
    Safe_DeCrease(i32),
    UnSafe(i32),
}


impl Outcome {

    fn check(prev: i32, cur: i32) -> Outcome {
        let diff = cur - prev;
        if diff.abs() >= 1 && diff.abs() <= 3 {
            if diff < 0 {
                return Outcome::Safe_DeCrease(diff);
            }
            return Outcome::Safe_InCrease(diff);
        }
        Outcome::UnSafe(diff)
    }

}


impl Report {

    fn all_lower(list: &Vec<i32>) -> bool {
        for i in 1..list.len() {
            if list[i-1] >= list[i] {
                return false;
            }
        }
        true
    }

    fn all_greater(list: &Vec<i32>) -> bool {
        for i in 1..list.len() {
            if list[i-1] <= list[i] {
                return false;
            }
        }
        true
    }

    fn check(levels: &Vec<i32>) -> bool {
        let is_safe_diff = |one: i32, two: i32| { two.abs_diff(one) >= 1 && two.abs_diff(one) <= 3};
        let allIncr = Report::all_lower(levels);
        let allDecr = Report::all_greater(levels);

        if allIncr || allDecr {
            // check diff
            for i in 1..levels.len() {
                if !is_safe_diff(levels[i-1], levels[i]) {
                    return false;
                }
            }
            return true;
        }
        false
    }

    fn is_safe(&self) -> bool {
        Self::check(&self.level)
        /*
        let is_safe_diff = |one: i32, two: i32| { two.abs_diff(one) >= 1 && two.abs_diff(one) <= 3};
        let allIncr = Report::all_lower(&self.level);
        let allDecr = Report::all_greater(&self.level);

        if allIncr || allDecr {
            // check diff
            for i in 1..self.level.len() {
                if !is_safe_diff(self.level[i-1], self.level[i]) {
                    return false;
                }
            }
            return true;
        }
        false
        */
    }

    fn is_safe_with_dampener(&self) -> bool {
        if self.is_safe() {
            return true;
        }

        for i in 0..self.level.len() {
            let mut dampened_levels = self.level.clone();
            dampened_levels.remove(i);

            if Self::check(&dampened_levels) {
                return true;
            }
        }

        false
    }



}


#[cfg(test)]
pub mod day1_tests {
    use super::*;

#[test]
fn example1() {
    let input =
"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    
    let (safe_levels, unsafe_levels): (Vec<Report>, Vec<Report>) =
            input.lines()
                .map(|report| report.split_whitespace())
                .map(|levels| 
                    Report{
                    level: levels.into_iter()
                                 .map(|lvl| lvl.parse().unwrap())
                                 .collect()
                })
                .partition(|level| level.is_safe());

    println!("total safe: {}", safe_levels.len());
    assert_eq!(2, safe_levels.len());
    assert_eq!(4, unsafe_levels.len());
}


#[test]
fn part1() {
    let input = fs::read_to_string("src/day2/input.txt").unwrap();
    let (safe_levels, unsafe_levels): (Vec<Report>, Vec<Report>) =
            input.lines()
                .map(|report| report.split_whitespace())
                .map(|levels| 
                    Report{
                    level: levels.into_iter()
                                 .map(|lvl| lvl.parse().unwrap())
                                 .collect()
                })
                .partition(|level| level.is_safe());

    println!("total safe: {}", safe_levels.len());
    assert_eq!(510, safe_levels.len());
    assert_eq!(490, unsafe_levels.len());
}


#[test]
fn example2() {
    let input =
"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    let (safe_levels, unsafe_levels): (Vec<Report>, Vec<Report>) =
            input.lines()
                .map(|report| report.split_whitespace())
                .map(|levels| 
                    Report{
                    level: levels.into_iter()
                                .map(|lvl| lvl.parse().unwrap())
                                .collect()
                })
                .partition(|level| level.is_safe_with_dampener());

    println!("total safe: {}", safe_levels.len());
    assert_eq!(4, safe_levels.len());
    assert_eq!(2, unsafe_levels.len());
}



#[test]
fn part2() {
    let input = fs::read_to_string("src/day2/input.txt").unwrap();
    let (safe_levels, unsafe_levels): (Vec<Report>, Vec<Report>) =
            input.lines()
                .map(|report| report.split_whitespace())
                .map(|levels| 
                    Report{
                    level: levels.into_iter()
                                .map(|lvl| lvl.parse().unwrap())
                                .collect()
                })
                .partition(|level| level.is_safe_with_dampener());

    println!("total safe: {}", safe_levels.len());
    assert_eq!(553, safe_levels.len());
    assert_eq!(447, unsafe_levels.len());
}

}