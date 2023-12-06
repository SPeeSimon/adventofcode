
fn main() {

}


#[derive(Debug)]
struct Race {
    time: i64,
    distance: i64,
}


fn parse_input(input: &str) -> Vec<Race> {
    let mut time : Vec<i64> = Vec::new();
    let mut distance: Vec<i64> = Vec::new();

    for line in input.lines() {
        let values = line.split_whitespace().skip(1).map(|c| c.parse::<i64>().unwrap()).collect();
        if line.starts_with("Time:") {
            time = values;
        }
        else if line.starts_with("Distance:") {
            distance = values;
        }
    }

    time.iter().zip(distance.iter())
        .map(|(t,d)| { Race { time: *t, distance: *d, }})
        .collect()
}


fn parse_input2(input: &str) -> Race {
    let mut time = 0;
    let mut distance = 0;

    for line in input.lines() {
        let values = line.split_once(":").unwrap().1.replace(char::is_whitespace, "").parse::<i64>().unwrap();
        if line.starts_with("Time:") {
            time = values;
        }
        else if line.starts_with("Distance:") {
            distance = values;
        }
    }

    Race {
        time: time,
        distance: distance,
    }
}


fn calc_record(race: &Race) -> Vec<i64> {
    let mut input = Vec::new();
    for hold_button_ms in 1..race.time-1 {
        if hold_button_ms * (race.time - hold_button_ms) > race.distance {
            input.push(hold_button_ms);
        }
    }
    return input;
}


#[test]
fn example1() {
    let input = 
"Time:      7  15   30
Distance:  9  40  200";

    let races = parse_input(input);
    println!("{:?}", &races);
    assert_eq!(4, calc_record(&races[0]).len());
    assert_eq!(vec![2,3,4,5], calc_record(&races[0]));
    assert_eq!(8, calc_record(&races[1]).len());
    assert_eq!(9, calc_record(&races[2]).len());
    assert_eq!(288, races.iter().map(|r| calc_record(r).len() as i64).product::<i64>());
}


#[test]
fn part1() {
    let input = include_str!("input.txt");
    println!("total: \x1b[32m{}\x1b[0m", parse_input(input).iter().map(|r| calc_record(r).len()).product::<usize>());
}


#[test]
fn example2() {
    let input = 
"Time:      7  15   30
Distance:  9  40  200";

    let race = parse_input2(input);
    println!("{:?}", &race);
    assert_eq!(71503, calc_record(&race).len());
}


#[test]
fn part2() {
    let input = include_str!("input.txt");
    let race = parse_input2(input);
    println!("total: \x1b[32m{}\x1b[0m", calc_record(&race).len());
}
