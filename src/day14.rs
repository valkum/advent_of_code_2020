use anyhow::Result;
use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use itertools::Itertools;
use std::str::FromStr;
use std::{
    collections::HashMap,
    fmt::Binary,
    fmt::{self, Debug},
    ops::BitOr,
};

const LOCAL_MASK: u64 = (1u64 << 36) - 1;
#[derive(Debug, Clone, Copy)]
pub struct Mask {
    mask: u64,
    value: u64,
}
impl FromStr for Mask {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        assert_eq!(s.len(), 36);

        return Ok(Mask {
            mask: s.bytes().enumerate().fold(0, |acc, x| match x.1 {
                b'X' => acc,
                b'0' | b'1' => acc | 1 << (35 - x.0),
                _ => unimplemented!(),
            }),
            value: s.bytes().enumerate().fold(0, |acc, x| match x.1 {
                b'X' | b'0' => acc,
                b'1' => acc | 1 << (35 - x.0),
                _ => unimplemented!(),
            }),
        });
    }
}

impl Binary for Mask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:b}\n", self.mask)?; // delegate to i32's implementation
        write!(f, "{:b}", self.value) // delegate to i32's implementation
    }
}

impl Mask {
    #[inline]
    pub fn get_adresses(&self) -> impl Iterator<Item = u64> + Clone + '_ {
        let mask = (1u64 << 36) - 1;
        (0..36).filter_map(move |i| match (!self.mask & mask & (1u64 << i)) != 0 {
            true => Some(1 << i),
            false => None,
        })
    }

    #[inline]
    pub fn get_adresses_vec(&self) -> Vec<u64> {
        let parts = (0..36)
            .filter_map(
                move |i| match (!self.mask & LOCAL_MASK & (1u64 << i)) != 0 {
                    true => Some(1 << i),
                    false => None,
                },
            )
            .collect::<Vec<u64>>();
        (0..2usize.pow(parts.len() as u32))
            .map(|i| {
                parts
                    .iter()
                    .enumerate()
                    .filter(|&(t, _)| (i >> t) % 2 == 1)
                    .map(|(_, element)| *element)
                    .sum()
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
pub enum Input {
    Mask(Mask),
    Mem(u64, u64),
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Vec<Input> {
    input
        .lines()
        .map(|line| {
            let line = line.trim();
            if line.starts_with("mask") {
                return Input::Mask(
                    line.get(7..line.len())
                        .unwrap()
                        .parse::<Mask>()
                        .expect("Could not parse Mask"),
                );
            } else if line.starts_with("mem") {
                let index_end = line.find(']').expect("No closing bracket found");
                return Input::Mem(
                    line.get(4..index_end)
                        .unwrap()
                        .parse::<u64>()
                        .expect("Could not parse memory index"),
                    line.get((index_end + 4)..line.len())
                        .unwrap()
                        .parse::<u64>()
                        .expect("Could not parse value."),
                );
            } else {
                unimplemented!()
            }
        })
        .collect()
}
#[derive(Debug, Clone, Copy)]
pub struct MemoryValue(u64);
impl From<u64> for MemoryValue {
    fn from(i: u64) -> Self {
        MemoryValue(i)
    }
}

impl BitOr<Mask> for MemoryValue {
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a | b`
    fn bitor(self, rhs: Mask) -> Self::Output {
        // println!("A: \t\t\t{:10b}", self.0);

        // println!("Mask: \t\t\t{:10b}", rhs.mask);
        // println!("Value: \t\t\t{:10b}", rhs.value);
        let ones = rhs.value ^ rhs.mask;
        let zeroes = !rhs.value ^ rhs.mask;
        // println!("Ones: \t\t\t{:10b}", ones);
        // println!("Zeroes: \t\t\t{:10b}", zeroes);

        // println!("6 | value: \t\t\t{:10b}", (self.0 ^ ones) | rhs.value);
        // println!("4: \t\t\t{:10b}", (self.0 ^ ones) & zeroes);

        Self(((self.0 ^ ones) | rhs.value) & zeroes)
    }
}
pub struct Computer {
    pub mask: Mask,
    pub memory: HashMap<usize, MemoryValue>,
}
impl Computer {
    fn new() -> Computer {
        Computer {
            mask: Mask { mask: 0, value: 0 },
            memory: HashMap::new(),
        }
    }
    fn apply(&mut self, input: &Input) {
        match input {
            Input::Mask(m) => self.mask = *m,
            Input::Mem(k, v) => {
                self.memory
                    .insert(*k as usize, MemoryValue::from(*v) | self.mask);
            }
        }
    }
    fn apply2(&mut self, input: &Input) {
        match input {
            Input::Mask(m) => self.mask = *m,
            Input::Mem(k, v) => {
                let adresses = self.mask.get_adresses();

                for count in 0..=(!self.mask.mask & LOCAL_MASK).count_ones() as usize {
                    for x in adresses.clone().combinations(count) {
                        self.memory.insert(
                            (((*k & self.mask.mask) | self.mask.value) | x.iter().sum::<u64>())
                                as usize,
                            MemoryValue::from(*v),
                        );
                    }
                }
            }
        }
    }
}

#[aoc(day14, part1)]
pub fn part1(input: &Vec<Input>) -> u64 {
    let mut computer = Computer::new();
    for x in input {
        computer.apply(x);
    }
    computer.memory.values().map(|x| x.0).sum::<u64>()
}

#[aoc(day14, part2)]
#[allow(non_snake_case)]
pub fn part2(input: &Vec<Input>) -> u64 {
    let mut computer = Computer::new();
    for x in input {
        computer.apply2(x);
    }
    computer.memory.values().map(|x| x.0).sum::<u64>()
}
#[aoc(day14, part2, alt)]
#[allow(non_snake_case)]
pub fn part2_alt(input: &Vec<Input>) -> u64 {
    let mut mask = Mask { mask: 0, value: 0 };
    let mut memory: HashMap<u32, u64> = HashMap::with_capacity(100000);
    let mut adresses: Vec<u64> = Vec::new();

    for x in input {
        match x {
            Input::Mask(m) => {
                mask = *m;
                let parts: Vec<u64> = (0..36)
                    .filter_map(
                        move |i| match (!mask.mask & LOCAL_MASK & (1u64 << i)) != 0 {
                            true => Some(1 << i),
                            false => None,
                        },
                    )
                    .collect();
                adresses = (0..2usize.pow((!mask.mask & LOCAL_MASK).count_ones() as u32))
                    .map(|i| {
                        parts
                            .iter()
                            .enumerate()
                            .filter(|&(t, _)| (i >> t) % 2 == 1)
                            .map(|(_, element)| element)
                            .sum::<u64>()
                    })
                    .collect();
            }
            Input::Mem(k, v) => {
                for x in &adresses {
                    let adress: u32 = (((*k & mask.mask) | mask.value) | x) as u32;
                    memory.insert(adress, *v);
                    // if let Some(k) = memory.get_mut(&adress) {
                    //     *k = *v;
                    // } else {
                    //     ;
                    // }
                }
            }
        }
    }
    memory.iter().map(|x| x.1).sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
    mem[8] = 11
    mem[7] = 101
    mem[8] = 0";
    const SAMPLE2: &str = "mask = 000000000000000000000000000000X1001X
    mem[42] = 100
    mask = 00000000000000000000000000000000X0XX
    mem[26] = 1";
    const SAMPLE3: &str = "3123
    67,7,59,61";

    #[test]
    fn sample1() {
        assert_eq!(part1(&input_generator(&SAMPLE)), 165);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&input_generator(&SAMPLE2)), 208);
    }

    #[test]
    fn sample2_alt() {
        assert_eq!(part2_alt(&input_generator(&SAMPLE2)), 208);
    }
}
