use std::ops::{BitXor, Rem};
use std::str::FromStr;
use std::{i32, usize};


const adv: u8 = 0;
const bxl: u8 = 1;
const bst: u8 = 2;
const jnz: u8 = 3;
const bxc: u8 = 4;
const out: u8 = 5;
const bdv: u8 = 6;
const cdv: u8 = 7;


struct Computer {
    program: Program,
    commands: Vec<u8>    
}


#[derive(Debug, Clone)]
struct Program {
    register_A: i64,
    register_B: i64,
    register_C: i64,
    output: Vec<i32>,
}


enum OperandType {
    LITERAL, COMBO,
}


enum InstructionResult {
    JUMP(usize),
    OUTPUT,
    REGISTER_UPD,
}


impl FromStr for Computer {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (registers, program_str) = input.split_once("\n\n").unwrap();
        Ok(Computer {
            program: Program::from_str(registers.trim())?,
            commands: program_str.trim().split_once(": ").unwrap().1.split(",").map(|d| d.parse().unwrap()).collect(),
        })
    }
}


fn detect_program_to_result(computer: &mut Computer, digit_to_check: i64, nr_outputs_to_match: usize, output_to_cmp: &Vec<i32>) -> Option<i64> {
    for next_part in 0..8 {
        let new_value = (digit_to_check * 8) + next_part;
        let mut program2 = Program {
            register_A: new_value,
            register_B: computer.program.register_B,
            register_C: computer.program.register_C,
            output: Vec::new(),
        };

        // do stuff
        program2.run_commands(&computer.commands);

        if program2.output.len() == nr_outputs_to_match && program2.has_output_match(output_to_cmp) {
            if program2.output.len() == output_to_cmp.len() {
                return Some(new_value);
            } else {
                // match next digit
                match detect_program_to_result(computer, new_value, nr_outputs_to_match + 1, output_to_cmp) {
                    Some(val) => {return Some(val);},
                    None => {},
                }
            }
        }
    }
    None
}


impl Computer {

    fn detect_program(&mut self) -> i64 {
        let output_to_cmp: Vec<i32> = self.commands.iter().map(|&d| i32::from(d)).collect();
        detect_program_to_result(self, 0, 1, &output_to_cmp).unwrap_or_default()
    }


    fn run_program(&mut self) -> Vec<i32> {
        self.program.run_commands(&self.commands);
        self.program.output.clone()
    }
}


impl FromStr for Program {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (reg_a, reg_b, reg_c) = 
                   input.lines()
                        .map(|line| line.split_once(": ").unwrap())
                        .fold((0, 0, 0), |res, (register, value)| {
                            match register {
                                "Register A" => (value.parse().unwrap(), res.1, res.2),
                                "Register B" => (res.0, value.parse().unwrap(), res.2),
                                "Register C" => (res.0, res.1, value.parse().unwrap()),
                                _ => res,
                            }
                        });
        Ok(Program {
            register_A: reg_a,
            register_B: reg_b,
            register_C: reg_c,
            output: Vec::new(),
        })
    }
}


impl Program {


    fn has_output_match(&self, expected: &Vec<i32>) -> bool {
        self.output.iter().rev()
                    .zip(expected.iter().rev())
                    .take(self.output.len())
                    .all(|(a,b)| a == b)
    }


    fn run_commands(&mut self, commands: &Vec<u8>) {
        let mut instruction_pointer = 0;
        while instruction_pointer < commands.len() {
            let opcode = commands[instruction_pointer];
            let operand = commands[instruction_pointer + 1];
    
            // do stuff
            match self.do_instruction(opcode, operand) {
                InstructionResult::JUMP(jmp) => instruction_pointer = jmp,
                _ => instruction_pointer += 2,
            };
        }
    }


    fn get_operand_value(&self, op_type: OperandType, value: u8) -> i64 {
        match op_type {
            // he value of a literal operand is the operand itself. For example, the value of the literal operand 7 is the number 7. 
            OperandType::LITERAL => value.into(),
            // The value of a combo operand can be found as follows:
            OperandType::COMBO => match value {
                0..=3 => value.into(), // represent literal values 0 through 3.
                4 => self.register_A, // represents the value of register A.
                5 => self.register_B, // represents the value of register B.
                6 => self.register_C, // represents the value of register C.
                7 => todo!("is reserved and will not appear in valid programs."),
                _ => unimplemented!(),
            },
        }
    }


    fn do_instruction(&mut self, instr: u8, operand: u8) -> InstructionResult {
        match instr {
            adv => {
            // The adv instruction (opcode 0) performs division. 
                // The numerator is the value in the A register. 
                // The denominator is found by raising 2 to the power of the instruction's combo operand. (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.) 
                // The result of the division operation is truncated to an integer and then written to the A register.
                let num = self.register_A;
                let dom = 2_i64.pow(self.get_operand_value(OperandType::COMBO, operand).try_into().unwrap());
                self.register_A = num.wrapping_div(dom);
            },
            bxl => {
                // The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the instruction's literal operand, then stores the result in register B.
                self.register_B = self.register_B.bitxor(self.get_operand_value(OperandType::LITERAL, operand));
            },
            bst => {
                // The bst instruction (opcode 2) calculates the value of its combo operand modulo 8 (thereby keeping only its lowest 3 bits), then writes that value to the B register.
                self.register_B = self.get_operand_value(OperandType::COMBO, operand).rem(8);
            },
            jnz => {
                // The jnz instruction (opcode 3) does nothing if the A register is 0. 
                // However, if the A register is not zero, it jumps by setting the instruction pointer to the value of its literal operand; if this instruction jumps, the instruction pointer is not increased by 2 after this instruction.
                if self.register_A != 0 {
                    return InstructionResult::JUMP(self.get_operand_value(OperandType::LITERAL, operand).try_into().unwrap());
                }
            },
            bxc => {
                // The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C, then stores the result in register B. (For legacy reasons, this instruction reads an operand but ignores it.)
                self.register_B= self.register_B.bitxor(self.register_C);
            },
            out => {
                // The out instruction (opcode 5) calculates the value of its combo operand modulo 8, then outputs that value. (If a program outputs multiple values, they are separated by commas.)
                let output_value = self.get_operand_value(OperandType::COMBO, operand).rem(8);
                self.output.push(output_value.try_into().unwrap());
                return InstructionResult::OUTPUT;
            },
            bdv => {
                // The bdv instruction (opcode 6) works exactly like the adv instruction except that the result is stored in the B register. (The numerator is still read from the A register.)
                let num = self.register_A;
                let dom = 2_i64.pow(self.get_operand_value(OperandType::COMBO, operand).try_into().unwrap());
                self.register_B = num.wrapping_div(dom);
            },
            cdv => {
                // The cdv instruction (opcode 7) works exactly like the adv instruction except that the result is stored in the C register. (The numerator is still read from the A register.)
                let num = self.register_A;
                let dom = 2_i64.pow(self.get_operand_value(OperandType::COMBO, operand).try_into().unwrap());
                self.register_C = num.wrapping_div(dom);
            },
            _ => {},
        }
        InstructionResult::REGISTER_UPD
    }

}


#[cfg(test)]
pub mod day17_tests {
    use itertools::Itertools;
    use super::*;


#[test]
fn example1() {
    let input = 
"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    let mut computer = Computer::from_str(input).unwrap();
    let total = computer.run_program();
    assert_eq!(vec![4,6,3,5,6,3,5,2,1,0], total);
}


#[test]
fn example1_1() {
    let input = 
"Register A: 10
Register B: 0
Register C: 0

Program: 5,0,5,1,5,4";

    let mut computer = Computer::from_str(input).unwrap();
    let total = computer.run_program();
    assert_eq!(vec![0,1,2], total);
}


#[test]
fn example1_2() {
    let input = 
"Register A: 2024
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    let mut computer = Computer::from_str(input).unwrap();
    let total = computer.run_program();
    assert_eq!(vec![4,2,5,6,7,7,7,7,3,1,0], total);
}


#[test]
fn part1() {
    let input = std::fs::read_to_string("src/day17/input.txt").unwrap();
    let mut computer = Computer::from_str(&input).unwrap();
    let total = computer.run_program();
    assert_eq!(vec![2, 0, 4, 2, 7, 0, 1, 0, 3], total);
    assert_eq!("2,0,4,2,7,0,1,0,3", total.iter().map(|d| format!("{}", d)).join(","));
}


#[test]
fn example2() {
    let input = 
"Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    let mut computer = Computer::from_str(input).unwrap();
    let total = computer.detect_program();
    assert_eq!(117440, total);
}


#[test]
fn part2() {
    let input = std::fs::read_to_string("src/day17/input.txt").unwrap();
    let mut computer = Computer::from_str(&input).unwrap();
    let total = computer.detect_program();
    assert_ne!(2147482647, total);
    assert_eq!(265601188299675, total);
}


}
