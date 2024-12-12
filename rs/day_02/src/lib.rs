use std::{fs::read_to_string, path::Path};

pub const PART_1: usize = 686;
pub const PART_2: usize = 717;

#[must_use]
pub fn read_data(data_dir: &str) -> String {
    read_to_string(Path::new(data_dir).join("day_02.txt"))
        .unwrap()
        .trim()
        .to_owned()
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Input {
    list: Vec<Vec<u8>>,
}

impl Input {
    #[must_use]
    pub fn from_data(data: &str) -> Self {
        let list = data
            .trim()
            .lines()
            .map(|line| {
                line.trim()
                    .split_ascii_whitespace()
                    .map(|entry| entry.parse::<u8>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Self { list }
    }

    #[must_use]
    #[allow(clippy::indexing_slicing)]
    pub fn part_1(&self) -> usize {
        self.list
            .iter()
            .filter(|levels| {
                (levels.windows(2).all(|win| win[0] <= win[1])
                    || levels.windows(2).all(|win| win[0] >= win[1]))
                    && levels
                        .windows(2)
                        .all(|win| (1..=3).contains(&win[0].abs_diff(win[1])))
            })
            .count()
    }

    #[must_use]
    #[allow(clippy::indexing_slicing)]
    pub fn part_2(&self) -> usize {
        self.list
            .iter()
            .filter(|&levels| {
                (levels.windows(2).all(|win| win[0] <= win[1])
                    || levels.windows(2).all(|win| win[0] >= win[1]))
                    && levels
                        .windows(2)
                        .all(|win| (1..=3).contains(&win[0].abs_diff(win[1])))
                    || (0..levels.len()).any(|skip| {
                        let mut level = levels.clone();
                        _ = level.remove(skip);

                        (level.windows(2).all(|win| win[0] <= win[1])
                            || level.windows(2).all(|win| win[0] >= win[1]))
                            && level
                                .windows(2)
                                .all(|win| (1..=3).contains(&win[0].abs_diff(win[1])))
                    })
            })
            .count()
    }
}

#[cfg(test)]
mod day_02_tests {
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
            });
        }

        fn run(test: &Case<'_>) {
            assert_eq!(test.expected, Input::from_data(test.input));
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
                expected: 2,
            });
        }

        #[test]
        fn actual() {
            run(&Case {
                data: Input::from_data(&read_data(DATA_DIR)),
                expected: PART_1,
            });
        }

        fn run(test: &Case) {
            assert_eq!(test.expected, test.data.part_1());
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
                expected: 4,
            });
        }

        #[test]
        fn actual() {
            run(&Case {
                data: Input::from_data(&read_data(DATA_DIR)),
                expected: PART_2,
            });
        }

        fn run(test: &Case) {
            assert_eq!(test.expected, test.data.part_2());
        }
    }

    fn example() -> (&'static str, Input) {
        (
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
            Input {
                list: vec![
                    vec![7, 6, 4, 2, 1],
                    vec![1, 2, 7, 8, 9],
                    vec![9, 7, 6, 2, 1],
                    vec![1, 3, 2, 4, 5],
                    vec![8, 6, 4, 4, 1],
                    vec![1, 3, 6, 7, 9],
                ],
            },
        )
    }
}
