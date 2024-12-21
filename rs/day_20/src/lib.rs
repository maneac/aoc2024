use std::{
    collections::{BTreeMap, BTreeSet, HashSet},
    fs::read_to_string,
    path::Path,
};

pub const PART_1: usize = 1372;
pub const PART_2: usize = 979_014;

#[must_use]
pub fn read_data(data_dir: &str) -> String {
    read_to_string(Path::new(data_dir).join("day_20.txt"))
        .unwrap()
        .trim()
        .to_owned()
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Input {
    race_track: HashSet<[usize; 2]>,
    start: [usize; 2],
    end: [usize; 2],
}

impl Input {
    #[must_use]
    pub fn from_data(data: &str) -> Self {
        let mut race_track = HashSet::new();
        let mut start = None;
        let mut end = None;
        for (y, line) in data.trim().lines().enumerate() {
            for (x, char) in line.char_indices() {
                match char {
                    '#' => {
                        _ = race_track.insert([y, x]);
                    }
                    'S' => {
                        _ = start.replace([y, x]);
                    }
                    'E' => {
                        _ = end.replace([y, x]);
                    }
                    _ => {}
                }
            }
        }

        Self {
            race_track,
            start: start.unwrap(),
            end: end.unwrap(),
        }
    }

    #[must_use]
    pub fn part_1(&self) -> usize {
        let cheats = self.cheat_savings::<2>();
        cheats
            .iter()
            .filter_map(|(&size, options)| (size >= 100).then_some(options.len()))
            .sum()
    }

    #[must_use]
    pub fn part_2(&self) -> usize {
        let cheats = self.cheat_savings::<20>();
        cheats
            .iter()
            .filter_map(|(&size, options)| (size >= 100).then_some(options.len()))
            .sum()
    }

    fn cheat_savings<const MAX_LEN: usize>(&self) -> BTreeMap<usize, HashSet<[[usize; 2]; 2]>> {
        let path = self.solve_maze();

        let mut cheats = BTreeMap::new();
        for (lhs_idx, lhs) in path.iter().copied().enumerate() {
            for (rhs_idx, rhs) in path
                .iter()
                .copied()
                .enumerate()
                .rev()
                .take_while(|(idx, _)| idx != &lhs_idx)
            {
                let distance = rhs[0].abs_diff(lhs[0]) + rhs[1].abs_diff(lhs[1]);
                if distance > MAX_LEN {
                    continue;
                }
                let cheat_size = rhs_idx - lhs_idx - distance;
                _ = cheats
                    .entry(cheat_size)
                    .and_modify(|values: &mut HashSet<[[usize; 2]; 2]>| {
                        _ = values.insert([lhs, rhs]);
                    })
                    .or_insert_with(|| HashSet::from([[lhs, rhs]]));
            }
        }
        cheats
    }

    fn solve_maze(&self) -> Vec<[usize; 2]> {
        let mut working_set = BTreeSet::from([(
            self.start[0].abs_diff(self.end[0] + self.start[1].abs_diff(self.end[1])),
            self.start,
            vec![],
        )]);
        while let Some((_distance, current, mut history)) = working_set.pop_first() {
            history.push(current);
            if current == self.end {
                return history;
            }

            let [y, x] = current;
            for neighbour in [
                x.checked_add(1).map(|new_x| [y, new_x]),
                x.checked_sub(1).map(|new_x| [y, new_x]),
                y.checked_add(1).map(|new_y| [new_y, x]),
                y.checked_sub(1).map(|new_y| [new_y, x]),
            ]
            .into_iter()
            .flatten()
            .filter(|neighbour| {
                !history.contains(neighbour) && !self.race_track.contains(neighbour)
            }) {
                let distance =
                    neighbour[0].abs_diff(self.end[0]) + neighbour[1].abs_diff(self.end[1]);
                _ = working_set.insert((distance, neighbour, history.clone()));
            }
        }

        #[expect(clippy::unreachable)]
        {
            unreachable!()
        }
    }
}

#[cfg(test)]
mod day_20_tests {
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
                expected: 0,
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
                expected: 0,
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
        let example = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
        (example, Input::from_data(example))
    }
}
