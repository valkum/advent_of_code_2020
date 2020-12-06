use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use std::iter::FromIterator;
use std::str::FromStr;
use anyhow::Result;
use itertools::Itertools;

#[derive(Debug,Default)]
pub struct Seat {
    row: u8,
    column: u8
}
impl FromStr for Seat {
    type Err = anyhow::Error;
    
    fn from_str(s: &str) -> Result<Self> {
        return Ok(Seat {
            row: s[0..=6].bytes().enumerate().fold(0u8, |acc, x| {
                match x.1 {
                    b'F' => {acc},
                    b'B' => {acc | 1<<(6-x.0)},
                    _ => unimplemented!()
                }
            }),
            column: s[7..=9].bytes().enumerate().fold(0u8, |acc, x| {
                match x.1 {
                    b'L' => {acc},
                    b'R' => {acc | 1<<(2-x.0)},
                    _ => unimplemented!()
                }
            })
        })
    }
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Vec<u32>> {
    let mut entries: Vec<Vec<u32>> = Vec::new();
    for group in input.split("\n\n") {
        entries.push(group.lines().map(|s| s.trim().bytes().fold(0u32, |acc, x| acc | 1<<(x as usize - 0x61))).collect());
    }
    return entries
}

#[aoc(day6, part1)]
pub fn part1(input: &[Vec<u32>]) -> u32 {
    input.iter()
        .map(|group| group
            .iter()
            .fold(0u32, |acc, x| acc | x)
        )
        .map(|x| x.count_ones())
        .sum()
}

#[aoc(day6, part2)]
pub fn part2(input: &[Vec<u32>]) -> u32 {
    input.iter()
        .map(|group| {
            // fold_first workaround
            let mut iter = group.iter();
            let first = *iter.next().unwrap();
            iter.fold(first, |acc, x| acc & x)
        })
        .map(|x| x.count_ones())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str= "abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    fn sample1() {
        let input = input_generator(&SAMPLE);
        assert_eq!(part1(&input), 11);
    }

    #[test]
    fn sample2() {
        let input = input_generator(&SAMPLE);
        assert_eq!(part2(&input), 6);
    }
}