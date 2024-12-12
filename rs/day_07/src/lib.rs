use std::{fs::read_to_string, path::Path};

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

pub const PART_1: usize = 663_613_490_587;
pub const PART_2: usize = 110_365_987_435_001;

#[must_use]
pub fn read_data(data_dir: &str) -> String {
    read_to_string(Path::new(data_dir).join("day_07.txt"))
        .unwrap()
        .trim()
        .to_owned()
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Input {
    equations: Vec<(usize, Vec<usize>)>,
}

impl Input {
    #[must_use]
    pub fn from_data(data: &str) -> Self {
        let equations = data
            .trim()
            .lines()
            .map(|line| {
                let (total, parts) = line.split_once(": ").unwrap();
                (
                    total.parse().unwrap(),
                    parts
                        .trim()
                        .split_ascii_whitespace()
                        .map(|part| part.parse().unwrap())
                        .collect(),
                )
            })
            .collect::<Vec<_>>();

        Self { equations }
    }

    #[must_use]
    pub fn part_1(&self) -> usize {
        self.equations
            .iter()
            .filter_map(|(target_ref, parts)| {
                let target = *target_ref;
                for bitset in 0..2_usize.pow(u32::try_from(parts.len()).unwrap()) {
                    let total = parts.iter().enumerate().fold(0_usize, |acc, (idx, part)| {
                        if bitset & 1 << idx > 0 {
                            acc * part
                        } else {
                            acc + part
                        }
                    });
                    if total == target {
                        return Some(target);
                    }
                }

                None
            })
            .sum()
    }

    #[must_use]
    pub fn part_2(&self) -> usize {
        self.equations
            .par_iter()
            .filter_map(|(target_ref, parts)| {
                let target = *target_ref;

                for permutation in 0..3_usize.pow(u32::try_from(parts.len()).unwrap()) {
                    let total = parts
                        .iter()
                        .copied()
                        .fold((0_usize, permutation), |(acc, rem), part| {
                            let n = rem % 3;
                            let acc = match n {
                                0 => acc + part,
                                1 => acc * part,
                                2 => {
                                    #[expect(
                                        clippy::cast_precision_loss,
                                        clippy::cast_possible_truncation,
                                        clippy::cast_sign_loss,
                                        clippy::as_conversions
                                    )]
                                    let digits = (part as f64).log10() as u32 + 1;
                                    (acc * 10_usize.pow(digits)) + part
                                }
                                #[expect(clippy::unreachable)]
                                _ => {
                                    unreachable!()
                                }
                            };
                            (acc, rem.div_euclid(3))
                        })
                        .0;
                    if total == target {
                        return Some(target);
                    }
                }

                None
            })
            .sum()
    }
}

#[cfg(test)]
mod day_07_tests {
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
                expected: 3749,
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
                expected: 11_387,
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
            "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20",
            Input {
                equations: vec![
                    (190, vec![10, 19]),
                    (3267, vec![81, 40, 27]),
                    (83, vec![17, 5]),
                    (156, vec![15, 6]),
                    (7290, vec![6, 8, 6, 15]),
                    (161_011, vec![16, 10, 13]),
                    (192, vec![17, 8, 14]),
                    (21037, vec![9, 7, 18, 13]),
                    (292, vec![11, 6, 16, 20]),
                ],
            },
        )
    }
}
