use std::{num::ParseIntError, str::FromStr, fs};


#[derive(Debug)]
struct Equation {
    test_value: i64,
    operators: Vec<i64>,
}


enum Operation {
    ADD,
    MULTIPLY,
    CONCATENATE
}


impl Equation {
    fn within_boundary(&self) -> bool {
        let min_total: i64 = self.operators.iter().sum(); // sum = equal or greater than test_value
        let max_total: i64 = self.operators.iter().product(); // product = equal or smaller then test_value
        !(self.test_value < min_total || self.test_value > max_total)
    }
}


impl Operation {
    fn apply(&self, a: &i64, b: &i64) -> i64 {
        match self {
            Operation::ADD => a + b,
            Operation::MULTIPLY => a * b,
            Operation::CONCATENATE => {
                let b_digits = (*b as f64).log10().floor() as u32 + 1;
                (a * 10i64.pow(b_digits)) + b
            },
        }
    }
}


impl FromStr for Equation {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pair = s.split_once(": ").unwrap();
        Ok(Equation {
            test_value: pair.0.parse().unwrap(),
            operators: pair.1.split_whitespace().map(|d| d.parse().unwrap()).collect()
        })
    }
}


fn create_equations(input: &str) -> Vec<Equation> {
    input.lines()
        .map(|line| Equation::from_str(line).unwrap())
        .collect()
}


fn calc(cur: &i64, prev: Vec<i64>) -> Vec<i64> {
    prev.iter().flat_map(|s| [Operation::ADD.apply(s, cur), Operation::MULTIPLY.apply(s, cur)]).collect()
}


fn calc3(cur: &i64, prev: Vec<i64>) -> Vec<i64> {
    prev.iter().flat_map(|s| [Operation::ADD, Operation::MULTIPLY, Operation::CONCATENATE].map(|o| o.apply(s, cur))).collect()
}


fn find_matches(equations: &Vec<Equation>) -> Vec<&Equation> { 
    let mut result = Vec::new();
    for eq in equations.iter() {
        if is_match(eq) {
            result.push(eq);
        }
    }
    result
}


fn is_match(eq: &Equation) -> bool {
    let mut loop_operators = eq.operators.iter();
    let mut sum = vec![*loop_operators.next().unwrap()]; // first one requires no calculation
    for x in loop_operators {
        let mut new_sum = calc(x, sum);
        new_sum.sort();
        new_sum.dedup();
        new_sum.retain(|&x| x <= eq.test_value);
        sum = new_sum;
    }
    sum.contains(&eq.test_value)
}


fn is_match3(eq: &Equation) -> bool {
    let mut loop_operators = eq.operators.iter();
    let mut sum = vec![*loop_operators.next().unwrap()]; // first one requires no calculation
    for x in loop_operators {
        let mut new_sum = calc3(x, sum);
        new_sum.sort();
        new_sum.dedup();
        new_sum.retain(|&x| x <= eq.test_value);
        sum = new_sum;
    }
    sum.contains(&eq.test_value)
}



#[cfg(test)]
pub mod day7_tests {
    use super::*;

#[test]
fn example1() {
    let input = 
"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    let equations = create_equations(input);
    let total: i64 = find_matches(&equations).iter()
                                            .map(|eq| eq.test_value)
                                            .sum();
    assert_eq!(3749, total);
}



#[test]
fn part1() {
    let input = fs::read_to_string("src/day7/input.txt").unwrap();
    let total: i64 = 
    create_equations(&input).iter()
                            .filter(|eq| is_match(eq))
                            .map(|eq| eq.test_value)
                            .sum();
    assert_ne!(303766878186, total); // too low
    assert_eq!(303766880536, total);
}


#[test]
fn example2() {
    let input = 
"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    let total: i64 = 
    create_equations(&input).iter()
                            .filter(|eq| is_match3(eq))
                            .map(|eq| eq.test_value)
                            .sum();
    assert_eq!(11387, total);
}


#[test]
fn part2() {
    let input = fs::read_to_string("src/day7/input.txt").unwrap();
    let total: i64 = 
    create_equations(&input).iter()
                            .filter(|eq| is_match3(eq))
                            .map(|eq| eq.test_value)
                            .sum();
    assert_eq!(337041851384440, total);
}

}
