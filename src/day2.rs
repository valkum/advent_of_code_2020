use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use std::iter::FromIterator;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
pub struct Entry {
    min: usize,
    max: usize,
    char: String,
    password: String,
}

impl FromStr for Entry {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(|c| c == ' ' || c == ':' || c == '-').collect();
        return Ok(Entry {
            min: parts[0].parse::<usize>()?,
            max: parts[1].parse::<usize>()?,
            char: parts[2].to_owned(),
            password: parts[4].to_owned(),
        });
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Entry> {
    Vec::from_iter(
        input
            .lines()
            .map(|s| s.trim().parse::<Entry>().ok().unwrap()),
    )
}

#[aoc(day2, part1)]
pub fn part1(input: &[Entry]) -> usize {
    input
        .iter()
        .filter(|x| {
            let len = x
                .password
                .bytes()
                .filter(|c| *c == x.char.as_bytes()[0])
                .count();
            len >= x.min && len <= x.max
        })
        .count()
}

#[aoc(day2, part2)]
pub fn part2(input: &[Entry]) -> usize {
    input
        .iter()
        .filter(|x| {
            (x.password.as_bytes()[x.min - 1] == x.char.as_bytes()[0])
                ^ (x.password.as_bytes()[x.max - 1] == x.char.as_bytes()[0])
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const sample: &str = "1-3 a: abcde
    1-3 b: cdefg
    2-9 c: ccccccccc";

    #[test]
    fn sample1() {
        assert_eq!(part1(&input_generator(&sample)), 2);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&input_generator(&sample)), 1);
    }
}
