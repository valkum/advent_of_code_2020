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

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<String> {
    Vec::from_iter(
        input.lines()
        .map(|s| s.trim().to_owned())
    )
}

#[aoc(day5, part1)]
pub fn part1(input: &[String]) -> usize {
    input.iter().map(|s| Seat::from_str(s).unwrap()).map(|s| s.row as usize * 8 + s.column as usize).max().unwrap()
}

#[aoc(day5, part2)]
pub fn part2(input: &[String]) -> usize {
    input.iter()
        .map(|s| Seat::from_str(s).unwrap())
        .map(|s| s.row as usize * 8 + s.column as usize).sorted()
        .tuple_windows::<(_, _)>()
        .filter_map(|(prev, next)| {
            if prev+2==next {
                Some(prev+1)
            } else {
                None
            }
        }).next().unwrap()


}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str= "FBFBBFFRLR
    BFFFBBFRRR
    FFFBBBFRRR
    BBFFBBFRLL";

    #[test]
    fn sample1() {
        let input = input_generator(&SAMPLE);
        assert_eq!(input.iter().map(|s| Seat::from_str(s).unwrap()).map(|s| s.row as usize * 8 + s.column as usize).collect::<Vec<usize>>(), vec![357, 567, 119, 820]);
        assert_eq!(part1(&input), 820);
    }
}