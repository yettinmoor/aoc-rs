use std::collections::HashSet;
use std::iter::repeat_with;

use crate::common::matrix::Matrix;
use crate::common::run::Run;
use crate::matrix_input;

pub struct Runner {}

impl Run for Runner {
    matrix_input!(usize);
    type Output = usize;

    fn part1(&self, input: &Self::Input) -> Self::Output {
        let mut input = input.clone();
        (0..100).map(|_| step(&mut input)).sum()
    }

    fn part2(&self, input: &Self::Input) -> Self::Output {
        let mut input = input.clone();
        let size = input.size();
        repeat_with(|| step(&mut input))
            .take_while(|&flashes| flashes < size)
            .count()
            + 1
    }
}

fn step(nodes: &mut Vec<Vec<usize>>) -> usize {
    for v in nodes.values_mut().into_iter() {
        *v += 1;
    }
    let mut flashed = HashSet::new();
    loop {
        let flash_queue = nodes
            .flat_iter()
            .filter_map(|(c, v)| (v > 9 && !flashed.contains(&c)).then_some(c))
            .collect::<Vec<_>>();
        if flash_queue.is_empty() {
            break;
        }
        for flasher in flash_queue {
            flashed.insert(flasher);
            for (c, _) in nodes.neighbors(flasher, true) {
                *nodes.get_coord_mut(c).unwrap() += 1;
            }
        }
    }
    for &c in flashed.iter() {
        *nodes.get_coord_mut(c).unwrap() = 0;
    }
    flashed.len()
}
