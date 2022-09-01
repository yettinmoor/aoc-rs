use crate::common::matrix::Matrix;
use crate::common::run::Run;

pub struct Runner {}

impl Run for Runner {
    type Input = Vec<Vec<usize>>;
    type Output = usize;

    fn parse(&self, s: &str) -> Self::Input {
        s.lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '0' => 0,
                        '1' => 1,
                        _ => panic!("unexpected char `{}`", c),
                    })
                    .collect()
            })
            .collect()
    }

    fn part1(&self, input: &Self::Input) -> Self::Output {
        let gamma = input
            .transpose()
            .iter()
            .map(|col| most_common(col))
            .fold(0, |acc, cur| acc << 1 | cur);
        let epsilon = !gamma & ((1 << input[0].len()) - 1);
        gamma * epsilon
    }

    fn part2(&self, input: &Self::Input) -> Self::Output {
        let mut oxy: Vec<Vec<usize>> = input.clone();
        let mut co2: Vec<Vec<usize>> = input.clone();
        for j in 0..input[0].len() {
            if oxy.len() > 1 {
                let most_common_jth_bit = most_common(&oxy.transpose()[j]);
                oxy = oxy
                    .into_iter()
                    .filter(|row| row[j] == most_common_jth_bit)
                    .collect();
            }
            if co2.len() > 1 {
                let most_common_jth_bit = most_common(&co2.transpose()[j]);
                co2 = co2
                    .into_iter()
                    .filter(|row| row[j] != most_common_jth_bit)
                    .collect();
            }
            if oxy.len() == 1 && co2.len() == 1 {
                break;
            }
        }
        bits_to_usize(&oxy[0]) * bits_to_usize(&co2[0])
    }
}

fn bits_to_usize(xs: &[usize]) -> usize {
    xs.iter().fold(0, |acc, &cur| acc << 1 | cur)
}

fn most_common(xs: &[usize]) -> usize {
    (xs.iter().filter(|&&b| b == 1).count() >= (xs.len() + 1) / 2).into()
}
