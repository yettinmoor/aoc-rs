use std::ops::Not;
use std::str::FromStr;

use crate::common::matrix::Matrix;
use crate::common::run::Run;

pub struct Runner {}

impl Run for Runner {
    type Input = Bingo;
    type Output = usize;

    fn parse(&self, s: &str) -> Self::Input {
        let mut it = s.split("\n\n");
        let draws = it
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();

        let boards = it.map(|board| Board::from_str(board).unwrap()).collect();

        Bingo {
            draws,
            boards,
            wins: vec![],
        }
    }

    fn part1(&self, input: &Self::Input) -> Self::Output {
        let mut bingo = input.clone();
        bingo.play_all();
        bingo.boards[bingo.wins[0]].score()
    }

    fn part2(&self, input: &Self::Input) -> Self::Output {
        let mut bingo = input.clone();
        bingo.play_all();
        bingo.boards[*bingo.wins.last().unwrap()].score()
    }
}

#[derive(Clone)]
pub struct Bingo {
    draws: Vec<usize>,
    boards: Vec<Board>,
    wins: Vec<usize>,
}

#[derive(Clone)]
pub struct Board {
    grid: Vec<Vec<usize>>,
    marked: Vec<Vec<bool>>,
    last_draw: usize,
}

impl Bingo {
    fn play_all(&mut self) {
        for &draw in self.draws.iter() {
            for (i, board) in self.boards.iter_mut().enumerate() {
                if !self.wins.contains(&i) {
                    board.play(draw);
                    if board.bingo() {
                        self.wins.push(i);
                    }
                }
            }
            if self.wins.len() == self.boards.len() {
                return;
            }
        }
    }
}

impl Board {
    fn bingo(&self) -> bool {
        let check = |it: &[Vec<bool>]| it.iter().any(|row| row.iter().all(|&m| m));
        check(&self.marked) || check(&self.marked.transpose())
    }

    fn play(&mut self, draw: usize) {
        for (i, row) in self.grid.iter().enumerate() {
            for (j, &num) in row.iter().enumerate() {
                if num == draw {
                    self.last_draw = draw;
                    self.marked[i][j] = true;
                    return;
                }
            }
        }
    }

    fn score(&self) -> usize {
        self.grid
            .iter()
            .flatten()
            .zip(self.marked.iter().flatten())
            .filter_map(|(num, marked)| marked.not().then(|| num))
            .sum::<usize>()
            * self.last_draw
    }
}

impl FromStr for Board {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect()
            })
            .collect();
        Ok(Board {
            grid,
            marked: vec![vec![false; 5]; 5],
            last_draw: 0,
        })
    }
}
