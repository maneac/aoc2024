use std::collections::{BTreeSet, HashMap, HashSet};
use std::{fs::read_to_string, path::Path};

pub const PART_1: usize = 1_457_298;
pub const PART_2: usize = 921_636;

#[must_use]
pub fn read_data(data_dir: &str) -> String {
    read_to_string(Path::new(data_dir).join("day_12.txt"))
        .unwrap()
        .trim()
        .to_owned()
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Input {
    plants: HashMap<u8, Vec<[u8; 2]>>,
}

impl Input {
    #[must_use]
    pub fn from_data(data: &str) -> Self {
        let plants = data
            .trim()
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.bytes().enumerate().map(move |(x, plant)| {
                    (u8::try_from(y).unwrap(), u8::try_from(x).unwrap(), plant)
                })
            })
            .fold(HashMap::<u8, Vec<_>>::new(), |mut acc, (y, x, plant)| {
                _ = acc
                    .entry(plant)
                    .and_modify(|entry| entry.push([y, x]))
                    .or_insert_with(|| vec![[y, x]]);
                acc
            });

        Self { plants }
    }

    #[must_use]
    pub fn part_1(&self) -> usize {
        self.plants
            .values()
            .flat_map(|locations| {
                let mut to_visit = BTreeSet::<&[u8; 2]>::from_iter(locations);

                let mut plots = Vec::new();
                while let Some(&[start_y, start_x]) = to_visit.pop_first() {
                    let mut perimeter = 0;

                    let mut plot = HashSet::new();
                    let mut working_set = BTreeSet::from([[start_y, start_x]]);
                    while let Some([y, x]) = working_set.pop_first() {
                        _ = plot.insert([y, x]);
                        perimeter += 4;

                        for neighbour in [
                            y.checked_sub(1).map(|new_y| [new_y, x]),
                            y.checked_add(1).map(|new_y| [new_y, x]),
                            x.checked_sub(1).map(|new_x| [y, new_x]),
                            x.checked_add(1).map(|new_x| [y, new_x]),
                        ]
                        .into_iter()
                        .flatten()
                        {
                            if plot.contains(&neighbour) {
                                perimeter -= 2;
                            }

                            if to_visit.remove(&neighbour) {
                                _ = working_set.insert(neighbour);
                            }
                        }
                    }
                    let area = plot.len();

                    plots.push([area, perimeter]);
                }

                plots.into_iter().map(|[area, perimeter]| area * perimeter)
            })
            .sum()
    }

    #[must_use]
    pub fn part_2(&self) -> usize {
        self.plants
            .values()
            .flat_map(|locations| {
                let mut to_visit = BTreeSet::<&[u8; 2]>::from_iter(locations);

                let mut plots = Vec::new();
                while let Some(&[start_y, start_x]) = to_visit.pop_first() {
                    let mut plot = HashSet::new();
                    let mut working_set = BTreeSet::from([[start_y, start_x]]);
                    while let Some([y, x]) = working_set.pop_first() {
                        _ = plot.insert([y, x]);

                        for neighbour in [
                            y.checked_sub(1).map(|new_y| [new_y, x]),
                            y.checked_add(1).map(|new_y| [new_y, x]),
                            x.checked_sub(1).map(|new_x| [y, new_x]),
                            x.checked_add(1).map(|new_x| [y, new_x]),
                        ]
                        .into_iter()
                        .flatten()
                        {
                            if to_visit.remove(&neighbour) {
                                _ = working_set.insert(neighbour);
                            }
                        }
                    }
                    let area = plot.len();

                    let corners = plot
                        .iter()
                        .flat_map(|&[y, x]| {
                            [
                                // Top/Right
                                [
                                    y.checked_sub(1).map(|new_y| [new_y, x]),
                                    y.checked_sub(1).and_then(|new_y| {
                                        x.checked_add(1).map(|new_x| [new_y, new_x])
                                    }),
                                    x.checked_add(1).map(|new_x| [y, new_x]),
                                ],
                                // Bottom/Right
                                [
                                    y.checked_add(1).map(|new_y| [new_y, x]),
                                    y.checked_add(1).and_then(|new_y| {
                                        x.checked_add(1).map(|new_x| [new_y, new_x])
                                    }),
                                    x.checked_add(1).map(|new_x| [y, new_x]),
                                ],
                                // Bottom/Left
                                [
                                    y.checked_add(1).map(|new_y| [new_y, x]),
                                    y.checked_add(1).and_then(|new_y| {
                                        x.checked_sub(1).map(|new_x| [new_y, new_x])
                                    }),
                                    x.checked_sub(1).map(|new_x| [y, new_x]),
                                ],
                                // Top/Left
                                [
                                    y.checked_sub(1).map(|new_y| [new_y, x]),
                                    y.checked_sub(1).and_then(|new_y| {
                                        x.checked_sub(1).map(|new_x| [new_y, new_x])
                                    }),
                                    x.checked_sub(1).map(|new_x| [y, new_x]),
                                ],
                            ]
                            .into_iter()
                            .map(|[one_side, diagonal, other_side]| {
                                let has_one_side = one_side.is_some_and(|val| plot.contains(&val));
                                let has_diagonal = diagonal.is_some_and(|val| plot.contains(&val));
                                let has_other_side =
                                    other_side.is_some_and(|val| plot.contains(&val));

                                match [has_one_side, has_diagonal, has_other_side] {
                                    [false, _, false] | [true, false, true] => 1,
                                    _ => 0,
                                }
                            })
                        })
                        .sum();

                    plots.push([area, corners]);
                }

                plots.into_iter().map(|[area, corner]| area * corner)
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
        fn larger_example() {
            run(&Case {
                input: super::larger_example().0,
                expected: super::larger_example().1,
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
                expected: 140,
            });
        }

        #[test]
        fn example_2() {
            run(&Case {
                data: super::example_2().1,
                expected: 772,
            });
        }

        #[test]
        fn larger_example() {
            run(&Case {
                data: super::larger_example().1,
                expected: 1930,
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
        fn example_1() {
            run(&Case {
                data: super::example_1().1,
                expected: 80,
            });
        }

        #[test]
        fn example_2() {
            run(&Case {
                data: super::example_2().1,
                expected: 436,
            });
        }

        #[test]
        fn example_3() {
            run(&Case {
                data: super::example_3().1,
                expected: 236,
            });
        }

        #[test]
        fn example_4() {
            run(&Case {
                data: super::example_4().1,
                expected: 368,
            });
        }

        #[test]
        fn larger_example() {
            run(&Case {
                data: super::larger_example().1,
                expected: 1206,
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
            "AAAA
BBCD
BBCC
EEEC",
            Input {
                plants: HashMap::from([
                    (b'A', vec![[0, 0], [0, 1], [0, 2], [0, 3]]),
                    (b'B', vec![[1, 0], [1, 1], [2, 0], [2, 1]]),
                    (b'C', vec![[1, 2], [2, 2], [2, 3], [3, 3]]),
                    (b'D', vec![[1, 3]]),
                    (b'E', vec![[3, 0], [3, 1], [3, 2]]),
                ]),
            },
        )
    }

    fn example_2() -> (&'static str, Input) {
        (
            "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO",
            Input {
                plants: HashMap::from([
                    (
                        b'O',
                        vec![
                            [0, 0],
                            [0, 1],
                            [0, 2],
                            [0, 3],
                            [0, 4],
                            [1, 0],
                            [1, 2],
                            [1, 4],
                            [2, 0],
                            [2, 1],
                            [2, 2],
                            [2, 3],
                            [2, 4],
                            [3, 0],
                            [3, 2],
                            [3, 4],
                            [4, 0],
                            [4, 1],
                            [4, 2],
                            [4, 3],
                            [4, 4],
                        ],
                    ),
                    (b'X', vec![[1, 1], [1, 3], [3, 1], [3, 3]]),
                ]),
            },
        )
    }

    fn example_3() -> (&'static str, Input) {
        (
            "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE",
            Input {
                plants: HashMap::from([
                    (
                        b'E',
                        vec![
                            [0, 0],
                            [0, 1],
                            [0, 2],
                            [0, 3],
                            [0, 4],
                            [1, 0],
                            [2, 0],
                            [2, 1],
                            [2, 2],
                            [2, 3],
                            [2, 4],
                            [3, 0],
                            [4, 0],
                            [4, 1],
                            [4, 2],
                            [4, 3],
                            [4, 4],
                        ],
                    ),
                    (
                        b'X',
                        vec![
                            [1, 1],
                            [1, 2],
                            [1, 3],
                            [1, 4],
                            [3, 1],
                            [3, 2],
                            [3, 3],
                            [3, 4],
                        ],
                    ),
                ]),
            },
        )
    }

    fn example_4() -> (&'static str, Input) {
        (
            "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA",
            Input {
                plants: HashMap::from([
                    (
                        b'A',
                        vec![
                            [0, 0],
                            [0, 1],
                            [0, 2],
                            [0, 3],
                            [0, 4],
                            [0, 5],
                            [1, 0],
                            [1, 1],
                            [1, 2],
                            [1, 5],
                            [2, 0],
                            [2, 1],
                            [2, 2],
                            [2, 5],
                            [3, 0],
                            [3, 3],
                            [3, 4],
                            [3, 5],
                            [4, 0],
                            [4, 3],
                            [4, 4],
                            [4, 5],
                            [5, 0],
                            [5, 1],
                            [5, 2],
                            [5, 3],
                            [5, 4],
                            [5, 5],
                        ],
                    ),
                    (
                        b'B',
                        vec![
                            [1, 3],
                            [1, 4],
                            [2, 3],
                            [2, 4],
                            [3, 1],
                            [3, 2],
                            [4, 1],
                            [4, 2],
                        ],
                    ),
                ]),
            },
        )
    }

    #[allow(clippy::too_many_lines)]
    fn larger_example() -> (&'static str, Input) {
        (
            "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE",
            Input {
                plants: HashMap::from([
                    (
                        b'R',
                        vec![
                            [0, 0],
                            [0, 1],
                            [0, 2],
                            [0, 3],
                            [1, 0],
                            [1, 1],
                            [1, 2],
                            [1, 3],
                            [2, 2],
                            [2, 3],
                            [2, 4],
                            [3, 2],
                        ],
                    ),
                    (
                        b'I',
                        vec![
                            [0, 4],
                            [0, 5],
                            [1, 4],
                            [1, 5],
                            [5, 2],
                            [6, 2],
                            [6, 3],
                            [6, 4],
                            [7, 1],
                            [7, 2],
                            [7, 3],
                            [7, 4],
                            [7, 5],
                            [8, 1],
                            [8, 2],
                            [8, 3],
                            [8, 5],
                            [9, 3],
                        ],
                    ),
                    (
                        b'C',
                        vec![
                            [0, 6],
                            [0, 7],
                            [1, 6],
                            [1, 7],
                            [1, 8],
                            [2, 5],
                            [2, 6],
                            [3, 3],
                            [3, 4],
                            [3, 5],
                            [4, 4],
                            [4, 7],
                            [5, 4],
                            [5, 5],
                            [6, 5],
                        ],
                    ),
                    (
                        b'F',
                        vec![
                            [0, 8],
                            [0, 9],
                            [1, 9],
                            [2, 7],
                            [2, 8],
                            [2, 9],
                            [3, 7],
                            [3, 8],
                            [3, 9],
                            [4, 8],
                        ],
                    ),
                    (
                        b'V',
                        vec![
                            [2, 0],
                            [2, 1],
                            [3, 0],
                            [3, 1],
                            [4, 0],
                            [4, 1],
                            [4, 2],
                            [4, 3],
                            [5, 0],
                            [5, 1],
                            [5, 3],
                            [6, 0],
                            [6, 1],
                        ],
                    ),
                    (
                        b'J',
                        vec![
                            [3, 6],
                            [4, 5],
                            [4, 6],
                            [5, 6],
                            [5, 7],
                            [6, 6],
                            [6, 7],
                            [7, 6],
                            [7, 7],
                            [8, 6],
                            [9, 6],
                        ],
                    ),
                    (
                        b'E',
                        vec![
                            [4, 9],
                            [5, 8],
                            [5, 9],
                            [6, 8],
                            [6, 9],
                            [7, 8],
                            [7, 9],
                            [8, 7],
                            [8, 8],
                            [8, 9],
                            [9, 7],
                            [9, 8],
                            [9, 9],
                        ],
                    ),
                    (b'M', vec![[7, 0], [8, 0], [9, 0], [9, 1], [9, 2]]),
                    (b'S', vec![[8, 4], [9, 4], [9, 5]]),
                ]),
            },
        )
    }
}
