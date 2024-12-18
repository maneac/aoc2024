use rayon::prelude::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use std::{
    collections::{BTreeMap, BTreeSet, HashSet},
    fs::read_to_string,
    path::Path,
};

pub const PART_1: usize = 374;
pub const PART_2: &str = "30,12";

#[must_use]
pub fn read_data(data_dir: &str) -> String {
    read_to_string(Path::new(data_dir).join("day_18.txt"))
        .unwrap()
        .trim()
        .to_owned()
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Input {
    incoming_bytes: Vec<[u8; 2]>,
}

impl Input {
    #[must_use]
    pub fn from_data(data: &str) -> Self {
        let incoming_bytes = data
            .trim()
            .lines()
            .map(|line| {
                let (x_str, y_str) = line.split_once(',').unwrap();
                [x_str.parse().unwrap(), y_str.parse().unwrap()]
            })
            .collect();

        Self { incoming_bytes }
    }

    #[must_use]
    pub fn part_1(&self) -> usize {
        self.part_1_sized::<70, 1024>()
    }

    #[must_use]
    pub fn part_2(&self) -> String {
        self.part_2_sized::<70, 1024>()
    }

    fn part_1_sized<const SIZE: u8, const SIMULATION_COUNT: usize>(&self) -> usize {
        let occupied = self
            .incoming_bytes
            .iter()
            .copied()
            .take(SIMULATION_COUNT)
            .collect::<HashSet<[u8; 2]>>();

        let target = [SIZE; 2];
        let mut working_set = BTreeSet::from([(0_usize, [0, 0])]);
        while let Some((length, position)) = working_set.pop_first() {
            if position == target {
                return length;
            }

            let [x, y] = position;
            for next_position in [
                x.checked_sub(1).map(|new_x| [new_x, y]),
                x.checked_add(1).map(|new_x| [new_x, y]),
                y.checked_sub(1).map(|new_y| [x, new_y]),
                y.checked_add(1).map(|new_y| [x, new_y]),
            ]
            .into_iter()
            .flatten()
            .filter(|next_pos| {
                next_pos[0] <= SIZE && next_pos[1] <= SIZE && !occupied.contains(next_pos)
            }) {
                _ = working_set.insert((length + 1, next_position));
            }
        }
        #[expect(clippy::unreachable)]
        {
            unreachable!()
        }
    }

    fn part_2_sized<const SIZE: u8, const MIN_SIMULATIONS: usize>(&self) -> String {
        let target = [SIZE; 2];
        self.incoming_bytes
            .par_iter()
            .copied()
            .enumerate()
            .skip(MIN_SIMULATIONS)
            .find_first(|&(idx, _byte)| {
                let occupied = self
                    .incoming_bytes
                    .iter()
                    .copied()
                    .take(idx + 1)
                    .collect::<HashSet<[u8; 2]>>();

                let mut working_set = BTreeMap::from([((SIZE + SIZE, [0, 0]), HashSet::new())]);
                while let Some(((_distance, position), mut history)) = working_set.pop_first() {
                    if position == target {
                        return false;
                    }

                    _ = history.insert(position);
                    let [x, y] = position;
                    for next_position in [
                        x.checked_sub(1).map(|new_x| [new_x, y]),
                        x.checked_add(1).map(|new_x| [new_x, y]),
                        y.checked_sub(1).map(|new_y| [x, new_y]),
                        y.checked_add(1).map(|new_y| [x, new_y]),
                    ]
                    .into_iter()
                    .flatten()
                    .filter(|next_pos| {
                        next_pos[0] <= SIZE
                            && next_pos[1] <= SIZE
                            && !occupied.contains(next_pos)
                            && !history.contains(next_pos)
                    }) {
                        let _prev = working_set.insert(
                            (
                                SIZE - next_position[0] + SIZE - next_position[1],
                                next_position,
                            ),
                            history.clone(),
                        );
                    }
                }
                true
            })
            .map(|(_, [final_x, final_y])| format!("{final_x},{final_y}"))
            .unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct GridSize<const WIDTH: usize = 70, const HEIGHT: usize = 70> {}

#[cfg(test)]
mod day_18_tests {
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

    mod part_1_sized {
        use super::*;

        struct Case {
            data: Input,
            expected: usize,
        }

        #[test]
        fn example() {
            run(&Case {
                data: super::example().1,
                expected: 22,
            });
        }

        fn run(test: &Case) {
            assert_eq!(test.expected, test.data.part_1_sized::<6, 12>());
        }
    }

    mod part_1 {
        use super::*;

        struct Case {
            data: Input,
            expected: usize,
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

    mod part_2_sized {
        use super::*;

        struct Case {
            data: Input,
            expected: &'static str,
        }

        #[test]
        fn example() {
            run(&Case {
                data: super::example().1,
                expected: "6,1",
            });
        }
        fn run(test: &Case) {
            assert_eq!(test.expected, test.data.part_2_sized::<6, 12>());
        }
    }

    mod part_2 {
        use super::*;

        struct Case {
            data: Input,
            expected: &'static str,
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
            "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0",
            Input {
                incoming_bytes: vec![
                    [5, 4],
                    [4, 2],
                    [4, 5],
                    [3, 0],
                    [2, 1],
                    [6, 3],
                    [2, 4],
                    [1, 5],
                    [0, 6],
                    [3, 3],
                    [2, 6],
                    [5, 1],
                    [1, 2],
                    [5, 5],
                    [2, 5],
                    [6, 5],
                    [1, 4],
                    [0, 4],
                    [6, 4],
                    [1, 1],
                    [6, 1],
                    [1, 0],
                    [0, 5],
                    [1, 6],
                    [2, 0],
                ],
            },
        )
    }
}
