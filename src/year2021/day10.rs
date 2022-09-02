use crate::common::run::Run;
use crate::vec_input;

pub struct Runner {}

fn closer(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => unreachable!(),
    }
}

impl Run for Runner {
    vec_input!(String);
    type Output = usize;

    fn part1(&self, input: &Self::Input) -> Self::Output {
        input
            .iter()
            .map(|line| {
                let mut stack = vec![];
                line.chars()
                    .find_map(|c| match c {
                        '(' | '[' | '{' | '<' => {
                            stack.push(c);
                            None
                        }
                        ')' | ']' | '}' | '>' => {
                            if let Some(last) = stack.pop() {
                                (c != closer(last)).then_some(c)
                            } else {
                                Some(c)
                            }
                        }
                        _ => unreachable!(),
                    })
                    .map(|c| match c {
                        ')' => 3,
                        ']' => 57,
                        '}' => 1197,
                        '>' => 25137,
                        _ => unreachable!(),
                    })
                    .unwrap_or(0)
            })
            .sum::<usize>()
    }

    fn part2(&self, input: &Self::Input) -> Self::Output {
        let mut scores = input
            .iter()
            .filter_map(|line| {
                let mut stack = vec![];
                for c in line.chars() {
                    match c {
                        '(' | '[' | '{' | '<' => {
                            stack.push(c);
                        }
                        ')' | ']' | '}' | '>' => {
                            if c != closer(stack.pop()?) {
                                return None;
                            }
                        }
                        _ => unreachable!(),
                    }
                }
                let score = stack.iter().rev().fold(0, |acc, &cur| {
                    acc * 5 + " )]}>".chars().position(|c| c == closer(cur)).unwrap()
                });
                Some(score)
            })
            .collect::<Vec<_>>();
        scores.sort();
        scores[scores.len() / 2]
    }
}
