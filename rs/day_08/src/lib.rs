use std::collections::{HashMap, HashSet};
use std::{fs::read_to_string, path::Path};

pub const PART_1: usize = 392;
pub const PART_2: usize = 1235;

#[must_use]
pub fn read_data(data_dir: &str) -> String {
    read_to_string(Path::new(data_dir).join("day_08.txt"))
        .unwrap()
        .trim()
        .to_owned()
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Input {
    antennae: HashMap<u8, Vec<[isize; 2]>>,
    max_x: isize,
    max_y: isize,
}

impl Input {
    #[must_use]
    pub fn from_data(data: &str) -> Self {
        let mut antennae = HashMap::new();
        let mut max_x = 0;
        let mut max_y = 0;

        for (y, line) in data.trim().lines().enumerate() {
            max_y = max_y.max(y);
            for (x, char) in line.bytes().enumerate() {
                max_x = max_x.max(x);
                if char == b'.' {
                    continue;
                }

                let loc = [isize::try_from(y).unwrap(), isize::try_from(x).unwrap()];
                _ = antennae
                    .entry(char)
                    .and_modify(|locs: &mut Vec<[isize; 2]>| {
                        locs.push(loc);
                    })
                    .or_insert_with(|| vec![loc]);
            }
        }

        Self {
            antennae,
            max_x: isize::try_from(max_x).unwrap(),
            max_y: isize::try_from(max_y).unwrap(),
        }
    }

    #[must_use]
    pub fn part_1(&self) -> usize {
        self.antennae
            .values()
            .fold(HashSet::<[isize; 2]>::new(), |mut acc, locations| {
                for &[lhs_y, lhs_x] in locations {
                    for &[rhs_y, rhs_x] in locations {
                        if lhs_x == rhs_x && lhs_y == rhs_y {
                            continue;
                        }

                        let x = lhs_x - rhs_x;
                        let y = lhs_y - rhs_y;

                        let first = [lhs_x + x, lhs_y + y];
                        if (0..=self.max_x).contains(&first[0])
                            && (0..=self.max_y).contains(&first[1])
                        {
                            _ = acc.insert(first);
                        }

                        let second = [rhs_x - x, rhs_y - y];
                        if (0..=self.max_x).contains(&second[0])
                            && (0..=self.max_y).contains(&second[1])
                        {
                            _ = acc.insert(second);
                        }
                    }
                }
                acc
            })
            .len()
    }

    #[must_use]
    pub fn part_2(&self) -> usize {
        self.antennae
            .values()
            .fold(HashSet::<[isize; 2]>::new(), |mut acc, locations| {
                for &[lhs_y, lhs_x] in locations {
                    _ = acc.insert([lhs_x, lhs_y]);
                    for &[rhs_y, rhs_x] in locations {
                        if lhs_x == rhs_x && lhs_y == rhs_y {
                            continue;
                        }

                        let x = lhs_x - rhs_x;
                        let y = lhs_y - rhs_y;

                        for num in 1.. {
                            let node = [lhs_x + (num * x), lhs_y + (num * y)];
                            if !(0..=self.max_x).contains(&node[0])
                                || !(0..=self.max_y).contains(&node[1])
                            {
                                break;
                            }
                            _ = acc.insert(node);
                        }

                        for num in 1.. {
                            let node = [rhs_x - (num * x), rhs_y - (num * y)];
                            if !(0..=self.max_x).contains(&node[0])
                                || !(0..=self.max_y).contains(&node[1])
                            {
                                break;
                            }
                            _ = acc.insert(node);
                        }
                    }
                }
                acc
            })
            .len()
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
                expected: 14,
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
                expected: 34,
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
            "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............",
            Input {
                antennae: HashMap::from([
                    (b'0', vec![[1, 8], [2, 5], [3, 7], [4, 4]]),
                    (b'A', vec![[5, 6], [8, 8], [9, 9]]),
                ]),
                max_x: 11,
                max_y: 11,
            },
        )
    }
}
