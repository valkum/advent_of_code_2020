use std::{cell::RefCell, char, str::FromStr};

use anyhow::Result;
use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use itertools::Itertools;
use nom::{IResult, Parser, bits::complete::take, branch::alt, branch::permutation, character::complete::{alpha0, anychar, char, digit1, one_of, space0, space1}, combinator::{map_res}, multi::{fold_many0, separated_list1}, sequence::{delimited, pair, preceded, separated_pair}};
use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
enum Rule {
    Or(Vec<Vec<usize>>),
    Alphabet(char)
}
impl Rule {
    fn is_char(&self) -> bool {
        match self {
            Self::Alphabet(_) => true,
            _ => false
        }
    }
}

pub struct Input {
    rules: Vec<String>,
    strings: Vec<String>
}

fn parse_rule_alphabet(input: &str) -> IResult<&str, Rule> {
    let (input, alphabet) = delimited(char('"'), anychar, char('"'))(input)?;
    Ok((input, Rule::Alphabet(alphabet as char)))

}

fn parse_rule_or(input: &str) -> IResult<&str, Rule> {
    let (input, sub) = separated_list1(preceded(space0, char('|')), preceded(space0, separated_list1(space1, map_res(digit1,FromStr::from_str))))(input)?;

    Ok((input, Rule::Or(sub)))
}


#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> Input {
    let rules = input.split("\n\n").collect::<Vec<&str>>();
    let mut rule_vec = rules[0].lines()
        .map(|s| {
            let line = s.trim();
            let split_pos = line.find(":").unwrap();
            ((line[0..split_pos]).parse::<usize>().unwrap(), (line[split_pos+1..line.len()].to_owned()))
        }).collect::<Vec<(usize, String)>>();
        rule_vec.sort_by_key(|k| k.0);

    Input { 
        rules: rule_vec.iter().map(|r| r.1.clone()).collect(),
        strings: rules[1].lines().map(|s| s.trim().to_owned()).collect()
    }
}

fn build_regex(string: &mut String, index: usize, rules: &Vec<Rule>) {

    match &rules[index] {
        Rule::Alphabet(c) => string.push(*c),
        Rule::Or(vec) => {
            if vec.len() > 1 {
                string.push_str("(?:")
            }
            for (n, ands) in vec.iter().enumerate() {
                if n > 0 {
                    string.push('|');
                }
                for &and in ands {
                    build_regex(string, and, rules);
                }
            }
            if vec.len() > 1 {
                string.push(')');
            }
        }
    }

}

#[aoc(day19, part1)]
pub fn part1(input: &Input) -> i64 {
    let rules = input.rules.iter().map(|rule| alt((parse_rule_or, parse_rule_alphabet))(rule.trim()).expect("Could not parse rule")).map(|(_, r)| r).collect::<Vec<Rule>>();

    let mut regex_string = "(?m)^".to_string();
    build_regex(&mut regex_string, 0, &rules);
    regex_string.push('$');
    let regex = Regex::new(&regex_string).unwrap();
    let mut valid = 0;
    for line in input.strings.clone() {

        if regex.is_match(line.as_str()) {
            valid += 1;
        }
    }
    valid
}

#[aoc(day19, part2)]
pub fn part2(input: &Input) -> u64 {
    let mut strrules = input.rules.clone();
    strrules[8] = "42 | 42 8".to_owned();
    strrules[11] = "42 31 | 42 11 31".to_owned();

    let rules = strrules.iter().map(|rule| alt((parse_rule_or, parse_rule_alphabet))(rule.trim()).expect("Could not parse rule")).map(|(_, r)| r).collect::<Vec<Rule>>();

    let mut r42 = "(".to_owned();
    build_regex(&mut r42, 42, &rules);
    r42.push(')');
    let r42_r = Regex::new(&r42).unwrap();

    let mut r31 = "(".to_owned();
    build_regex(&mut r31, 31, &rules);
    r31.push(')');
    let r31_r = Regex::new(&r31).unwrap();

    let from_42_to_31_r = Regex::new(&format!(r"^(?P<start_42>(?:{})+)(?P<end_31>(?:{})+)$", r42, r31)).unwrap();

    let mut valid = 0u64;
    for line in input.strings.clone() {
        if from_42_to_31_r.is_match(line.as_str()) {
            let c = from_42_to_31_r.captures_iter(line.as_str()).collect::<Vec<_>>();
            if c.len() == 1 {
                let c = c.first().unwrap();
                let c31 = r31_r.find_iter(&c["end_31"]).count();
                let c42 = r42_r.find_iter(&c["start_42"]).count();
                if c42 >= c31+1 {
                    valid += 1;
                } 
            }
        }
    }
    valid
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: &str = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#;

    #[test]
    fn sample1() {
        assert_eq!(part1(&input_generator(&SAMPLE1)), 2);
    }

    // #[test]
    // fn sample2() {
    //     assert_eq!(part2(&SAMPLE1), 46);
    //     assert_eq!(part2(&SAMPLE2), 1445);
    //     assert_eq!(part2(&SAMPLE3), 669060);
    //     assert_eq!(part2(&SAMPLE4), 23340);
    // }
}
