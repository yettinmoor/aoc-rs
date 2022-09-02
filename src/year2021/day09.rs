use std::collections::HashSet;

use crate::common::matrix::{Coord, Matrix};
use crate::common::run::Run;
use crate::common::walk::Walk;
use crate::matrix_input;

pub struct Runner {}

impl Run for Runner {
    matrix_input!(usize);
    type Output = usize;

    fn part1(&self, input: &Self::Input) -> Self::Output {
        input
            .flat_iter()
            .filter_map(|(coord, x)| {
                input
                    .neighbors(coord, false)
                    .iter()
                    .all(|&(_, n)| n > x)
                    .then_some(x + 1)
            })
            .sum()
    }

    fn part2(&self, input: &Self::Input) -> Self::Output {
        let mut sizes = input
            .flat_iter()
            .filter(|&(coord, x)| input.neighbors(coord, false).iter().all(|&(_, n)| n > x))
            .map(|(low_point, _)| {
                input
                    .walk(
                        low_point,
                        HashSet::<Coord>::new(),
                        |region, prev, coord, value| {
                            if value == 9 {
                                return;
                            }
                            if prev.map_or(true, |prev| input.get_coord(prev).unwrap() < value) {
                                region.insert(coord);
                            }
                        },
                        |coord| {
                            input
                                .neighbors(coord, false)
                                .iter()
                                .filter_map(|&(c, v)| {
                                    (v > input.get_coord(coord).unwrap()).then_some(c)
                                })
                                .collect::<Vec<_>>()
                        },
                    )
                    .len()
            })
            .collect::<Vec<_>>();
        sizes.sort_by(|a, b| b.cmp(a));
        sizes.iter().take(3).product()
    }
}
