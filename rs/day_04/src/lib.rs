use std::{fs::read_to_string, path::Path};

pub const PART_1: usize = 2578;
pub const PART_2: usize = 1972;

pub fn read_data(data_dir: &str) -> String {
    read_to_string(Path::new(data_dir).join("day_04.txt"))
        .unwrap()
        .trim()
        .to_string()
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Input<'i> {
    grid: Vec<&'i [u8]>,
}

impl<'i> Input<'i> {
    pub fn from_data(data: &'i str) -> Self {
        let grid = data.lines().map(|line| line.as_bytes()).collect::<Vec<_>>();
        Self { grid }
    }

    pub fn part_1(&self) -> usize {
        let x_locs = self
            .grid
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .filter_map(move |(x, c)| (*c == b'X').then_some((y, x)))
            })
            .collect::<Vec<_>>();

        x_locs
            .iter()
            .map(|&(y, x)| {
                [
                    // right
                    self.grid.get(y).and_then(|row| row.get(x + 1)) == Some(&b'M')
                        && self.grid.get(y).and_then(|row| row.get(x + 2)) == Some(&b'A')
                        && self.grid.get(y).and_then(|row| row.get(x + 3)) == Some(&b'S'),
                    // right / down
                    self.grid.get(y + 1).and_then(|row| row.get(x + 1)) == Some(&b'M')
                        && self.grid.get(y + 2).and_then(|row| row.get(x + 2)) == Some(&b'A')
                        && self.grid.get(y + 3).and_then(|row| row.get(x + 3)) == Some(&b'S'),
                    // down
                    self.grid.get(y + 1).and_then(|row| row.get(x)) == Some(&b'M')
                        && self.grid.get(y + 2).and_then(|row| row.get(x)) == Some(&b'A')
                        && self.grid.get(y + 3).and_then(|row| row.get(x)) == Some(&b'S'),
                    // left / down
                    x >= 3
                        && self.grid.get(y + 1).and_then(|row| row.get(x - 1)) == Some(&b'M')
                        && self.grid.get(y + 2).and_then(|row| row.get(x - 2)) == Some(&b'A')
                        && self.grid.get(y + 3).and_then(|row| row.get(x - 3)) == Some(&b'S'),
                    // left
                    x >= 3
                        && self.grid.get(y).and_then(|row| row.get(x - 1)) == Some(&b'M')
                        && self.grid.get(y).and_then(|row| row.get(x - 2)) == Some(&b'A')
                        && self.grid.get(y).and_then(|row| row.get(x - 3)) == Some(&b'S'),
                    // left / up
                    x >= 3
                        && y >= 3
                        && self.grid.get(y - 1).and_then(|row| row.get(x - 1)) == Some(&b'M')
                        && self.grid.get(y - 2).and_then(|row| row.get(x - 2)) == Some(&b'A')
                        && self.grid.get(y - 3).and_then(|row| row.get(x - 3)) == Some(&b'S'),
                    // up
                    y >= 3
                        && self.grid.get(y - 1).and_then(|row| row.get(x)) == Some(&b'M')
                        && self.grid.get(y - 2).and_then(|row| row.get(x)) == Some(&b'A')
                        && self.grid.get(y - 3).and_then(|row| row.get(x)) == Some(&b'S'),
                    // up / right
                    y >= 3
                        && self.grid.get(y - 1).and_then(|row| row.get(x + 1)) == Some(&b'M')
                        && self.grid.get(y - 2).and_then(|row| row.get(x + 2)) == Some(&b'A')
                        && self.grid.get(y - 3).and_then(|row| row.get(x + 3)) == Some(&b'S'),
                ]
                .iter()
                .filter(|v| **v)
                .count()
            })
            .sum()
    }

    pub fn part_2(&self) -> usize {
        let a_locs = self
            .grid
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .filter_map(move |(x, c)| (*c == b'A').then_some((y, x)))
            })
            .collect::<Vec<_>>();

        a_locs
            .iter()
            .filter(|&&(y, x)| y > 0 && x > 0)
            .map(|&(y, x)| {
                [
                    // M,M,S,S
                    self.grid.get(y - 1).and_then(|row| row.get(x - 1)) == Some(&b'M')
                        && self.grid.get(y - 1).and_then(|row| row.get(x + 1)) == Some(&b'M')
                        && self.grid.get(y + 1).and_then(|row| row.get(x - 1)) == Some(&b'S')
                        && self.grid.get(y + 1).and_then(|row| row.get(x + 1)) == Some(&b'S'),
                    // M,S,M,S
                    self.grid.get(y - 1).and_then(|row| row.get(x - 1)) == Some(&b'M')
                        && self.grid.get(y - 1).and_then(|row| row.get(x + 1)) == Some(&b'S')
                        && self.grid.get(y + 1).and_then(|row| row.get(x - 1)) == Some(&b'M')
                        && self.grid.get(y + 1).and_then(|row| row.get(x + 1)) == Some(&b'S'),
                    // S,M,S,M
                    self.grid.get(y - 1).and_then(|row| row.get(x - 1)) == Some(&b'S')
                        && self.grid.get(y - 1).and_then(|row| row.get(x + 1)) == Some(&b'M')
                        && self.grid.get(y + 1).and_then(|row| row.get(x - 1)) == Some(&b'S')
                        && self.grid.get(y + 1).and_then(|row| row.get(x + 1)) == Some(&b'M'),
                    // S,S,M,M
                    self.grid.get(y - 1).and_then(|row| row.get(x - 1)) == Some(&b'S')
                        && self.grid.get(y - 1).and_then(|row| row.get(x + 1)) == Some(&b'S')
                        && self.grid.get(y + 1).and_then(|row| row.get(x - 1)) == Some(&b'M')
                        && self.grid.get(y + 1).and_then(|row| row.get(x + 1)) == Some(&b'M'),
                ]
                .iter()
                .filter(|v| **v)
                .count()
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
            expected: Input<'c>,
        }

        #[test]
        fn example() {
            run(&Case {
                input: super::example().0,
                expected: super::example().1,
            })
        }

        fn run(test: &Case) {
            assert_eq!(test.expected, Input::from_data(test.input))
        }
    }

    mod part_1 {
        use super::*;

        struct Case<'c> {
            data: Input<'c>,
            expected: usize,
        }

        #[test]
        fn example() {
            run(&Case {
                data: super::example().1,
                expected: 18,
            })
        }

        #[test]
        fn actual() {
            run(&Case {
                data: Input::from_data(&read_data(DATA_DIR)),
                expected: PART_1,
            })
        }

        fn run(test: &Case) {
            assert_eq!(test.expected, test.data.part_1())
        }
    }

    mod part_2 {
        use super::*;

        struct Case<'c> {
            data: Input<'c>,
            expected: usize,
        }

        #[test]
        fn example() {
            run(&Case {
                data: super::example().1,
                expected: 9,
            })
        }

        #[test]
        fn actual() {
            run(&Case {
                data: Input::from_data(&read_data(DATA_DIR)),
                expected: PART_2,
            })
        }

        fn run(test: &Case) {
            assert_eq!(test.expected, test.data.part_2())
        }
    }

    fn example() -> (&'static str, Input<'static>) {
        (
            "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
            Input {
                grid: vec![
                    b"MMMSXXMASM",
                    b"MSAMXMSMSA",
                    b"AMXSXMAAMM",
                    b"MSAMASMSMX",
                    b"XMASAMXAMM",
                    b"XXAMMXXAMA",
                    b"SMSMSASXSS",
                    b"SAXAMASAAA",
                    b"MAMMMXMMMM",
                    b"MXMXAXMASX",
                ],
            },
        )
    }
}
