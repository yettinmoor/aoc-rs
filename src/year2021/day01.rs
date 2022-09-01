use crate::common::run::Run;
use crate::vec_input;

pub struct Runner {}

fn increases<T: PartialOrd>(input: &[T]) -> usize {
    input
        .iter()
        .zip(input.iter().skip(1))
        .filter(|(a, b)| a < b)
        .count()
}

impl Run for Runner {
    vec_input!(usize);

    type Output = usize;

    fn part1(&self, input: &Self::Input) -> Self::Output {
        increases(input)
    }

    fn part2(&self, input: &Self::Input) -> Self::Output {
        let sums = input
            .windows(3)
            .map(|w| w.iter().sum())
            .collect::<Vec<usize>>();
        increases(&sums)
    }
}
