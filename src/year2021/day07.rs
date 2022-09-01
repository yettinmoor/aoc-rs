use crate::common::run::Run;
use crate::vec_input;

pub struct Runner {}

fn solve<F>(input: &[usize], dist_fn: F) -> usize
where
    F: Fn(usize) -> usize,
{
    let (&min, &max) = (input.iter().min().unwrap(), input.iter().max().unwrap());
    (min..max)
        .map(|target| {
            input
                .iter()
                .map(|x| dist_fn(x.abs_diff(target)))
                .sum::<usize>()
        })
        .min()
        .unwrap()
}

impl Run for Runner {
    vec_input!(usize, ',');
    type Output = usize;

    fn part1(&self, input: &Self::Input) -> Self::Output {
        solve(input, |x| x)
    }

    fn part2(&self, input: &Self::Input) -> Self::Output {
        solve(input, |x| x * (x + 1) / 2)
    }
}
