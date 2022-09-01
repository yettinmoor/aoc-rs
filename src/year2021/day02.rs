use std::str::FromStr;

use crate::common::run::Run;
use crate::vec_input;

pub struct Runner {}

impl Run for Runner {
    vec_input!(Step);

    type Output = usize;

    fn part1(&self, input: &Self::Input) -> Self::Output {
        let (x, y) = input.iter().fold((0, 0), |(x, y), step| match step.dir {
            Dir::Forward => (x + step.mag, y),
            Dir::Up => (x, y - step.mag),
            Dir::Down => (x, y + step.mag),
        });
        x * y
    }

    fn part2(&self, input: &Self::Input) -> Self::Output {
        let (x, y, _) = input
            .iter()
            .fold((0, 0, 0), |(x, y, aim), step| match step.dir {
                Dir::Forward => (x + step.mag, y + step.mag * aim, aim),
                Dir::Up => (x, y, aim - step.mag),
                Dir::Down => (x, y, aim + step.mag),
            });
        x * y
    }
}

pub struct Step {
    dir: Dir,
    mag: usize,
}

enum Dir {
    Forward,
    Up,
    Down,
}

impl FromStr for Step {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split_whitespace();
        let dir = match it.next().unwrap() {
            "forward" => Dir::Forward,
            "up" => Dir::Up,
            "down" => Dir::Down,
            _ => return Err(()),
        };
        let mag = it.next().unwrap().parse::<usize>().unwrap();
        Ok(Step { dir, mag })
    }
}
