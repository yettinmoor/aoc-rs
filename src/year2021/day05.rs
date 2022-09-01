use std::str::FromStr;

use crate::common::counter::Countable;
use crate::common::run::Run;
use crate::vec_input;

pub struct Runner {}

impl Run for Runner {
    vec_input!(Line);
    type Output = usize;

    fn part1(&self, input: &Self::Input) -> Self::Output {
        solve(input, false)
    }

    fn part2(&self, input: &Self::Input) -> Self::Output {
        solve(input, true)
    }
}

fn solve(lines: &[Line], diagonals: bool) -> usize {
    lines
        .iter()
        .flat_map(|line| line.point_set(diagonals))
        .counter()
        .into_values()
        .filter(|&c| c > 1)
        .count()
}

type Point = (usize, usize);

pub struct Line {
    a: Point,
    b: Point,
}

impl FromStr for Line {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(" -> ");
        let mut get_point = || {
            let xs = it
                .next()
                .unwrap()
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect::<Vec<_>>();
            (xs[0], xs[1])
        };
        let a = get_point();
        let b = get_point();
        Ok(Line { a, b })
    }
}

impl Line {
    fn point_set(&self, diagonals: bool) -> Vec<Point> {
        let (x1, y1) = self.a;
        let (x2, y2) = self.b;
        if x1 == x2 {
            (y1.min(y2)..=y1.max(y2)).map(|y| (x1, y)).collect()
        } else if y1 == y2 {
            (x1.min(x2)..=x1.max(x2)).map(|x| (x, y1)).collect()
        } else if diagonals {
            if (x1 < x2) == (y1 < y2) {
                (x1.min(x2)..=x1.max(x2))
                    .zip(y1.min(y2)..=y1.max(y2))
                    .collect::<Vec<_>>()
            } else {
                (x1.min(x2)..=x1.max(x2))
                    .rev()
                    .zip(y1.min(y2)..=y1.max(y2))
                    .collect::<Vec<_>>()
            }
        } else {
            vec![]
        }
    }
}
