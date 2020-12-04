use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::str::FromStr;
use std::num::ParseIntError;
use anyhow::Result;

#[derive(Debug,Default)]
pub struct Entry {
    byr: Option<usize>,
    iyr: Option<usize>,
    eyr: Option<usize>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<usize>
}



impl FromStr for Entry {
    type Err = anyhow::Error;
    
    fn from_str(s: &str) -> Result<Self> {
        let params: Vec<&str> = s.split(" ").skip(1).collect();
        let mut build = Entry::default();
        for param in params {
            let part: Vec<&str> = param.split(":").collect();
            match part[0] {
                "byr" => {build.byr = Some(part[1].parse::<usize>()?)},
                "iyr" => {build.iyr = Some(part[1].parse::<usize>()?)},
                "eyr" => {build.eyr = Some(part[1].parse::<usize>()?)},
                "hgt" => {build.hgt = Some(part[1].to_owned())},
                "hcl" => {build.hcl = Some(part[1].to_owned())},
                "ecl" => {build.ecl = Some(part[1].to_owned())},
                "pid" => {build.pid = Some(part[1].to_owned())},
                "cid" => {build.cid = Some(part[1].parse::<usize>()?)},
                _ => unimplemented!(),
            }
        }
        dbg!(&build);
        return Ok(build)
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Entry> {
    let mut entries: Vec<Entry> = Vec::new();
    let mut buf: String = String::new();
    for line in input.lines() {
        if line.trim() == "" {
            match buf.parse::<Entry>() {
                Ok(entry) => entries.push(entry),
                Err(err) => {dbg!(err, &buf);}
            } 
            buf.clear()
        } else {
            buf.push_str(" ");
            buf.push_str(line.trim());
        }
    }
    return entries
}

#[aoc(day4, part1)]
pub fn part1(input: &[Entry]) -> usize {
    input.iter().filter_map(|e| {
        if e.byr.is_some() & e.iyr.is_some() & e.eyr.is_some() & e.hgt.is_some() & e.hcl.is_some() & e.ecl.is_some() & e.pid.is_some() {
            Some(e)
        } else {
            None
        }
    }).count()
}

#[aoc(day4, part2)]
pub fn part2(input: &[Entry]) -> usize {
   return 0
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, input_generator};

    const SAMPLE: &str= "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
    byr:1937 iyr:2017 cid:147 hgt:183cm
    
    iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
    hcl:#cfa07d byr:1929
    
    hcl:#ae17e1 iyr:2013
    eyr:2024
    ecl:brn pid:760753108 byr:1931
    hgt:179cm
    
    hcl:#cfa07d eyr:2025 pid:166559648
    iyr:2011 ecl:brn hgt:59in";

    #[test]
    fn sample1() {
        assert_eq!(part1(&input_generator(&SAMPLE)), 2);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&input_generator(&SAMPLE)), 1);
    }
}