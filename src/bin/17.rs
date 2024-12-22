use itertools::Itertools;
use tracing::{debug, info, instrument};

advent_of_code::solution!(17);

fn parse_input(input: &str) -> Machine {
    let [a, b, c] = input
        .lines()
        .take(3)
        .map(|line| {
            line.split_once(":")
                .unwrap()
                .1
                .to_string()
                .trim()
                .parse::<u32>()
                .expect("number")
        })
        .collect_vec()[0..3]
    else {
        unreachable!("invalid")
    };
    let instructions = input
        .lines()
        .skip(4) // 3 registers + newline
        .next()
        .expect("program")
        .trim_end()
        .split_once(":")
        .expect("instructions")
        .1
        .trim()
        .split(",")
        .map(|c| match c {
            "0" => Instruction::ADV,
            "1" => Instruction::BXL,
            "2" => Instruction::BST,
            "3" => Instruction::JNZ,
            "4" => Instruction::BXC,
            "5" => Instruction::OUT,
            "6" => Instruction::BDV,
            "7" => Instruction::CDV,
            _ => unreachable!("invalid instruction code"),
        })
        .collect_vec();
    Machine::new(a, b, c, instructions.to_vec())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    /// The adv instruction (opcode 0) performs division. The numerator is the value in the A register.
    /// The denominator is found by raising 2 to the power of the instruction's combo operand.
    /// (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.)
    /// The result of the division operation is truncated to an integer and then written to the A register.
    ADV = 0,

    /// The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the instruction's literal operand,
    /// then stores the result in register B.
    BXL = 1,

    ///The bst instruction (opcode 2) calculates the value of its combo operand modulo 8 (thereby keeping only its lowest 3 bits),
    /// then writes that value to the B register.
    BST = 2,

    ///The jnz instruction (opcode 3) does nothing if the A register is 0.
    /// However, if the A register is not zero, it jumps by setting the instruction pointer to the value of its literal operand;
    /// if this instruction jumps, the instruction pointer is not increased by 2 after this instruction.
    JNZ = 3,

    /// The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C,
    /// then stores the result in register B.
    /// (For legacy reasons, this instruction reads an operand but ignores it.)
    BXC = 4,

    /// The out instruction (opcode 5) calculates the value of its combo operand modulo 8, then outputs that value.
    /// (If a program outputs multiple values, they are separated by commas.)
    OUT = 5,

    /// The bdv instruction (opcode 6) works exactly like the adv instruction except that the result is stored in the B register.
    /// (The numerator is still read from the A register.)
    BDV = 6,

    /// The cdv instruction (opcode 7) works exactly like the adv instruction except that the result is stored in the C register.
    /// (The numerator is still read from the A register.)
    CDV = 7,
}

#[derive(Debug)]
struct Machine {
    ptr: usize,
    a: u32,
    b: u32,
    c: u32,
    instr: Vec<Instruction>,
    output: Vec<String>,
}

impl Machine {
    fn new(a: u32, b: u32, c: u32, instructions: Vec<Instruction>) -> Self {
        Self {
            ptr: 0,
            a,
            b,
            c,
            instr: instructions,
            output: vec![],
        }
    }
    fn combo_val(&self, i: &Instruction) -> u32 {
        let n = *i as u32;
        match n {
            0..=3 => n,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            7 => unreachable!("never should 7 appear"),
            _ => unreachable!("invalid combo operator"),
        }
    }

    fn run(&mut self) {
        let mut executed = 0;
        while self.ptr < self.instr.len() && executed < 1_000 {
            let instruction = self.instr[self.ptr];
            let operand = self.instr[self.ptr + 1];
            let val = if instruction == Instruction::BXL {
                operand as u32
            } else {
                self.combo_val(&operand)
            };
            debug!(instr = ?instruction, raw_operand = ?operand as usize, operand_val = val, ?self.ptr, self.a, self.b, self.c, ?self.output);
            match instruction {
                Instruction::ADV => self.a /= 2_u32.pow(val),
                Instruction::BXL => self.b ^= val,
                Instruction::BST => self.b = val.rem_euclid(8),
                Instruction::JNZ => {
                    if self.a == 0 {
                        self.ptr += 2; // do nothing, but increment the instruction pointer
                    } else {
                        self.ptr = val as usize;
                    }
                }
                Instruction::BXC => self.b = self.b ^ self.c,
                Instruction::OUT => {
                    let v = val.rem_euclid(8);
                    self.output.push(v.to_string())
                }
                Instruction::BDV => self.b = self.a / 2_u32.pow(val),
                Instruction::CDV => self.c = self.a / 2_u32.pow(val),
            }
            if instruction != Instruction::JNZ {
                self.ptr += 2
            }
            executed += 1;
        }
    }
}

#[instrument(skip(input))]
pub fn part_one(input: &str) -> Option<String> {
    let mut machine = parse_input(input);
    info!(?machine);
    machine.run();
    info!(ending_state =  ?machine);
    Some(machine.output.join(","))
}

#[instrument(skip(input))]
pub fn part_two(input: &str) -> Option<String> {
    None
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use tracing_test::traced_test;

    use super::*;

    #[traced_test]
    #[rstest]
    #[case(
        r"Register A: 10
Register B: 0
Register C: 0

Program: 5,0,5,1,5,4",
        "0,1,2"
    )]
    #[case(
        r"Register A: 0
Register B: 0
Register C: 9

Program: 2,6",
        ""
    )]
    #[case(
        r"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0",
        "4,6,3,5,6,3,5,2,1,0"
    )]
    #[case(
        r"Register A: 2024
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0",
        "4,2,5,6,7,7,7,7,3,1,0"
    )]
    #[case(
        r"Register A: 0
Register B: 29
Register C: 0

Program: 1,7",
        ""
    )]
    // looks like this one will be really annoying... time to learn how to use rstest
    fn test_examples(#[case] input: &str, #[case] expected: String) {
        let result = part_one(input);
        assert_eq!(result, Some(expected));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
