use std::fs;


fn main() {
    println!("Hello, world!");
}


fn calc(input: &str) -> (i64, i64) {
    let mut result = (0,0);
    for line in input.lines() {
        let history: Vec<i64> = line.split_whitespace().map(|d| d.parse::<i64>().unwrap()).collect();
        let mut calc_stack = Vec::new();
        calc_stack.push(history);
        
        while !calc_stack.last().unwrap().iter().all(|d| *d == 0) {
            let diff = calc_stack.last().unwrap().windows(2).map(|e| e[1]-e[0]).collect();
            calc_stack.push(diff);
        }

        let extrapolated = calc_stack.iter().rfold((0, 0), |acc, e| (e.first().unwrap() - acc.0, acc.1 + e.last().unwrap()));
        result.0 += extrapolated.0;
        result.1 += extrapolated.1;

        // for c in calc_stack {
        //     print!("{:?} => ", &c);
        // }
        // println!("(before/after) {:?}", extrapolated);
    }
    result
}



#[test]
fn example1_2() {
    let input =
"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    // 0 3 6 9 12 15 => 3 3 3 3 3 => 0 0 0 0
    // B  <= A <= 0
    // next result: 18 <= 3 <= 0
    // previous result: -3 <= 3 <= 0 (part2)
    // -3 and 18
    // 1 3 6 10 15 21 => 2 3 4 5 6 => 1 1 1 1 => 0 0 0
    // next result: 28 <= 7 <= 1 <= 0
    // previous result: 0 <= 1 <= 1 <= 0 (part2)
    // 0 and 28
    // 10 13 16 21 30 45 => 3 3 5 9 15 => 0 2 4 6 => 2 2 2 => 0 0
    // next result: 68 <= 23 <= 8 <= 2 <= 0
    // previous result: 5 <= 5 <= -2 <= 2 <= 0 (part2)
    // 5 and 68
    // total before: 2
    // total after: 114
    assert_eq!((2, 114), calc(input));
}


#[test]
fn part1_2() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let result = calc(input.as_str());
    println!("extrapolate before: \x1b[32m{}\x1b[0m", result.0);
    println!("extrapolate after: \x1b[32m{}\x1b[0m", result.1);
    assert_eq!((913, 1789635132), result);
}
