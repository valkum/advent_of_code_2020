use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use std::{cmp::Ordering, fmt::Debug, iter::FromIterator};
use std::num::ParseIntError;
use std::str::FromStr;
use anyhow::Result;
use num_integer::*;


#[derive(Debug, Clone)]
pub struct Input {
    pub earliest_time: usize,
    pub busses: Vec<Option<usize>>
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Input {
    let mut iter = input.split("\n");
    Input {
        earliest_time: iter.nth(0).unwrap().parse::<usize>().unwrap(),
        busses: iter.nth(0).unwrap().trim().split(",").map(|x| x.parse::<usize>().ok()).collect()
    }
}



#[aoc(day13, part1)]
pub fn part1(input: &Input) -> usize {
    let bus = input.busses.iter()
    .filter_map(|x| -> Option<usize> {*x})
    .map(|x| (x, x - input.earliest_time % x))
    .min_by(|(x,y), (x2, y2)| y.cmp(y2)).unwrap();
    
    bus.0 * bus.1
}


#[aoc(day13, part2)]
#[allow(non_snake_case)]
pub fn part2(input: &Input) -> i64 {

    let iter = input.busses.iter().enumerate().filter(|(_,bus)| bus.is_some()).map(|(i, bus)| (i, bus.unwrap() as i64));

    let N: i64 = iter.clone().map(|(_,n)| n).product();

    iter.clone().map(|(a, n)| {
        let a = -(a as i64).rem_euclid(n);
        let N_i: i64 = N / n;
        let bezout = N_i.extended_gcd(&n);
        (a*bezout.x*N_i).rem_euclid(N)
    }).sum::<i64>().rem_euclid(N)
}

#[aoc(day13, part2, constructive)]
#[allow(non_snake_case)]
pub fn part2_constructive(input: &Input) -> i64 {

    let mut iter = input.busses.iter().enumerate().filter(|(_,bus)| bus.is_some()).map(|(i, bus)| (-(i as i64).rem_euclid(bus.unwrap() as i64), bus.unwrap() as i64));

    let mut acc = iter.next().unwrap();
    for cur in iter {
        let bezout = acc.1.extended_gcd(&cur.1);
        acc = ((acc.0*bezout.y*cur.1 + cur.0*bezout.x*acc.1).rem_euclid(acc.1*cur.1), acc.1*cur.1)
    }

    acc.0.rem_euclid(acc.1)
}

#[cfg(test)]
mod tests {
    use super::{*};

    const SAMPLE: &str = "939
    7,13,x,x,59,x,31,19";
    const SAMPLE2: &str = "3123
    17,x,13,19";
    const SAMPLE3: &str = "3123
    67,7,59,61";


    #[test]
    fn sample1() {
        assert_eq!(part1(&input_generator(&SAMPLE)), 295);
    }

    #[test]
    fn sample2_1() {
        assert_eq!(part2(&input_generator(&SAMPLE)), 1068781);
        assert_eq!(part2_constructive(&input_generator(&SAMPLE)), 1068781);
    }
    #[test]
    fn sample2_2() {
        assert_eq!(part2(&input_generator(&SAMPLE2)), 3417);
        assert_eq!(part2_constructive(&input_generator(&SAMPLE2)), 3417);
    }
    #[test]
    fn sample2_3() {
        assert_eq!(part2(&input_generator(&SAMPLE3)), 754018);
        assert_eq!(part2_constructive(&input_generator(&SAMPLE3)), 754018);
    }
}
