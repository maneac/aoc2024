use std::collections::{BTreeSet, HashSet};
use std::{fs::read_to_string, path::Path};

pub const PART_1: usize = 698;
pub const PART_2: usize = 1436;

#[must_use]
pub fn read_data(data_dir: &str) -> String {
    read_to_string(Path::new(data_dir).join("day_10.txt"))
        .unwrap()
        .trim()
        .to_owned()
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Input {
    map: Vec<Vec<u8>>,
}

impl Input {
    #[must_use]
    pub fn from_data(data: &str) -> Self {
        let map = data
            .trim()
            .lines()
            .map(|line| {
                line.bytes()
                    .map(|byte| if byte == b'.' { 10 } else { byte - b'0' })
                    .collect()
            })
            .collect();

        Self { map }
    }

    #[must_use]
    pub fn part_1(&self) -> usize {
        self.map
            .iter()
            .enumerate()
            .flat_map(|(y, vals)| {
                vals.iter()
                    .copied()
                    .enumerate()
                    .map(move |(x, val)| (y, x, val))
            })
            .filter_map(|(y, x, val)| (val == 0).then_some((y, x)))
            .map(|trailhead| {
                let mut found_ends = HashSet::new();
                let mut working_set = BTreeSet::from([(trailhead.0, trailhead.1, 0_u8)]);
                while let Some((y, x, val)) = working_set.pop_first() {
                    if val == 9 {
                        _ = found_ends.insert((y, x));
                        continue;
                    }

                    for new in [
                        x.checked_add(1).map(|new_x| (y, new_x)),
                        y.checked_add(1).map(|new_y| (new_y, x)),
                        x.checked_sub(1).map(|new_x| (y, new_x)),
                        y.checked_sub(1).map(|new_y| (new_y, x)),
                    ]
                    .into_iter()
                    .flatten()
                    .filter_map(|(new_y, new_x)| {
                        self.map
                            .get(new_y)
                            .and_then(|ys| ys.get(new_x))
                            .and_then(|new_val| {
                                (*new_val == val + 1).then_some((new_y, new_x, *new_val))
                            })
                    }) {
                        _ = working_set.insert(new);
                    }
                }
                found_ends.len()
            })
            .sum()
    }

    #[must_use]
    pub fn part_2(&self) -> usize {
        self.map
            .iter()
            .enumerate()
            .flat_map(|(y, vals)| {
                vals.iter()
                    .copied()
                    .enumerate()
                    .map(move |(x, val)| (y, x, val))
            })
            .filter_map(|(y, x, val)| (val == 0).then_some((y, x)))
            .map(|trailhead| {
                let mut found_trails = 0;
                let mut working_set = BTreeSet::from([(vec![trailhead], 0_u8)]);
                while let Some((trail, val)) = working_set.pop_first() {
                    if val == 9 {
                        found_trails += 1;
                        continue;
                    }
                    let (y, x) = trail.last().copied().unwrap();

                    for (new_y, new_x, new_val) in [
                        x.checked_add(1).map(|new_x| (y, new_x)),
                        y.checked_add(1).map(|new_y| (new_y, x)),
                        x.checked_sub(1).map(|new_x| (y, new_x)),
                        y.checked_sub(1).map(|new_y| (new_y, x)),
                    ]
                    .into_iter()
                    .flatten()
                    .filter_map(|(new_y, new_x)| {
                        self.map
                            .get(new_y)
                            .and_then(|ys| ys.get(new_x))
                            .and_then(|new_val| {
                                (*new_val == val + 1).then_some((new_y, new_x, *new_val))
                            })
                    }) {
                        let mut new_trail = trail.clone();
                        new_trail.push((new_y, new_x));
                        _ = working_set.insert((new_trail, new_val));
                    }
                }
                found_trails
            })
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

        #[test]
        fn example_3() {
            run(&Case {
                input: super::example_3().0,
                expected: super::example_3().1,
            });
        }

        #[test]
        fn example_4() {
            run(&Case {
                input: super::example_4().0,
                expected: super::example_4().1,
            });
        }

        #[test]
        fn example_5() {
            run(&Case {
                input: super::example_5().0,
                expected: super::example_5().1,
            });
        }

        #[test]
        fn example_6() {
            run(&Case {
                input: super::example_6().0,
                expected: super::example_6().1,
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
        fn example_1() {
            run(&Case {
                data: super::example_1().1,
                expected: 1,
            });
        }

        #[test]
        fn example_2() {
            run(&Case {
                data: super::example_2().1,
                expected: 2,
            });
        }

        #[test]
        fn example_3() {
            run(&Case {
                data: super::example_3().1,
                expected: 4,
            });
        }

        #[test]
        fn example_4() {
            run(&Case {
                data: super::example_4().1,
                expected: 3,
            });
        }

        #[test]
        fn larger_example() {
            run(&Case {
                data: super::larger_example().1,
                expected: 36,
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
        fn example_5() {
            run(&Case {
                data: super::example_5().1,
                expected: 3,
            });
        }

        #[test]
        fn example_3() {
            run(&Case {
                data: super::example_3().1,
                expected: 13,
            });
        }

        #[test]
        fn example_6() {
            run(&Case {
                data: super::example_6().1,
                expected: 227,
            });
        }

        #[test]
        fn larger_example() {
            run(&Case {
                data: super::larger_example().1,
                expected: 81,
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

    fn example_1() -> (&'static str, Input) {
        (
            "0123
1234
8765
9876",
            Input {
                map: vec![
                    vec![0, 1, 2, 3],
                    vec![1, 2, 3, 4],
                    vec![8, 7, 6, 5],
                    vec![9, 8, 7, 6],
                ],
            },
        )
    }

    fn example_2() -> (&'static str, Input) {
        (
            "...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9",
            Input {
                map: vec![
                    vec![10, 10, 10, 0, 10, 10, 10],
                    vec![10, 10, 10, 1, 10, 10, 10],
                    vec![10, 10, 10, 2, 10, 10, 10],
                    vec![6, 5, 4, 3, 4, 5, 6],
                    vec![7, 10, 10, 10, 10, 10, 7],
                    vec![8, 10, 10, 10, 10, 10, 8],
                    vec![9, 10, 10, 10, 10, 10, 9],
                ],
            },
        )
    }

    fn example_3() -> (&'static str, Input) {
        (
            "..90..9
...1.98
...2..7
6543456
765.987
876....
987....",
            Input {
                map: vec![
                    vec![10, 10, 9, 0, 10, 10, 9],
                    vec![10, 10, 10, 1, 10, 9, 8],
                    vec![10, 10, 10, 2, 10, 10, 7],
                    vec![6, 5, 4, 3, 4, 5, 6],
                    vec![7, 6, 5, 10, 9, 8, 7],
                    vec![8, 7, 6, 10, 10, 10, 10],
                    vec![9, 8, 7, 10, 10, 10, 10],
                ],
            },
        )
    }

    fn example_4() -> (&'static str, Input) {
        (
            "10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01",
            Input {
                map: vec![
                    vec![1, 0, 10, 10, 9, 10, 10],
                    vec![2, 10, 10, 10, 8, 10, 10],
                    vec![3, 10, 10, 10, 7, 10, 10],
                    vec![4, 5, 6, 7, 6, 5, 4],
                    vec![10, 10, 10, 8, 10, 10, 3],
                    vec![10, 10, 10, 9, 10, 10, 2],
                    vec![10, 10, 10, 10, 10, 0, 1],
                ],
            },
        )
    }

    fn example_5() -> (&'static str, Input) {
        (
            ".....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9....",
            Input {
                map: vec![
                    vec![10, 10, 10, 10, 10, 0, 10],
                    vec![10, 10, 4, 3, 2, 1, 10],
                    vec![10, 10, 5, 10, 10, 2, 10],
                    vec![10, 10, 6, 5, 4, 3, 10],
                    vec![10, 10, 7, 10, 10, 4, 10],
                    vec![10, 10, 8, 7, 6, 5, 10],
                    vec![10, 10, 9, 10, 10, 10, 10],
                ],
            },
        )
    }

    fn example_6() -> (&'static str, Input) {
        (
            "012345
123456
234567
345678
4.6789
56789.",
            Input {
                map: vec![
                    vec![0, 1, 2, 3, 4, 5],
                    vec![1, 2, 3, 4, 5, 6],
                    vec![2, 3, 4, 5, 6, 7],
                    vec![3, 4, 5, 6, 7, 8],
                    vec![4, 10, 6, 7, 8, 9],
                    vec![5, 6, 7, 8, 9, 10],
                ],
            },
        )
    }

    fn larger_example() -> (&'static str, Input) {
        (
            "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
            Input {
                map: vec![
                    vec![8, 9, 0, 1, 0, 1, 2, 3],
                    vec![7, 8, 1, 2, 1, 8, 7, 4],
                    vec![8, 7, 4, 3, 0, 9, 6, 5],
                    vec![9, 6, 5, 4, 9, 8, 7, 4],
                    vec![4, 5, 6, 7, 8, 9, 0, 3],
                    vec![3, 2, 0, 1, 9, 0, 1, 2],
                    vec![0, 1, 3, 2, 9, 8, 0, 1],
                    vec![1, 0, 4, 5, 6, 7, 3, 2],
                ],
            },
        )
    }
}
