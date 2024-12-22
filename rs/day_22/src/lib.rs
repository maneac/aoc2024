use std::{collections::HashMap, fs::read_to_string, path::Path};

pub const PART_1: usize = 17_724_064_040;
pub const PART_2: usize = 1998;

#[must_use]
pub fn read_data(data_dir: &str) -> String {
    read_to_string(Path::new(data_dir).join("day_22.txt"))
        .unwrap()
        .trim()
        .to_owned()
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Input {
    secret_seeds: Vec<usize>,
}

impl Input {
    #[must_use]
    pub fn from_data(data: &str) -> Self {
        let secret_seeds = data
            .trim()
            .lines()
            .map(|line| line.parse().unwrap())
            .collect();

        Self { secret_seeds }
    }

    #[must_use]
    pub fn part_1(&self) -> usize {
        self.secret_seeds
            .iter()
            .map(|&secret_number| Monkey { secret_number }.nth(2000).unwrap())
            .sum()
    }

    #[must_use]
    pub fn part_2(&self) -> usize {
        let diffs = self
            .secret_seeds
            .iter()
            .fold(HashMap::new(), |mut acc, &secret_number| {
                let monkey = Monkey { secret_number };
                let prices = monkey.take(2000).map(|num| num % 10).collect::<Vec<_>>();

                let mut monkey_acc = HashMap::new();

                for window in prices.windows(5) {
                    let mut iter = window.windows(2).map(
                        #[expect(
                            clippy::indexing_slicing,
                            clippy::as_conversions,
                            clippy::cast_possible_wrap
                        )]
                        |chunk| (chunk[1] as isize).saturating_sub_unsigned(chunk[0]),
                    );
                    let sequence = core::array::from_fn::<isize, 4, _>(|_idx| iter.next().unwrap());

                    _ = monkey_acc.entry(sequence).or_insert(
                        #[expect(clippy::indexing_slicing)]
                        window[4],
                    );
                }

                for (sequence, price) in monkey_acc {
                    _ = acc
                        .entry(sequence)
                        .and_modify(|val: &mut usize| *val += price)
                        .or_insert(price);
                }
                acc
            });

        diffs.values().max().copied().unwrap()
    }
}

struct Monkey {
    secret_number: usize,
}

impl Iterator for Monkey {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let initial = self.secret_number;

        self.mix(self.secret_number << 6);
        self.prune();

        self.mix(self.secret_number >> 5);
        self.prune();

        self.mix(self.secret_number << 11);
        self.prune();

        Some(initial)
    }
}

impl Monkey {
    #[inline]
    fn mix(&mut self, result: usize) {
        self.secret_number ^= result;
    }

    #[inline]
    fn prune(&mut self) {
        self.secret_number &= 0b1111_1111_1111_1111_1111_1111;
    }
}

#[cfg(test)]
mod day_22_tests {
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

    mod monkey {
        use super::*;

        #[test]
        fn example() {
            let mut initial = Monkey { secret_number: 123 };
            for expected in [
                123, 15_887_950, 16_495_136, 527_345, 704_524, 1_553_684, 12_683_156, 11_100_544,
                12_249_484, 7_753_432, 5_908_254,
            ] {
                assert_eq!(expected, initial.next().unwrap());
            }
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
                expected: 37_327_623,
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
        fn example_2() {
            run(&Case {
                data: super::example_2().1,
                expected: 23,
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
            "1
10
100
2024",
            Input {
                secret_seeds: vec![1, 10, 100, 2024],
            },
        )
    }

    fn example_2() -> (&'static str, Input) {
        (
            "1
2
3
2024",
            Input {
                secret_seeds: vec![1, 2, 3, 2024],
            },
        )
    }
}
