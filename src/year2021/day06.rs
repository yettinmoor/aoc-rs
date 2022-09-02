use std::iter::{once, successors};

use crate::common::counter::{Countable, CounterSum};
use crate::common::run::Run;
use crate::vec_input;

pub struct Runner {}

impl Run for Runner {
    vec_input!(usize, ',');
    type Output = usize;

    fn part1(&self, input: &Self::Input) -> Self::Output {
        const N: usize = 80;
        solve(input, N)
    }

    fn part2(&self, input: &Self::Input) -> Self::Output {
        const N: usize = 256;
        solve(input, N)
    }
}

fn solve(xs: &[usize], n: usize) -> usize {
    let xs = xs.iter().copied().counter();
    successors(Some(xs), |xs| {
        let zeroes = *xs.get(&0).unwrap_or(&0);
        Some(
            xs.iter()
                .filter_map(|(&k, &v)| (k != 0).then_some((k - 1, v)))
                .chain(once((6, zeroes)))
                .chain(once((8, zeroes)))
                .counter_sum(),
        )
    })
    .nth(n)
    .unwrap()
    .values()
    .sum()
}

#[test]
fn test() {
    let input = vec![3, 4, 3, 1, 2];
    let ans = solve(&input, 18);
    dbg!(ans);
}
