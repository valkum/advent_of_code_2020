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


#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .split(",")
        .map(|num| {
            num.parse::<u32>().expect("Could not parse number")
        })
        .collect()
}
const BOUNDRY: usize = 3_000_000;
pub struct SequenceGenerator{
    state: HashMap<u32, usize>,
    cache: Box<[usize]>,
    head: u32,
    index: usize
}

impl SequenceGenerator{
    fn from_seed(seed: &[u32]) -> SequenceGenerator {
        let state = HashMap::new(); // number -> occurance
        let mut cache = vec![0; BOUNDRY].into_boxed_slice();
        for (i, x) in seed.iter().enumerate() {
            cache[*x as usize] = i+1;
        }
        let head = *seed.last().unwrap();
        SequenceGenerator {
            state: state,
            cache: cache,
            head: head,
            index: seed.len()
        }
    }

    #[inline]
    fn next_u32(&mut self) -> u32 {
        let last_at: usize;
        {
            last_at = if (self.head as usize) < BOUNDRY {
                *self.cache.get(self.head as usize).unwrap()
            } else {
                *self.state.get(&self.head).unwrap_or(&0)
            };
        }
        {
            if (self.head as usize) < BOUNDRY {
                self.cache[self.head as usize] = self.index;
            }else{
                self.state.insert(self.head, self.index);
            }
            if last_at == 0 {
                self.head = 0;
            }else {
                self.head = (self.index - last_at)as u32;
            }
        }
        self.index += 1;
        return self.head;
    }
}

#[aoc(day15, part1)]
pub fn part1(input: &[u32]) -> u32 {
    let mut generator = SequenceGenerator::from_seed(input);
    for _ in input.len()..2019 {
        generator.next_u32();
    }
    generator.next_u32()
}

#[aoc(day15, part2)]
pub fn part2(input: &[u32]) -> u32 {
    let mut generator = SequenceGenerator::from_seed(input);
    for _ in input.len()..29_999_999 {
        generator.next_u32();
    }
    generator.next_u32()
}


#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: &str = "0,3,6";
    const SAMPLE2: &str = "1,3,2";
    const SAMPLE3: &str = "2,1,3";

    #[test]
    fn sample1() {
        assert_eq!(part1(&input_generator(&SAMPLE1)), 436);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&input_generator(&SAMPLE1)), 175594);
    }

    #[test]
    fn sample2_2() {
        assert_eq!(part2(&input_generator(&SAMPLE2)), 2578);
    }
    #[test]
    fn sample2_4() {
        assert_eq!(part2(&input_generator(&SAMPLE3)), 3544142);
    }
}
