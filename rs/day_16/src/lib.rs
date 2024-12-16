use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::{fs::read_to_string, path::Path};

pub const PART_1: usize = 103_512;
pub const PART_2: usize = 554;

#[must_use]
pub fn read_data(data_dir: &str) -> String {
    read_to_string(Path::new(data_dir).join("day_16.txt"))
        .unwrap()
        .trim()
        .to_owned()
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Input {
    start: [usize; 2],
    end: [usize; 2],
    walls: HashSet<[usize; 2]>,
}

impl Input {
    #[must_use]
    pub fn from_data(data: &str) -> Self {
        let mut start = None;
        let mut end = None;
        let mut walls = HashSet::new();

        for (y, line) in data.trim().lines().enumerate() {
            for (x, char) in line.char_indices() {
                match char {
                    'S' => {
                        _ = start.replace([y, x]);
                    }
                    'E' => {
                        _ = end.replace([y, x]);
                    }
                    '#' => {
                        _ = walls.insert([y, x]);
                    }
                    _ => {}
                }
            }
        }

        Self {
            start: start.unwrap(),
            end: end.unwrap(),
            walls,
        }
    }

    #[must_use]
    pub fn part_1(&self) -> usize {
        let mut visited = HashSet::new();
        let mut working_set = VecDeque::from([(0_usize, self.start, Direction::East)]);
        while let Some((score, location, facing)) = working_set.pop_front() {
            if location == self.end {
                return score;
            }

            if visited.contains(&(location, facing)) {
                continue;
            }
            _ = visited.insert((location, facing));

            let (forward, rotations) = match facing {
                Direction::East => (
                    [location[0], location[1] + 1],
                    [Direction::North, Direction::South],
                ),
                Direction::North => (
                    [location[0].saturating_sub(1), location[1]],
                    [Direction::East, Direction::West],
                ),
                Direction::South => (
                    [location[0] + 1, location[1]],
                    [Direction::West, Direction::East],
                ),
                Direction::West => (
                    [location[0], location[1].saturating_sub(1)],
                    [Direction::South, Direction::North],
                ),
            };

            if !self.walls.contains(&forward) {
                let to_insert = (score + 1, forward, facing);
                if let Err(idx) = working_set.binary_search(&to_insert) {
                    working_set.insert(idx, to_insert);
                }
            }

            for new_facing in rotations {
                let to_insert = (score + 1000, location, new_facing);
                if let Err(idx) = working_set.binary_search(&to_insert) {
                    working_set.insert(idx, to_insert);
                }
            }
        }

        #[expect(clippy::unreachable)]
        {
            unreachable!()
        }
    }

    #[must_use]
    pub fn part_2(&self) -> usize {
        let mut best_score = None;

        let mut best_score_visited = HashSet::new();
        let mut visited = HashMap::new();

        let mut working_set =
            BTreeMap::from([((0_usize, self.start, Direction::East), vec![vec![]])]);
        while let Some(((score, location, facing), mut histories)) = working_set.pop_first() {
            if best_score.is_some_and(|best| score > best) {
                break;
            }

            if location == self.end {
                let best = *best_score.get_or_insert(score);
                if score == best {
                    for history in histories {
                        best_score_visited.extend(history);
                    }
                    _ = best_score_visited.insert(location);
                }
                continue;
            }

            if visited
                .get(&(location, facing))
                .is_some_and(|&existing| existing < score)
            {
                continue;
            }
            _ = visited.insert((location, facing), score);

            for history in &mut histories {
                history.push(location);
            }

            let (forward, rotations) = match facing {
                Direction::East => (
                    [location[0], location[1] + 1],
                    [Direction::North, Direction::South],
                ),
                Direction::North => (
                    [location[0].saturating_sub(1), location[1]],
                    [Direction::East, Direction::West],
                ),
                Direction::South => (
                    [location[0] + 1, location[1]],
                    [Direction::West, Direction::East],
                ),
                Direction::West => (
                    [location[0], location[1].saturating_sub(1)],
                    [Direction::South, Direction::North],
                ),
            };

            if !self.walls.contains(&forward) && best_score.is_none_or(|best| score < best) {
                _ = working_set
                    .entry((score + 1, forward, facing))
                    .and_modify(|vals| {
                        vals.append(&mut histories);
                    })
                    .or_insert_with(|| histories.clone());
            }

            if best_score.is_none_or(|best| (score + 1000) < best) {
                for new_facing in rotations {
                    _ = working_set
                        .entry((score + 1000, location, new_facing))
                        .and_modify(|vals| {
                            vals.append(&mut histories);
                        })
                        .or_insert_with(|| histories.clone());
                }
            }
        }

        best_score_visited.len()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[cfg(test)]
mod day_16_tests {
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
                expected: 7036,
            });
        }

        #[test]
        fn example_2() {
            run(&Case {
                data: super::example_2().1,
                expected: 11048,
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
                expected: 45,
            });
        }

        #[test]
        fn example_2() {
            run(&Case {
                data: super::example_2().1,
                expected: 64,
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

    #[expect(clippy::too_many_lines)]
    fn example_1() -> (&'static str, Input) {
        (
            "
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############",
            Input {
                start: [13, 1],
                end: [1, 13],
                walls: HashSet::from([
                    [0, 0],
                    [0, 1],
                    [0, 2],
                    [0, 3],
                    [0, 4],
                    [0, 5],
                    [0, 6],
                    [0, 7],
                    [0, 8],
                    [0, 9],
                    [0, 10],
                    [0, 11],
                    [0, 12],
                    [0, 13],
                    [0, 14],
                    [1, 0],
                    [1, 8],
                    [1, 14],
                    [2, 0],
                    [2, 2],
                    [2, 4],
                    [2, 5],
                    [2, 6],
                    [2, 8],
                    [2, 10],
                    [2, 11],
                    [2, 12],
                    [2, 14],
                    [3, 0],
                    [3, 6],
                    [3, 8],
                    [3, 12],
                    [3, 14],
                    [4, 0],
                    [4, 2],
                    [4, 3],
                    [4, 4],
                    [4, 6],
                    [4, 7],
                    [4, 8],
                    [4, 9],
                    [4, 10],
                    [4, 12],
                    [4, 14],
                    [5, 0],
                    [5, 2],
                    [5, 4],
                    [5, 12],
                    [5, 14],
                    [6, 0],
                    [6, 2],
                    [6, 4],
                    [6, 5],
                    [6, 6],
                    [6, 7],
                    [6, 8],
                    [6, 10],
                    [6, 11],
                    [6, 12],
                    [6, 14],
                    [7, 0],
                    [7, 12],
                    [7, 14],
                    [8, 0],
                    [8, 1],
                    [8, 2],
                    [8, 4],
                    [8, 6],
                    [8, 7],
                    [8, 8],
                    [8, 9],
                    [8, 10],
                    [8, 12],
                    [8, 14],
                    [9, 0],
                    [9, 4],
                    [9, 10],
                    [9, 12],
                    [9, 14],
                    [10, 0],
                    [10, 2],
                    [10, 4],
                    [10, 6],
                    [10, 7],
                    [10, 8],
                    [10, 10],
                    [10, 12],
                    [10, 14],
                    [11, 0],
                    [11, 6],
                    [11, 10],
                    [11, 12],
                    [11, 14],
                    [12, 0],
                    [12, 2],
                    [12, 3],
                    [12, 4],
                    [12, 6],
                    [12, 8],
                    [12, 10],
                    [12, 12],
                    [12, 14],
                    [13, 0],
                    [13, 4],
                    [13, 10],
                    [13, 14],
                    [14, 0],
                    [14, 1],
                    [14, 2],
                    [14, 3],
                    [14, 4],
                    [14, 5],
                    [14, 6],
                    [14, 7],
                    [14, 8],
                    [14, 9],
                    [14, 10],
                    [14, 11],
                    [14, 12],
                    [14, 13],
                    [14, 14],
                ]),
            },
        )
    }

    #[expect(clippy::too_many_lines)]
    fn example_2() -> (&'static str, Input) {
        (
            "
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################",
            Input {
                start: [15, 1],
                end: [1, 15],
                walls: HashSet::from([
                    [0, 0],
                    [0, 1],
                    [0, 2],
                    [0, 3],
                    [0, 4],
                    [0, 5],
                    [0, 6],
                    [0, 7],
                    [0, 8],
                    [0, 9],
                    [0, 10],
                    [0, 11],
                    [0, 12],
                    [0, 13],
                    [0, 14],
                    [0, 15],
                    [0, 16],
                    [1, 0],
                    [1, 4],
                    [1, 8],
                    [1, 12],
                    [1, 16],
                    [2, 0],
                    [2, 2],
                    [2, 4],
                    [2, 6],
                    [2, 8],
                    [2, 10],
                    [2, 12],
                    [2, 14],
                    [2, 16],
                    [3, 0],
                    [3, 2],
                    [3, 4],
                    [3, 6],
                    [3, 10],
                    [3, 14],
                    [3, 16],
                    [4, 0],
                    [4, 2],
                    [4, 4],
                    [4, 6],
                    [4, 8],
                    [4, 9],
                    [4, 10],
                    [4, 12],
                    [4, 14],
                    [4, 16],
                    [5, 0],
                    [5, 4],
                    [5, 6],
                    [5, 8],
                    [5, 14],
                    [5, 16],
                    [6, 0],
                    [6, 2],
                    [6, 4],
                    [6, 6],
                    [6, 8],
                    [6, 10],
                    [6, 11],
                    [6, 12],
                    [6, 13],
                    [6, 14],
                    [6, 16],
                    [7, 0],
                    [7, 2],
                    [7, 6],
                    [7, 8],
                    [7, 10],
                    [7, 16],
                    [8, 0],
                    [8, 2],
                    [8, 4],
                    [8, 5],
                    [8, 6],
                    [8, 7],
                    [8, 8],
                    [8, 10],
                    [8, 12],
                    [8, 13],
                    [8, 14],
                    [8, 16],
                    [9, 0],
                    [9, 2],
                    [9, 4],
                    [9, 12],
                    [9, 16],
                    [10, 0],
                    [10, 2],
                    [10, 4],
                    [10, 5],
                    [10, 6],
                    [10, 8],
                    [10, 9],
                    [10, 10],
                    [10, 11],
                    [10, 12],
                    [10, 14],
                    [10, 15],
                    [10, 16],
                    [11, 0],
                    [11, 2],
                    [11, 4],
                    [11, 8],
                    [11, 14],
                    [11, 16],
                    [12, 0],
                    [12, 2],
                    [12, 4],
                    [12, 6],
                    [12, 7],
                    [12, 8],
                    [12, 9],
                    [12, 10],
                    [12, 12],
                    [12, 13],
                    [12, 14],
                    [12, 16],
                    [13, 0],
                    [13, 2],
                    [13, 4],
                    [13, 14],
                    [13, 16],
                    [14, 0],
                    [14, 2],
                    [14, 4],
                    [14, 6],
                    [14, 7],
                    [14, 8],
                    [14, 9],
                    [14, 10],
                    [14, 11],
                    [14, 12],
                    [14, 13],
                    [14, 14],
                    [14, 16],
                    [15, 0],
                    [15, 2],
                    [15, 16],
                    [16, 0],
                    [16, 1],
                    [16, 2],
                    [16, 3],
                    [16, 4],
                    [16, 5],
                    [16, 6],
                    [16, 7],
                    [16, 8],
                    [16, 9],
                    [16, 10],
                    [16, 11],
                    [16, 12],
                    [16, 13],
                    [16, 14],
                    [16, 15],
                    [16, 16],
                ]),
            },
        )
    }
}
