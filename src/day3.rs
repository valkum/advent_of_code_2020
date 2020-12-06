use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use std::iter::FromIterator;
use array2d::*;


#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Array2D<bool> {
    let rows = Vec::from_iter(input.lines()
    .map(|s| s.trim().bytes().map(|b| b == b'#').collect()));
    Array2D::from_rows(&rows)
}

#[aoc(day3, part1)]
pub fn part1(input: &Array2D<bool>) -> usize {
    let len = input.num_columns();
    let mut count = 0;
    for i in 0..input.num_rows() {
        if input[(i*1, i*3 % len)] {
            count += 1;
        }
    }
    return count
}

#[aoc(day3, part2)]
pub fn part2(input: &Array2D<bool>) -> usize {
    let len = input.num_columns();
    let mut count1 = 0;
    let mut count2 = 0;
    let mut count3 = 0;
    let mut count4 = 0;
    let mut count5 = 0;
    for i in 0..input.num_rows() {
        if input[(i*1, i*1 % len)] {
            count1 += 1;
        }
        if input[(i*1, i*3 % len)] {
            count2 += 1;
        }
        if input[(i*1, i*5 % len)] {
            count3 += 1;
        }
        if input[(i*1, i*7 % len)] {
            count4 += 1;
        }
    }
    for i in 0..input.num_rows()/2 {
        if input[(i*2, i*1 % len)] {
            count5 += 1;
        }
    }
    return count1 * count2 * count3 * count4 * count5
}

#[aoc(day3, part2, Iterator)]
pub fn part2_iterator(input: &Array2D<bool>) -> usize {
    let len = input.num_columns();
    let slopes = [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];
    slopes.iter().map(|&(r,c)| {
        input.rows_iter()
            .enumerate()
            .step_by(r)
            .map(|(i, mut row)| row.nth((i/r * c) % len).unwrap_or(&false))
            .filter(|&b| *b)
            .count()
    }).product()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, part2_iterator, input_generator};

    const SAMPLE: &str= "..##.......
        #...#...#..
        .#....#..#.
        ..#.#...#.#
        .#...##..#.
        ..#.##.....
        .#.#.#....#
        .#........#
        #.##...#...
        #...##....#
        .#..#...#.#";

    #[test]
    fn sample1() {
        assert_eq!(part1(&input_generator(&SAMPLE)), 7);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&input_generator(&SAMPLE)), 336);
    }

    #[test]
    fn sample2_iterator() {
        assert_eq!(part2_iterator(&input_generator(&SAMPLE)), 336);
    }
}