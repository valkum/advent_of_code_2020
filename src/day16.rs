use std::{collections::HashMap, ops::RangeInclusive};

use anyhow::Result;
use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[derive(Debug, Clone)]
pub struct Input {
    pub validation: HashMap<String, Vec<RangeInclusive<u32>>>,
    pub my_ticket: Vec<u32>,
    pub other_tickets: Vec<Vec<u32>>,
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Input {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let mut validation: HashMap<String, Vec<RangeInclusive<u32>>> = HashMap::new();
    let mut my_ticket: Vec<u32> = Vec::new();
    let mut other_tickets: Vec<Vec<u32>> = Vec::new();
    for line in parts[0].lines() {
        let split_pos = line.find(":").unwrap();
        let mut ranges = Vec::new();
        for range in line[split_pos + 1..line.len()].split(" or ") {
            let split_pos = range.find("-").unwrap();
            ranges.push(RangeInclusive::new(
                range[0..split_pos].trim().parse::<u32>().unwrap(),
                range[split_pos + 1..range.len()]
                    .trim()
                    .parse::<u32>()
                    .unwrap(),
            ));
        }
        validation.insert(line[0..split_pos].to_owned(), ranges);
    }
    for line in parts[1].lines().skip(1) {
        my_ticket = line
            .split(',')
            .map(|s| s.trim().parse::<u32>().unwrap())
            .collect();
    }
    for line in parts[2].lines().skip(1) {
        let ticket = line
            .split(',')
            .map(|s| s.trim().parse::<u32>().unwrap())
            .collect();
        other_tickets.push(ticket);
    }
    Input {
        validation,
        my_ticket,
        other_tickets,
    }
}

#[aoc(day16, part1)]
pub fn part1(input: &Input) -> u32 {
    input
        .other_tickets
        .iter()
        .map(|ticket| {
            ticket
                .iter()
                .filter_map(|value| {
                    if input
                        .validation
                        .iter()
                        .any(|(_, ranges)| ranges.iter().any(|range| range.contains(value)))
                    {
                        return None;
                    }
                    return Some(value);
                })
                .map(|s| *s)
                .collect::<Vec<u32>>()
        })
        .flatten()
        .sum()
}
#[inline]
fn part2_helper(input: &Input) -> Vec<(String, u64)> {
    let mut classes: HashMap<String, Vec<u64>> = HashMap::new();
    let other_tickets = input.other_tickets.iter().filter(|ticket| {
        ticket
            .iter()
            .filter_map(|value| {
                if input
                    .validation
                    .iter()
                    .any(|(_, ranges)| ranges.iter().any(|range| range.contains(value)))
                {
                    return None;
                }
                return Some(value);
            })
            .count()
            == 0
    });
    let len = input.validation.len().clone();
    let validation = input.validation.clone();
    for i in 0..len {
        for (class, ranges) in validation.iter() {
            if other_tickets
                .clone()
                .all(|ticket| ranges.iter().any(|r| r.contains(&ticket[i])))
            {
                if let Some(v) = classes.get_mut(class) {
                    v.push(input.my_ticket[i] as u64);
                } else {
                    classes.insert(class.clone(), vec![input.my_ticket[i] as u64]);
                }
            }
        }
    }
    let mut unique_classes: Vec<(String, u64)> = Vec::new();
    while classes.len() != 0 {
        {
            unique_classes.extend(
                classes
                    .iter()
                    .filter(|s| s.1.len() == 1)
                    .map(|s| (s.0.clone(), s.1[0])),
            );
        }
        {
            for uniqe_class in unique_classes.iter() {
                classes.iter_mut().for_each(|x| {
                    let class = x.1;

                    if let Some(pos) = class.iter().position(|s| *s == uniqe_class.1) {
                        class.remove(pos);
                    }
                });
            }
            classes
                .clone()
                .iter()
                .filter(|s| s.1.len() == 0)
                .for_each(|s| {
                    classes.remove(s.0);
                });
        }
    }
    unique_classes
}

#[aoc(day16, part2)]
pub fn part2(input: &Input) -> u64 {
    let my_classes = part2_helper(input);
    assert_eq!(my_classes.len(), input.validation.len());

    my_classes
        .iter()
        .filter(|(class, _)| class.starts_with("departure"))
        .map(|(_, value)| value)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: &str = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
    const SAMPLE2: &str = "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";

    #[test]
    fn sample1() {
        assert_eq!(part1(&input_generator(&SAMPLE1)), 71);
    }

    #[test]
    fn sample2() {
        assert!(vec![
            ("row".to_owned(), 11),
            ("class".to_owned(), 12),
            ("seat".to_owned(), 13)
        ]
        .iter()
        .all(|s| part2_helper(&input_generator(&SAMPLE2)).contains(s)));
    }
}
