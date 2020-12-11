use anyhow::Result;
use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<u64> {
    let mut input: Vec<u64> = input.lines().map(|x| x.parse::<u64>().unwrap()).collect();
    input.push(0);
    input.sort();
    input.push(input.last().unwrap() + 3);
    input
}


#[aoc(day10, part1)]
pub fn part1(input: &Vec<u64>) -> u64 {
    let mut differences = vec![0u64; 3];
    // dbg!(&input);
    input.windows(2).for_each(|x| {
        let difference = x[1] - x[0];
        // dbg!(&differences, &difference);
        differences[(difference - 1) as usize] = differences
            .get((difference - 1) as usize)
            .unwrap_or(&0)
            .checked_add(1)
            .unwrap()
    });
    differences[0] * differences[2]
}

#[aoc(day10, part2)]
pub fn part2(input: &Vec<u64>) -> u64 {
    let mut i = input.len() - 1;
    let mut arrangements: Vec<u64> = vec![0; input.len()];
    arrangements[input.len() - 1] = 1;
    loop {
        i -= 1;

        let z = match input.len() - i - 1 {
            0 => unimplemented!(),
            1 => (input[i + 1] - input[i], 99, 99),
            2 => (input[i + 1] - input[i], input[i + 2] - input[i], 99),
            _ => (
                input[i + 1] - input[i],
                input[i + 2] - input[i],
                input[i + 3] - input[i],
            ),
        };

        match z {
            (1, 2, 3) => {
                arrangements[i] = arrangements[i + 1] + arrangements[i + 2] + arrangements[i + 3]
            }
            (1, 2, _) | (1, 3, _) | (2, 3, _) => {
                arrangements[i] = arrangements[i + 1] + arrangements[i + 2]
            }
            (1, _, _) | (2, _, _) | (3, _, _) => arrangements[i] = arrangements[i + 1],
            _ => panic!("missing pattern {:?}", z),
        };

        if i == 0 {
            break;
        }
    }
    return arrangements[0];
}

#[aoc(day10, part1)]
pub fn part1(input: &Vec<u64>) -> u64 {
    let result = find_part1(input.into()).expect("find_part1 failed");
    result[0] * result[2]
}

#[aoc(day10, part2)]
pub fn part2(input: &Vec<u64>) -> u64 {
    find_part2(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "16
10
15
5
1
11
7
19
6
12
4";

    const SAMPLE2: &str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    #[test]
    fn sample1() {
        let input = input_generator(&SAMPLE);
        let result = part1(&input);
        assert_eq!(result, 35);

        let input = input_generator(&SAMPLE2);
        let result = part1(&input);
        assert_eq!(result, 220);
    }

    #[test]
    fn sample2_1() {
        let input = input_generator(&SAMPLE);
        let result = part2(&input);
        assert_eq!(result, 8);
    }
    #[test]
    fn sample2_2() {
        let input = input_generator(&SAMPLE2);
        let result = part2(&input);
        assert_eq!(result, 19208);
    }
}
