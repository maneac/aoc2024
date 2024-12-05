use std::{fs::read_to_string, path::Path};

pub const PART_1: usize = 2264607;
pub const PART_2: usize = 19457120;

pub fn read_data(data_dir: &str) -> String {
    read_to_string(Path::new(data_dir).join("day_01.txt"))
        .unwrap()
        .trim()
        .to_string()
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Input {
    lhs: Vec<usize>,
    rhs: Vec<usize>,
}

impl Input {
    pub fn from_data(data: &str) -> Self {
        let (mut lhs, mut rhs): (Vec<_>, Vec<_>) = data
            .trim()
            .lines()
            .map(|line| {
                let (lhs, rhs) = line.trim().split_once("   ").unwrap();
                (lhs.parse::<usize>().unwrap(), rhs.parse::<usize>().unwrap())
            })
            .unzip();

        lhs.sort_unstable();
        rhs.sort_unstable();

        Self { lhs, rhs }
    }

    pub fn part_1(&self) -> usize {
        self.lhs
            .iter()
            .zip(&self.rhs)
            .map(|(left, right)| left.abs_diff(*right))
            .sum()
    }

    pub fn part_2(&self) -> usize {
        self.lhs
            .iter()
            .map(|left| left * self.rhs.iter().filter(|right| left.eq(right)).count())
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA_DIR: &str = "../../data";

    mod from_data {
        use super::*;

        struct Case<'c> {
            input: &'c str,
            expected: Input,
        }

        #[test]
        fn example() {
            run(&Case {
                input: super::example().0,
                expected: super::example().1,
            })
        }

        fn run(test: &Case) {
            assert_eq!(test.expected, Input::from_data(test.input))
        }
    }

    mod part_1 {
        use super::*;

        struct Case {
            data: Input,
            expected: usize,
        }

        #[test]
        fn example() {
            run(&Case {
                data: super::example().1,
                expected: 11,
            })
        }

        #[test]
        fn actual() {
            run(&Case {
                data: Input::from_data(&read_data(DATA_DIR)),
                expected: PART_1,
            })
        }

        fn run(test: &Case) {
            assert_eq!(test.expected, test.data.part_1())
        }
    }

    mod part_2 {
        use super::*;

        struct Case {
            data: Input,
            expected: usize,
        }

        #[test]
        fn example() {
            run(&Case {
                data: super::example().1,
                expected: 31,
            })
        }

        #[test]
        fn actual() {
            run(&Case {
                data: Input::from_data(&read_data(DATA_DIR)),
                expected: PART_2,
            })
        }

        fn run(test: &Case) {
            assert_eq!(test.expected, test.data.part_2())
        }
    }

    fn example() -> (&'static str, Input) {
        (
            "3   4
4   3
2   5
1   3
3   9
3   3",
            Input {
                lhs: vec![1, 2, 3, 3, 3, 4],
                rhs: vec![3, 3, 3, 4, 5, 9],
            },
        )
    }
}
