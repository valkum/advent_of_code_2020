use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use std::iter::FromIterator;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug,Copy,Clone)]
pub enum Action {
    North(usize),
    East(usize),
    South(usize),
    West(usize),
    Right(usize),
    Left(usize),
    Forward(usize)
}

impl FromStr for Action {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let number = s.chars().skip(1).collect::<String>().parse()?;
        Ok(match s.chars().nth(0).unwrap() {
            'N' => Action::North(number),
            'E' => Action::East(number),
            'S' => Action::South(number),
            'W' => Action::West(number),
            'R' => Action::Right(number),
            'L' => Action::Left(number),
            'F' => Action::Forward(number),
            _ => unimplemented!(),
        })
    }
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<Action> {
    Vec::from_iter(
        input
            .lines()
            .map(|s| s.trim().parse::<Action>().ok().unwrap()),
    )
}

fn rotate(pos: (i32, i32), degree: i32) -> (i32, i32) {
    match degree {
        90|-270 => (-pos.1, pos.0),
        180|-180 => (-pos.0, -pos.1),
        270|-90 => (pos.1, -pos.0),
        _ => unimplemented!()
    }
}
#[derive(Debug, Clone)]
struct Ship {
    direction: usize, // CW, 0 = East
    pub x: i32, // East+
    pub y: i32, // North+
    waypoint_x: i32, // East+
    waypoint_y: i32, // North+
}

impl Ship {
    fn new() -> Self {
        Ship {
            direction: 0,
            x: 0,
            y: 0,
            waypoint_x: 10,
            waypoint_y: 1
        }
    }

    fn process_action(&mut self, action: &Action) {
        match *action {
            Action::North(distance) => {self.y += distance as i32}
            Action::East(distance) => {self.x += distance as i32}
            Action::South(distance) => {self.y -= distance as i32}
            Action::West(distance) => {self.x -= distance as i32}
            Action::Right(angle) => {self.direction = (self.direction + angle) % 360}
            Action::Left(angle) => {self.direction = (360 + self.direction - angle) % 360}
            Action::Forward(distance) => {
                match self.direction {
                    0 => {self.x += distance as i32}
                    90 => {self.y -= distance as i32}
                    180 => {self.x -= distance as i32}
                    270 => {self.y += distance as i32}
                    _ => unimplemented!(),
                }
            }
        }
    }

    fn process_action_correctly(&mut self, action: &Action) {
        match *action {
            Action::North(distance) => {self.waypoint_y += distance as i32}
            Action::East(distance) => {self.waypoint_x += distance as i32}
            Action::South(distance) => {self.waypoint_y -= distance as i32}
            Action::West(distance) => {self.waypoint_x -= distance as i32}

            Action::Right(angle) => {
                let pos = rotate((self.waypoint_x, self.waypoint_y), -(angle as i32));
                self.waypoint_x = pos.0;
                self.waypoint_y = pos.1;
            }
            Action::Left(angle) => {
                let pos = rotate((self.waypoint_x, self.waypoint_y), angle as i32);
                self.waypoint_x = pos.0;
                self.waypoint_y = pos.1;
            }

            Action::Forward(distance) => {
                self.x = self.x + self.waypoint_x * distance as i32;
                self.y = self.y + self.waypoint_y * distance as i32;
            }
        }
    }
}

#[aoc(day12, part1)]
pub fn part1(input: &[Action]) -> i32 {
    let mut ship = Ship::new();
    for action in input {
        ship.process_action(&action);
    }
    return ship.x.abs() + ship.y.abs()
}



#[aoc(day12, part2)]
pub fn part2(input: &[Action]) -> i32 {
    let mut ship = Ship::new();
    for action in input {
        ship.process_action_correctly(&action);
    }
    return ship.x.abs() + ship.y.abs()
}

#[cfg(test)]
mod tests {
    use super::{*};

    const sample: &str = "F10
N3
F7
R90
F11";

    #[test]
    fn sample1() {
        assert_eq!(part1(&input_generator(&sample)), 25);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&input_generator(&sample)), 286);
    }
}
