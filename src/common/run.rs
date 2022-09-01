use std::{fmt::Debug, time::Instant};

pub trait Run {
    type Input;
    type Output: Debug + PartialEq;

    fn parse(&self, s: &str) -> Self::Input;
    fn part1(&self, input: &Self::Input) -> Self::Output;
    fn part2(&self, input: &Self::Input) -> Self::Output;

    fn solve(
        &self,
        year: &str,
        day: &str,
        expected1: Option<Self::Output>,
        expected2: Option<Self::Output>,
    ) {
        let input_file = format!("input/{}/{}.txt", year, day);
        let input = std::fs::read_to_string(&input_file)
            .unwrap_or_else(|_| panic!("could not open file {}", &input_file));
        let parsed = self.parse(&input);

        macro_rules! solve_part {
            ($part: ident, $expected: ident) => {
                let now = Instant::now();
                let ans = self.$part(&parsed);
                let t = now.elapsed().as_secs_f64();
                println!(
                    "{}/{}/{} : [{:.3}s] : {:20} {}",
                    year.strip_prefix("year").unwrap(),
                    day.strip_prefix("day").unwrap(),
                    stringify!($part).strip_prefix("part").unwrap(),
                    t,
                    format!("{:?}", ans),
                    if let Some(expected) = $expected {
                        if expected == ans {
                            "✓".to_string()
                        } else {
                            format!("✗ : {:?}", expected)
                        }
                    } else {
                        "as".to_string()
                    }
                );
            };
        }

        solve_part!(part1, expected1);
        solve_part!(part2, expected2);
    }
}

#[macro_export]
macro_rules! vec_input {
    ($Input: ty) => {
        type Input = Vec<$Input>;
        fn parse(&self, s: &str) -> Self::Input {
            use std::str::FromStr;
            s.lines()
                .map(|line| <$Input>::from_str(line).unwrap())
                .collect()
        }
    };

    ($Input: ty, $sep: literal) => {
        type Input = Vec<$Input>;
        fn parse(&self, s: &str) -> Self::Input {
            use std::str::FromStr;
            s.trim()
                .split($sep)
                .map(|line| <$Input>::from_str(line).unwrap())
                .collect()
        }
    };
}

#[macro_export]
macro_rules! matrix_input {
    ($Input: ty) => {
        type Input = Vec<Vec<$Input>>;
        fn parse(&self, s: &str) -> Self::Input {
            s.lines()
                .map(|line| {
                    line.chars()
                        .map(|c| c.to_digit(10).unwrap() as usize)
                        .collect()
                })
                .collect()
        }
    };
}

#[macro_export]
macro_rules! solve {
    ($year: ident, $day: ident) => {{
        use common::run::Run;
        $year::$day::Runner {}.solve(
            stringify!($year),
            &format!("{:02}", stringify!($day)),
            None,
            None,
        );
    }};

    ($year: ident, $day: ident, $ans1: literal, $ans2: literal) => {{
        use common::run::Run;
        $year::$day::Runner {}.solve(
            stringify!($year),
            &format!("{:02}", stringify!($day)),
            (stringify!($ans1) != "None").then(|| $ans1),
            (stringify!($ans2) != "None").then(|| $ans2),
        );
    }};
}
