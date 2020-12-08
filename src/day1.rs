use aoc_runner_derive::aoc;
use std::collections::HashSet;
use std::iter::FromIterator;

#[aoc(day1, part1, Chars)]
pub fn part1(input: &str) -> u32 {
    let entries = HashSet::<u32>::from_iter(
        input
            .lines()
            .into_iter()
            .filter_map(|s| s.trim().parse::<u32>().ok()),
    );
    entries
        .iter()
        .find_map(|x| entries.get(&(2020 - x)).map(|y| x * y))
        .unwrap()
        .clone()
}
#[aoc(day1, part2, Chars)]
pub fn part2(input: &str) -> u32 {
    let entries = HashSet::<u32>::from_iter(
        input
            .lines()
            .into_iter()
            .filter_map(|s| s.trim().parse::<u32>().ok()),
    );
    entries
        .iter()
        .by_ref()
        .find_map(|x| {
            entries
                .iter()
                .filter(|y| *y <= x && (x + *y) <= 2020)
                .find_map(|y| entries.get(&(2020 - (x + y))).map(|z| x * y * z))
        })
        .unwrap()
        .clone()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const sample: &str = "1721
    979
    366
    299
    675
    1456";

    #[test]
    fn sample1() {
        assert_eq!(part1(&sample), 514579);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&sample), 241861950);
    }
}
