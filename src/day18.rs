use anyhow::Result;
use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use nom::{
    branch::alt,
    character::complete::{char, digit1, one_of, space0},
    combinator::map_res,
    multi::fold_many0,
    sequence::{delimited, pair},
    IResult,
};
// Mostly taken from the Nom banchmarks

fn factor(input: &[u8]) -> IResult<&[u8], i64> {
    delimited(
        space0,
        alt((
            map_res(digit1, |digits| {
                unsafe { std::str::from_utf8_unchecked(digits) }.parse()
            }),
            delimited(char('('), expr, char(')')),
        )),
        space0,
    )(input)
}

fn expr(input: &[u8]) -> IResult<&[u8], i64> {
    let (input, init) = factor(input)?;
    fold_many0(pair(one_of("+*"), factor), init, |acc, (op, val)| {
        if op == '+' {
            acc + val
        } else {
            acc * val
        }
    })(input)
}

fn factor_part2(input: &[u8]) -> IResult<&[u8], i64> {
    delimited(
        space0,
        alt((
            map_res(digit1, |digits| {
                unsafe { std::str::from_utf8_unchecked(digits) }.parse()
            }),
            delimited(char('('), term, char(')')),
        )),
        space0,
    )(input)
}

fn expr_2(input: &[u8]) -> IResult<&[u8], i64> {
    let (input, init) = factor_part2(input)?;
    fold_many0(pair(one_of("+"), factor_part2), init, |acc, (op, val)| {
        if op == '+' {
            acc + val
        } else {
            acc
        }
    })(input)
}

fn term(input: &[u8]) -> IResult<&[u8], i64> {
    let (input, init) = expr_2(input)?;
    fold_many0(pair(one_of("*"), expr_2), init, |acc, (op, val)| {
        if op == '*' {
            acc * val
        } else {
            acc
        }
    })(input)
}

#[aoc(day18, part1)]
pub fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| expr(line.as_bytes()).unwrap().1)
        .sum()
}

#[aoc(day18, part2)]
pub fn part2(input: &str) -> i64 {
    input
        .lines()
        .map(|line| term(line.as_bytes()).unwrap().1)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: &str = "2 * 3 + (4 * 5)";
    const SAMPLE2: &str = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
    const SAMPLE3: &str = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
    const SAMPLE4: &str = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";

    #[test]
    fn sample1() {
        assert_eq!(part1(&SAMPLE1), 26);
        assert_eq!(part1(&SAMPLE2), 437);
        assert_eq!(part1(&SAMPLE3), 12240);
        assert_eq!(part1(&SAMPLE4), 13632);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&SAMPLE1), 46);
        assert_eq!(part2(&SAMPLE2), 1445);
        assert_eq!(part2(&SAMPLE3), 669060);
        assert_eq!(part2(&SAMPLE4), 23340);
    }
}
