use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use crate::common::reverse::Reverse;
use crate::common::run::Run;
use crate::vec_input;

pub struct Runner {}

impl Run for Runner {
    vec_input!(Display);
    type Output = usize;

    fn part1(&self, input: &Self::Input) -> Self::Output {
        input
            .iter()
            .map(|display| {
                display
                    .solve()
                    .iter()
                    .filter(|digit| [1, 4, 7, 8].contains(digit))
                    .count()
            })
            .sum()
    }

    fn part2(&self, input: &Self::Input) -> Self::Output {
        input
            .iter()
            .map(|display| {
                display
                    .solve()
                    .iter()
                    .fold(0usize, |acc, &cur| acc * 10 + cur as usize)
            })
            .sum()
    }
}

type Digit = HashSet<u8>;

pub struct Display {
    digits: Vec<Digit>,
    displayed: Vec<Digit>,
}

impl Display {
    fn solve(&self) -> Vec<u8> {
        let mut map = HashMap::<u8, u8>::new();

        // 1. unique freqs -> B(6), E(4), F(9).
        let freqs = b"abcdefg"
            .iter()
            .copied()
            .map(|seg| {
                let freq = self.digits.iter().filter(|&d| d.contains(&seg)).count();
                (freq, seg)
            })
            .collect::<HashMap<_, _>>();

        map.insert(b'b', freqs[&6]);
        map.insert(b'e', freqs[&4]);
        map.insert(b'f', freqs[&9]);

        let mut do_digit = |seg: u8, found: &[u8]| {
            let digit = self
                .digits
                .iter()
                .find(|d| d.len() == found.len() + 1)
                .unwrap();
            let unmapped_segs = digit
                .iter()
                .filter(|&&seg| !map.reverse().contains_key(&seg))
                .collect::<Vec<_>>();
            assert_eq!(unmapped_segs.len(), 1);
            map.insert(seg, *unmapped_segs[0]);
        };

        // 2. 1 (CF) -> C.
        do_digit(b'c', b"f");

        // 3. 7 (ACF) -> A.
        do_digit(b'a', b"cf");

        // 4. 4 (BCDF) -> D.
        do_digit(b'd', b"bcf");

        // 5. 8 (ABCDEFG) -> G.
        do_digit(b'g', b"abcdef");

        let map = map.reverse();

        self.displayed
            .iter()
            .map(|displayed| canonical(displayed, &map))
            .collect()
    }
}

fn canonical(digit: &Digit, map: &HashMap<u8, u8>) -> u8 {
    let mut s = digit.iter().map(|c| map[c]).collect::<Vec<_>>();
    s.sort();
    let s = std::str::from_utf8(&s).unwrap();
    HashMap::from([
        ("abcefg", 0),
        ("cf", 1),
        ("acdeg", 2),
        ("acdfg", 3),
        ("bcdf", 4),
        ("abdfg", 5),
        ("abdefg", 6),
        ("acf", 7),
        ("abcdefg", 8),
        ("abcdfg", 9),
    ])[&s]
}

impl FromStr for Display {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split('|');
        let digits = it
            .next()
            .unwrap()
            .split_whitespace()
            .map(|digit| digit.bytes().collect())
            .collect();
        let displayed = it
            .next()
            .unwrap()
            .split_whitespace()
            .map(|digit| digit.bytes().collect())
            .collect();
        Ok(Display { digits, displayed })
    }
}

// 0: ABCEFG  6
// 1: CF      2*
// 2: ACDEG   5
// 3: ACDFG   5
// 4: BCDF    4*
// 5: ABDFG   5
// 6: ABDEFG  6
// 7: ACF     3*
// 8: ABCDEFG 7*
// 9: ABCDFG  6

// A: 02356789  8
// B: 045689    6*
// C: 01234789  8
// D: 2345689   7
// E: 0268      4*
// F: 013456789 9*
// G: 0235689   7
