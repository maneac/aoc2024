use std::collections::{HashMap, HashSet};
use std::{fs::read_to_string, path::Path};

use rayon::prelude::{IntoParallelIterator, ParallelIterator};

pub const PART_1: usize = 4752;
pub const PART_2: usize = 1719;

#[must_use]
pub fn read_data(data_dir: &str) -> String {
    read_to_string(Path::new(data_dir).join("day_06.txt"))
        .unwrap()
        .trim()
        .to_owned()
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Input {
    x_limmit: usize,
    y_limit: usize,
    obstacles: HashSet<[usize; 2]>,
    guard_position: [usize; 2],
}

impl Input {
    #[must_use]
    pub fn from_data(data: &str) -> Self {
        data.trim()
            .lines()
            .enumerate()
            .fold(Self::default(), |mut acc, (y_idx, line)| {
                line.char_indices().for_each(|(x_idx, char)| match char {
                    '#' => {
                        _ = acc.obstacles.insert([y_idx, x_idx]);
                    }
                    '^' => {
                        acc.guard_position = [y_idx, x_idx];
                    }
                    _ => {}
                });

                acc.x_limmit = acc.x_limmit.max(line.len());
                acc.y_limit = acc.y_limit.max(y_idx + 1);
                acc
            })
    }

    #[must_use]
    pub fn part_1(&self) -> usize {
        self.walk(&self.obstacles).unwrap().len()
    }

    #[must_use]
    pub fn part_2(&self) -> usize {
        let initial_path = self.walk(&self.obstacles).unwrap();

        initial_path
            .into_par_iter()
            .filter(|&obstacle| {
                if obstacle == self.guard_position {
                    return false;
                }
                let mut obstacles = self.obstacles.clone();
                _ = obstacles.insert(obstacle);
                self.walk(&obstacles).is_none()
            })
            .count()
    }

    fn walk(&self, obstacles: &HashSet<[usize; 2]>) -> Option<HashSet<[usize; 2]>> {
        let [mut guard_y, mut guard_x] = self.guard_position;
        let mut guard_facing = Facing::Up;

        let mut visited = HashMap::<[usize; 2], Vec<Facing>>::new();
        loop {
            if guard_x >= self.x_limmit || guard_y >= self.y_limit {
                return Some(visited.into_keys().collect());
            }

            if let Some(facings) = visited.get_mut(&[guard_y, guard_x]) {
                if facings.contains(&guard_facing) {
                    return None;
                }
                facings.push(guard_facing);
            } else {
                let _prev = visited.insert([guard_y, guard_x], vec![guard_facing]);
            }

            let next_location = match guard_facing {
                Facing::Up => [guard_y.wrapping_sub(1), guard_x],
                Facing::Right => [guard_y, guard_x + 1],
                Facing::Down => [guard_y + 1, guard_x],
                Facing::Left => [guard_y, guard_x.wrapping_sub(1)],
            };

            if obstacles.contains(&next_location) {
                guard_facing = guard_facing.next()?;
            } else {
                guard_y = next_location[0];
                guard_x = next_location[1];
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
enum Facing {
    #[default]
    Up,
    Right,
    Down,
    Left,
}

#[expect(clippy::copy_iterator)]
impl Iterator for Facing {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        Some(match *self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        })
    }
}

#[cfg(test)]
mod day_06_tests {
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
                expected: 41,
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
                expected: 6,
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
            "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...",
            Input {
                y_limit: 10,
                x_limmit: 10,
                obstacles: HashSet::from([
                    [0, 4],
                    [1, 9],
                    [3, 2],
                    [4, 7],
                    [6, 1],
                    [7, 8],
                    [8, 0],
                    [9, 6],
                ]),
                guard_position: [6, 4],
            },
        )
    }
}
