use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<u64> {
    input.lines().map(|x| x.parse::<u64>().unwrap()).collect()
}

fn find_part1(input: &[u64], preamble: usize) -> impl Iterator<Item=u64> + '_ {
    let mut iter = input.iter().enumerate();
    for i in 0..preamble {
        iter.next().ok_or(i);
    }
    iter.filter_map(move |(i, &x)| {
        for (i2, &y) in input[(i-preamble)..i].iter().enumerate() {
            for &z in input[(i-preamble)+i2..i].iter() {
                if y+z==x && z != y {
                    return None;
                }
            }
        }
        return Some(x)
    })
}

fn find_part2(input: &[u64], preamble: usize) -> Vec<u64> {
    let target = find_part1(input, preamble).next().unwrap();
    input.iter().enumerate().filter_map(move |(i, _)| {
        let mut stack = Vec::new();
        let mut acc = 0;
        for x in input[i..input.len()].iter() {
            acc += x;
            stack.push(*x);
            if acc == target {
                return Some(stack);
            }
            if acc >= target {
                return None
            }
        }
        return None
    }).next().unwrap()

}

#[aoc(day9, part1)]
pub fn part1(input: &[u64]) -> u64 {
    find_part1(input, 25).next().unwrap()
}

#[aoc(day9, part2)]
pub fn part2(input: &[u64]) -> u64 {
    let range = find_part2(input, 25);
    range.iter().min().unwrap() + range.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;
    use super::*;
    lazy_static!{
        static ref SAMPLE: Vec<u64> = (1..=100).into_iter().collect();
    }

    const SAMPLE2: &str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    #[test]
    fn sample1() {
        assert!(find_part1(&SAMPLE, 25).find(|&x| x == 100 || x ==50).is_some());
        let input = input_generator(&SAMPLE2);
        assert_eq!(find_part1(&input, 5).collect::<Vec<u64>>(), vec![127]);
    }

    #[test]
    fn sample2() {
        let input = input_generator(&SAMPLE2);
        assert_eq!(find_part2(&input, 5), vec![15,25,47,40]);
    }
}
