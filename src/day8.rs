use anyhow::anyhow;
use anyhow::Result;
use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use nom::character::complete::char as nomchar;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    character::complete::{digit1, space1},
    combinator::{map, map_res},
    number::complete::{be_u8, i8},
    sequence::{separated_pair, tuple},
    IResult,
};
use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Instruction {
    Nop(i16),
    Acc(i16),
    Jmp(i16),
}

impl Instruction {
    fn is_nop(&self) -> bool {
        match *self {
            Instruction::Nop(_) => true,
            _ => false,
        }
    }

    fn is_jmp(&self) -> bool {
        match *self {
            Instruction::Jmp(_) => true,
            _ => false,
        }
    }
}
// fn parse_number(input: &str) -> IResult<&str, i8> {

// }

// fn parse_nop(input: &str) -> IResult<&str, Instruction>  {
//     let (input, (red, green, blue)) = tuple((tag('NOP'), parse_number))(input)?;

// fn parse_jmp(input: &str) -> IResult<&str, Instruction>  {
//     tuple
// }
// fn parse_acc(input: &str) -> IResult<&str, Instruction>  {
//     tuple
// }
fn parse_num(input: &str) -> IResult<&str, i16> {
    let (input, (sign, number)): (&str, (&str, i16)) = alt((
        tuple((tag("+"), map_res(digit1, FromStr::from_str))),
        tuple((tag("-"), map_res(digit1, FromStr::from_str))),
    ))(input)?;
    match sign {
        "+" => Ok((input, number)),
        "-" => Ok((input, -number)),
        _ => unimplemented!(),
    }
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, (ins, int)) = alt((
        separated_pair(tag("nop"), space1, parse_num),
        separated_pair(tag("acc"), space1, parse_num),
        separated_pair(tag("jmp"), space1, parse_num),
    ))(input)?;
    Ok(match ins {
        "nop" => (input, Instruction::Nop(int)),
        "acc" => (input, Instruction::Acc(int)),
        "jmp" => (input, Instruction::Jmp(int)),
        _ => unimplemented!(),
    })
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|x| parse_instruction(x))
        .map(|x| x.unwrap().1)
        .collect()
}

#[derive(Default)]
struct Computer {
    instructions: VecDeque<Instruction>,
    pc: usize,
    acc: i64,
    already_run: HashSet<usize>,
}
impl Computer {
    fn new(instructions: &Vec<Instruction>) -> Self {
        Self {
            instructions: VecDeque::from(instructions.clone()),
            ..Default::default()
        }
    }
    fn run1(&mut self) {
        while !self.already_run.contains(&self.pc) {
            self.already_run.insert(self.pc);
            // dbg!(&self.pc, &self.acc, &self.already_run);
            self.step();
        }
    }
    fn run2(&mut self) -> Result<i64> {
        while !self.already_run.contains(&self.pc) {
            self.already_run.insert(self.pc);
            // dbg!(&self.pc, &self.acc, &self.already_run);
            self.step();
            if self.pc == self.instructions.len() {
                return Ok(self.acc);
            }
        }
        Err(anyhow!("Did not termination before loop"))
    }
    fn step(&mut self) {
        match self.instructions[self.pc] {
            Instruction::Nop(offset) => self.pc = self.pc + 1,
            Instruction::Acc(amount) => {
                self.acc += amount as i64;
                self.pc = self.pc + 1
            }
            Instruction::Jmp(offset) => self.pc = (self.pc as i64 + offset as i64) as usize,
        }
    }
}

#[aoc(day8, part1)]
pub fn part1(input: &Vec<Instruction>) -> i64 {
    let mut computer = Computer::new(input);
    computer.run1();
    return computer.acc;
}

#[aoc(day8, part2)]
pub fn part2(input: &Vec<Instruction>) -> i64 {
    let mut search_space: Vec<Vec<Instruction>> = Vec::new();
    for (i, _) in input
        .iter()
        .enumerate()
        .filter(|(_, &x)| x.is_nop() || x.is_jmp())
    {
        let mut cpy = input.clone();
        cpy[i] = match input[i] {
            Instruction::Jmp(target) => Instruction::Nop(target),
            Instruction::Nop(target) => Instruction::Jmp(target),
            _ => input[i],
        };
        search_space.push(cpy);
    }
    // dbg!(&search_space);
    for program in search_space {
        let mut computer = Computer::new(&program);
        match computer.run2() {
            Ok(acc) => return acc,
            Err(_) => {}
        }
    }
    panic!("did not find any");
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    const SAMPLE2: &str = "";
    #[test]
    fn sample1() {
        let input = input_generator(&SAMPLE);
        assert_eq!(part1(&input), 5);
    }

    #[test]
    fn sample2() {
        let input = input_generator(&SAMPLE);
        assert_eq!(part2(&input), 8);
    }
}
