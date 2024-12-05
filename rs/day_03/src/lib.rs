use std::{fs::read_to_string, path::Path};

pub const PART_1: usize = 182619815;
pub const PART_2: usize = 80747545;

pub fn read_data(data_dir: &str) -> String {
    read_to_string(Path::new(data_dir).join("day_03.txt"))
        .unwrap()
        .trim()
        .to_string()
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Input<'s> {
    input: &'s str,
}

impl<'s> Input<'s> {
    pub fn from_data(data: &'s str) -> Self {
        Self { input: data }
    }

    pub fn part_1(&self) -> usize {
        regex::Regex::new(r"(mul\(\d{1,3},\d{1,3}\))")
            .unwrap()
            .captures_iter(self.input)
            .map(|cap| {
                let found = cap.iter().next().unwrap().unwrap().as_str();
                let (lhs, rhs) = found
                    .strip_prefix("mul(")
                    .unwrap()
                    .strip_suffix(")")
                    .unwrap()
                    .split_once(',')
                    .unwrap();
                lhs.parse::<usize>().unwrap() * rhs.parse::<usize>().unwrap()
            })
            .sum()
    }

    pub fn part_2(&self) -> usize {
        regex::Regex::new(r"(mul\(\d{1,3},\d{1,3}\)|do\(\)|don\'t\(\))")
            .unwrap()
            .captures_iter(self.input)
            .fold((true, 0usize), |(enabled, acc), cap| {
                let found = cap.iter().next().unwrap().unwrap().as_str();
                match found {
                    "do()" => (true, acc),
                    "don't()" => (false, acc),
                    _ if !enabled => (enabled, acc),
                    _ => {
                        let (lhs, rhs) = found
                            .strip_prefix("mul(")
                            .unwrap()
                            .strip_suffix(")")
                            .unwrap()
                            .split_once(',')
                            .unwrap();
                        (
                            enabled,
                            acc + (lhs.parse::<usize>().unwrap() * rhs.parse::<usize>().unwrap()),
                        )
                    }
                }
            })
            .1
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
            expected: Input<'c>,
        }

        #[test]
        fn example_1() {
            run(&Case {
                input: super::example_1().0,
                expected: super::example_1().1,
            })
        }

        #[test]
        fn example_2() {
            run(&Case {
                input: super::example_2().0,
                expected: super::example_2().1,
            })
        }

        fn run(test: &Case) {
            assert_eq!(test.expected, Input::from_data(test.input))
        }
    }

    mod part_1 {
        use super::*;

        struct Case<'c> {
            data: Input<'c>,
            expected: usize,
        }

        #[test]
        fn example() {
            run(&Case {
                data: super::example_1().1,
                expected: 161,
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

        struct Case<'c> {
            data: Input<'c>,
            expected: usize,
        }

        #[test]
        fn example() {
            run(&Case {
                data: super::example_2().1,
                expected: 48,
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

    fn example_1() -> (&'static str, Input<'static>) {
        (
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
            Input {
                input: "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
            },
        )
    }

    fn example_2() -> (&'static str, Input<'static>) {
        (
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
            Input {
                input: "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
            },
        )
    }
}
