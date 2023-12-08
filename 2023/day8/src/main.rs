use std::{collections::BTreeMap, fs, str::Lines};

fn main() {
    println!("Hello, world!");
}


fn get_direction<'a>(direction: char, nodes: &'a Option<&(String,String)>) -> &'a str {
    if direction == 'L' {
        return nodes.unwrap().0.as_str();
    }
    return nodes.unwrap().1.as_str();
}


fn create_network_with_directions(input: &str) -> (&str, BTreeMap<String, (String, String)>) {
    let mut lines = input.lines();
    let mut directions = lines.next().unwrap();
    lines.next(); // skip blank line
    (directions, create_network(lines))
}


fn create_network(lines: Lines) -> BTreeMap<String, (String, String)> {
    let mut network: BTreeMap<String, (String, String)> = BTreeMap::new();
    for node in lines {
        let n = node.split_once(" = ").unwrap();
        let node_id = n.0;
        let connecting_nodes = n.1.replace("(", "").replace(")", "").clone();
        let (node_l, node_r) = connecting_nodes.split_once(", ").unwrap();
        network.insert(node_id.to_string(), (node_l.to_string(), node_r.to_string()));
    }
    return network;
}


fn indexed_network(network: BTreeMap<String, (String, String)>) -> (Vec<String>, Vec<(usize,usize)>) {
    let mut node_ids: Vec<String> = Vec::from_iter(network.keys().map(|x| x.clone()));
    let mut indexed_network = Vec::new();
    
    for id in node_ids.iter() {
        let (l, r) = network.get(id).unwrap();
        indexed_network.push((
            node_ids.iter().position(|x| x == l).unwrap(),
            node_ids.iter().position(|x| x == r).unwrap()
        ));
    }

    (node_ids, indexed_network)
}


fn extract_indexes(ids2: &Vec<String>, end_value: &str) -> Vec<usize> {
    ids2.iter()
        .enumerate()
        .filter(|x| x.1.ends_with(end_value))
        .map(|x| x.0)
        .collect()
}


fn calculate_steps(input: &str) -> i32 {
    let (dir, network) = create_network_with_directions(input);
    let mut directions = dir.chars().into_iter().cycle();

    let mut steps = 0;
    let mut current_node = "AAA".to_string();
    loop {
        let next_node = network.get(current_node.as_str());
        let dir = directions.next().unwrap();
        let next = get_direction(dir, &next_node);
        if current_node == next || current_node == "ZZZ" {
            break;
        }
        current_node = next.to_string();
        steps += 1
    }
    steps
}


fn calculate_steps2(input: &str) -> i128 {
    let mut lines = input.lines();
    let mut directions = lines.next().unwrap();
    lines.next(); // skip blank line
    let network = create_network(lines);
    let (ids2, network2) = indexed_network(network);
    let end_nodes = extract_indexes(&ids2, "Z");
    let mut endings = Vec::new();

    for n in extract_indexes(&ids2, "A") {
        let mut steps = 0;
        let mut next = n;
        let mut directions_iter = directions.chars().into_iter().cycle();
        print!("Start from {} ", n);
        loop {
            let dir = directions_iter.next().unwrap();
            if end_nodes.contains(&next) {
                break;
            }
            if dir == 'L' {
                next = network2.get(next).unwrap().0;
            } else {
                next = network2.get(next).unwrap().1;
            }
            steps += 1;
        }
        println!(" => {}", steps);
        endings.push(steps);
    }

    endings.iter().fold(1, |c, n| num::integer::lcm(c, *n))
}


#[test]
fn example1() {
    let input =
"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

// AAA | 0 (1,2)
// BBB | 1 (3,4)
// CCC | 2 (6,5)
// DDD | 3 (3,3)
// EEE | 4 (4,4)
// GGG | 5 (5,5)
// ZZZ | 6 (6,6)
    assert_eq!(2, calculate_steps(input));
}


#[test]
fn example2() {
    let input =
"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
    assert_eq!(6, calculate_steps(input));
}


#[test]
fn part1() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let result = calculate_steps(input.as_str());
    println!("total steps: \x1b[32m{}\x1b[0m", result);
    assert_ne!(13300, result);
    assert_eq!(13301, result);
}


#[test]
fn example3() {
    let input =
"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
    assert_eq!(6, calculate_steps2(input));
}


#[test]
fn part2() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let result = calculate_steps2(input.as_str());
    println!("total steps: \x1b[32m{}\x1b[0m", result);
    assert_ne!(12169, result); // groter
    assert_ne!(13268366086992805522755101, result); // kleiner
    assert_eq!(7309459565207, result);
}
