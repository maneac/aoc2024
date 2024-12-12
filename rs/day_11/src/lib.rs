use std::collections::HashMap;
use std::{fs::read_to_string, path::Path};

pub const PART_1: usize = 175_006;
pub const PART_2: usize = 207_961_583_799_296;

#[must_use]
pub fn read_data(data_dir: &str) -> String {
    read_to_string(Path::new(data_dir).join("day_11.txt"))
        .unwrap()
        .trim()
        .to_owned()
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Input {
    stones: Vec<usize>,
}

impl Input {
    #[must_use]
    pub fn from_data(data: &str) -> Self {
        let stones = data
            .trim()
            .split_ascii_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();

        Self { stones }
    }

    #[must_use]
    pub fn part_1(&self) -> usize {
        self.blink::<25>()
    }

    #[must_use]
    pub fn part_2(&self) -> usize {
        self.blink::<75>()
    }

    fn blink<const COUNT: u8>(&self) -> usize {
        (0..COUNT)
            .fold(
                (
                    self.stones
                        .iter()
                        .copied()
                        .map(|stone| (stone, 1_usize))
                        .collect::<HashMap<_, _>>(),
                    HashMap::with_capacity(self.stones.len()),
                ),
                |(mut acc, mut new), _| {
                    for (stone, count) in acc.drain() {
                        if stone == 0 {
                            _ = new
                                .entry(1)
                                .and_modify(|val| *val += count)
                                .or_insert(count);
                            continue;
                        }

                        #[expect(
                            clippy::as_conversions,
                            clippy::cast_sign_loss,
                            clippy::cast_precision_loss,
                            clippy::cast_possible_truncation
                        )]
                        let num_digits = (stone as f64).log10() as u32 + 1;
                        if num_digits & 1 > 0 {
                            _ = new
                                .entry(stone * 2024)
                                .and_modify(|val| *val += count)
                                .or_insert(count);
                        } else {
                            let divisor = 10_usize.pow(num_digits >> 1);

                            let lhs = stone.div_euclid(divisor);
                            let rhs = stone % divisor;

                            _ = new
                                .entry(lhs)
                                .and_modify(|val| *val += count)
                                .or_insert(count);

                            _ = new
                                .entry(rhs)
                                .and_modify(|val| *val += count)
                                .or_insert(count);
                        }
                    }
                    (new, acc)
                },
            )
            .0
            .values()
            .sum()
    }
}

#[cfg(test)]
mod day_11_tests {
    use super::*;

    const DATA_DIR: &str = "../../data";

    mod from_data {
        use super::*;

        struct Case<'c> {
            input: &'c str,
            expected: Input,
        }

        #[test]
        fn example_1() {
            run(&Case {
                input: super::example_1().0,
                expected: super::example_1().1,
            });
        }

        #[test]
        fn example_2() {
            run(&Case {
                input: super::example_2().0,
                expected: super::example_2().1,
            });
        }

        fn run(test: &Case<'_>) {
            assert_eq!(test.expected, Input::from_data(test.input));
        }
    }

    mod blink {
        use super::*;

        struct Case {
            data: Input,
            expected: usize,
        }

        #[test]
        fn example_1() {
            run::<1>(&Case {
                data: super::example_1().1,
                expected: 7,
            });
        }

        #[test]
        fn example_2_short() {
            run::<6>(&Case {
                data: example_2().1,
                expected: 22,
            });
        }

        fn run<const COUNT: u8>(test: &Case) {
            assert_eq!(test.expected, test.data.blink::<COUNT>());
        }
    }

    mod part_1 {
        use super::*;

        struct Case {
            data: Input,
            expected: usize,
        }

        #[test]
        fn example_2_full() {
            run(&Case {
                data: example_2().1,
                expected: 55312,
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

    fn example_1() -> (&'static str, Input) {
        (
            "0 1 10 99 999",
            Input {
                stones: vec![0, 1, 10, 99, 999],
            },
        )
    }

    fn example_2() -> (&'static str, Input) {
        (
            "125 17",
            Input {
                stones: vec![125, 17],
            },
        )
    }
}
