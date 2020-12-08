use anyhow::Result;
use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use std::str::FromStr;

#[derive(Debug, Default)]
pub struct Entry {
    byr: Option<usize>,
    iyr: Option<usize>,
    eyr: Option<usize>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<usize>,
}

impl FromStr for Entry {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let params: Vec<&str> = s.split(" ").collect();
        let mut build = Entry::default();
        for param in params {
            let part: Vec<&str> = param.split(":").collect();
            match part[0] {
                "byr" => build.byr = Some(part[1].parse::<usize>()?),
                "iyr" => build.iyr = Some(part[1].parse::<usize>()?),
                "eyr" => build.eyr = Some(part[1].parse::<usize>()?),
                "hgt" => build.hgt = Some(part[1].to_owned()),
                "hcl" => build.hcl = Some(part[1].to_owned()),
                "ecl" => build.ecl = Some(part[1].to_owned()),
                "pid" => build.pid = Some(part[1].to_owned()),
                "cid" => build.cid = Some(part[1].parse::<usize>()?),
                _ => unimplemented!(),
            }
        }
        return Ok(build);
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Entry> {
    let mut entries: Vec<Entry> = Vec::new();
    for line in input.split("\n\n") {
        match line.replace("\n", " ").parse::<Entry>() {
            Ok(entry) => entries.push(entry),
            Err(err) => {
                dbg!(err, &line);
            }
        }
    }
    return entries;
}

#[aoc(day4, part1)]
pub fn part1(input: &[Entry]) -> usize {
    input
        .iter()
        .filter_map(|e| {
            if e.byr.is_some()
                && e.iyr.is_some()
                && e.eyr.is_some()
                && e.hgt.is_some()
                && e.hcl.is_some()
                && e.ecl.is_some()
                && e.pid.is_some()
            {
                Some(e)
            } else {
                None
            }
        })
        .count()
}
fn is_valid_hgt(input: &str) -> bool {
    match input[0..input.len() - 2].parse::<usize>() {
        Ok(hgt) => match input[input.len() - 2..input.len()].as_ref() {
            "cm" => hgt >= 150 && hgt <= 193,
            "in" => hgt >= 59 && hgt <= 76,
            _ => false,
        },
        Err(_err) => false,
    }
}
fn is_valid_hcl(input: &str) -> bool {
    return input.starts_with("#")
        && input[1..input.len()]
            .bytes()
            .all(|b| (b >= b'0' && b <= b'9') || (b >= b'a' && b <= b'f'));
}
fn is_valid_ecl(input: &str) -> bool {
    match input {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
        _ => false,
    }
}
fn is_valid_pid(input: &str) -> bool {
    return input.len() == 9 && input.bytes().all(|b| b >= b'0' && b <= b'9');
}

#[aoc(day4, part2)]
pub fn part2(input: &[Entry]) -> usize {
    input
        .iter()
        .filter_map(|e| {
            if e.byr.is_some()
                && e.iyr.is_some()
                && e.eyr.is_some()
                && e.hgt.is_some()
                && e.hcl.is_some()
                && e.ecl.is_some()
                && e.pid.is_some()
                && e.byr.unwrap() >= 1920
                && e.byr.unwrap() <= 2002
                && e.iyr.unwrap() >= 2010
                && e.iyr.unwrap() <= 2020
                && e.eyr.unwrap() >= 2020
                && e.eyr.unwrap() <= 2030
                && is_valid_hgt(e.hgt.as_ref().unwrap())
                && is_valid_hcl(e.hcl.as_ref().unwrap())
                && is_valid_ecl(e.ecl.as_ref().unwrap())
                && is_valid_pid(e.pid.as_ref().unwrap())
            {
                Some(e)
            } else {
                None
            }
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const SAMPLE: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";
    const SAMPLE_INVALID: &str = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

    const SAMPLE_VALID: &str = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

    #[test]
    fn sample1() {
        assert_eq!(part1(&input_generator(&SAMPLE)), 2);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&input_generator(&SAMPLE_INVALID)), 0);
        assert_eq!(part2(&input_generator(&SAMPLE_VALID)), 4);
    }
}
