use std::ops::Mul;
use std::str::FromStr;
use crate::utils::{position::Position};


#[derive(Debug)]
struct Machine {
    id: usize,
    price_location: Position,
    move_a: Position,
    move_b: Position,
}

struct Arcade {
    machines: Vec<Machine>,
    max_times: usize,
}

impl FromStr for Machine {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut btn_a = None;
        let mut bnt_b = None;
        let mut price = None;
        let find_xy = |val: &str, split: &str| val.split(", ")
                                                                          .map(|v| v.split_once(split).unwrap())
                                                                          .map(|(dir, diff)| (dir, diff.parse().unwrap()))
                                                                          .fold(Position::default(), |p, (xy, len)| {
                                                                                match xy {
                                                                                    "X" => Position { x: len, y: p.y },
                                                                                    "Y" => Position { x: p.x, y: len },
                                                                                    _ => p,
                                                                                }
                                                                          });

        input.lines().take_while(|line| !line.is_empty())
                     .map(|line| line.split_once(": ").unwrap())
                     .for_each(|(cmd, val)| {
                        match cmd {
                            "Button A" => {
                                btn_a = Some(find_xy(val, "+"));
                            }
                            "Button B" => {
                                bnt_b = Some(find_xy(val, "+"));
                            }
                            "Prize" => {
                                price = Some(find_xy(val, "="));
                            }
                            _ => {}
                        }
                     });

        if price.is_none() {
            return Err("unknown price");
        }
        if btn_a.is_none() {
            return Err("unknown button A");
        }
        if bnt_b.is_none() {
            return Err("unknown button B");
        }

        Ok(Machine {
            id: 0,
            price_location: price.unwrap(),
            move_a: btn_a.unwrap(),
            move_b: bnt_b.unwrap(),
        })
    }

}


fn create_machines(input: &str) -> Vec<Machine> {
    let machines: Vec<Machine> = input.split("\n\n")
                                      .enumerate()
                                      .map(|(i, line)| Machine::from_str(line).and_then(|mut m| {m.id = i; Ok(m)}))
                                      .map(|r| r.expect("Machine failed"))
                                      .collect();
    machines
}

fn create_machines2(input: &str) -> Vec<Machine> {
    let mut machines = create_machines(input);
    for m in machines.iter_mut() {
        m.price_location = Position{x: m.price_location.x + 10000000000000, y: m.price_location.y + 10000000000000};
    }
    machines
}


impl Arcade {

    fn calculate_total_tokens(self) -> usize {
        let mut total = 0;
        for machine in self.machines {
            let max_range_a = machine.max_range_a(self.max_times);
            let max_range_b = machine.max_range_b(self.max_times);
            
            if (max_range_a * machine.move_a.x + max_range_b * machine.move_b.x) < machine.price_location.x ||
               (max_range_a * machine.move_a.y + max_range_b * machine.move_b.y) < machine.price_location.y
            {
                // println!("NOT POSSIBLE: {:?} BETWEEN A:{:?}, B:{:?}", machine, max_range_a, max_range_b);
                continue;
            } else {
                if let Some((add_a, add_b)) = machine.find_move(max_range_a, max_range_b) {
                    // println!("    POSSIBLE: {:?} == A:{:?}, B:{:?}", machine, add_a, add_b);
                    total += 3 * add_a + 1 * add_b;
                } else {
                    // println!("NOT POSSIBLE: {:?} (BETWEEN A:{:?}, B:{:?})", machine, max_range_a, max_range_b);
                }
            }
        }
        total
    }
    
}


impl Machine {
    
    fn max_range_a(&self, max: usize) -> usize {
        max.min( self.price_location.x / self.move_a.x ).min( self.price_location.y / self.move_a.y )
    }

    fn max_range_b(&self, max: usize) -> usize {
        max.min( self.price_location.x / self.move_b.x ).min( self.price_location.y / self.move_b.y )
    }

    fn calc_location(&self, steps_a: usize, steps_b: usize) -> Position {
        let new_x = self.move_a.x.mul(steps_a) + self.move_b.x.mul(steps_b);
        let new_y = self.move_a.y.mul(steps_a) + self.move_b.y.mul(steps_b);
        Position{x: new_x, y: new_y}
    }


    fn find_move(&self, max_a: usize, max_b: usize) -> Option<(usize, usize)> {
        for btn_a_cnt in 0..=max_a {
            for btn_b_cnt in 0..=max_b {
                let calc_pos = self.calc_location(btn_a_cnt, btn_b_cnt);
                match self.price_location.cmp(&calc_pos) {
                    std::cmp::Ordering::Less => break,
                    std::cmp::Ordering::Equal => return Some((btn_a_cnt, btn_b_cnt)),
                    std::cmp::Ordering::Greater => continue,
                }
            }
        }
        None
    }
}



#[cfg(test)]
pub mod day13_tests {
    use super::*;
    use std::fs;


#[test]
fn example1() {
    // list of machine button behaviour and prize location
    let input = 
"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    // button | tokens
    // A | 3
    // B | 1

    // move claw righ (X), forward (Y)
    // 1 prize, exacly above price x,y
    // smallest nr of tokens to win as many prices as possible

    // machine 1: A*80 + B*40  (80*94 + 40*22 = 8400) + (80*34 + 40*67 = 5400) = 280 tokens
    // machine 2: no prize
    // machine 3: A*38 + B*86 = 200 tokens
    // machine 4: no prize
    // no more than 100 times
        // (n * a.x) + (m * b.x) = value.x && (n * a.y) + (m * b.y) = value.y
        // (80 * 94) + (40 * 22) = 8400    && (80 * 34) + (40 * 67) = 5400
        //      7520 + 880       = 8400    &&      2720 + 2680      = 5400
        // n = 0-89 && n = 0-158
        // m = 0-381 && m = 0-80

    let arcade = Arcade{
        max_times: 100,
        machines: create_machines(&input),
    };
    let total = arcade.calculate_total_tokens();
    assert_eq!(280 + 200, total);
    assert_eq!(480, total);
}


#[test]
fn part1() {
    let input = fs::read_to_string("src/day13/input.txt").unwrap();
    let arcade = Arcade{
        max_times: 100,
        machines: create_machines(&input),
    };
    let total = arcade.calculate_total_tokens();
    assert_ne!(21456, total); // too low
    assert_eq!(25751, total);
}


// #[test]
// fn example2() {
//     let input = 
// "Button A: X+94, Y+34
// Button B: X+22, Y+67
// Prize: X=8400, Y=5400

// Button A: X+26, Y+66
// Button B: X+67, Y+21
// Prize: X=12748, Y=12176

// Button A: X+17, Y+86
// Button B: X+84, Y+37
// Prize: X=7870, Y=6450

// Button A: X+69, Y+23
// Button B: X+27, Y+71
// Prize: X=18641, Y=10279";

//     let arcade = Arcade{
//         max_times: 1000000000000000,
//         machines: create_machines2(&input),
//     };
//     let total = arcade.calculate_total_tokens();
//     assert_eq!(4800000000000000, total);
// }

// #[test]
// // fn part2() {
//     let input = fs::read_to_string("src/day13/input.txt").unwrap();
//     let arcade = Arcade{
//         max_times: 1000000000000000,
//         machines: create_machines(&input),
//     };
//     let total = arcade.calculate_total_tokens();
//     assert_ne!(21456, total); // too low
//     assert_eq!(25751, total); ?
// }

}
