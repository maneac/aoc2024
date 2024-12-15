use std::collections::{BTreeSet, HashMap, HashSet};
use std::{fs::read_to_string, path::Path};

pub const PART_1: usize = 1_414_416;
pub const PART_2: usize = 1_386_070;

#[must_use]
pub fn read_data(data_dir: &str) -> String {
    read_to_string(Path::new(data_dir).join("day_15.txt"))
        .unwrap()
        .trim()
        .to_owned()
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Input {
    warehouse: HashMap<[usize; 2], Obstacle>,
    robot: [usize; 2],
    instructions: Vec<Direction>,
}

impl Input {
    #[must_use]
    pub fn from_data(data: &str) -> Self {
        let (warehouse_str, instruction_str) = data.trim().split_once("\n\n").unwrap();

        let mut warehouse = HashMap::new();
        let mut robot = [0_usize; 2];
        for (y, line) in warehouse_str.trim().lines().enumerate() {
            for (x, char) in line.char_indices() {
                let obstacle = match char {
                    '#' => Obstacle::Wall,
                    'O' => Obstacle::Box,
                    '@' => {
                        robot = [y, x];
                        continue;
                    }
                    _ => continue,
                };
                _ = warehouse.insert([y, x], obstacle);
            }
        }

        let instructions = instruction_str
            .trim()
            .chars()
            .filter(|char| *char != '\n')
            .map(|char| match char {
                '^' => Direction::Up,
                '>' => Direction::Right,
                'v' => Direction::Down,
                '<' => Direction::Left,
                #[expect(clippy::unreachable)]
                _ => unreachable!(),
            })
            .collect();

        Self {
            warehouse,
            robot,
            instructions,
        }
    }

    #[must_use]
    pub fn part_1(&self) -> usize {
        let warehouse = self
            .instructions
            .iter()
            .fold(
                (self.robot, self.warehouse.clone()),
                |(robot, mut warehouse), &direction| {
                    let [d_y, d_x]: [isize; 2] = match direction {
                        Direction::Up => [-1, 0],
                        Direction::Right => [0, 1],
                        Direction::Down => [1, 0],
                        Direction::Left => [0, -1],
                    };

                    let mut empty_loc = robot;
                    loop {
                        empty_loc = [
                            empty_loc[0].saturating_add_signed(d_y),
                            empty_loc[1].saturating_add_signed(d_x),
                        ];
                        match warehouse.get(&empty_loc) {
                            Some(Obstacle::Wall) => {
                                return (robot, warehouse);
                            }
                            None => {
                                break;
                            }
                            Some(_) => {}
                        }
                    }

                    let mut tile = empty_loc;
                    loop {
                        let prev_tile = [
                            tile[0].saturating_add_signed(-d_y),
                            tile[1].saturating_add_signed(-d_x),
                        ];

                        let current = warehouse.remove(&prev_tile);
                        match current {
                            Some(obstacle) => {
                                _ = warehouse.insert(tile, obstacle);
                            }
                            None => break,
                        }
                        tile = prev_tile;
                    }

                    (tile, warehouse)
                },
            )
            .1;

        warehouse
            .iter()
            .filter_map(|([y, x], obstacle)| match obstacle {
                Obstacle::Box => Some((100 * y) + x),
                Obstacle::Wall | Obstacle::BoxLeft | Obstacle::BoxRight => None,
            })
            .sum()
    }

    #[must_use]
    pub fn part_2(&self) -> usize {
        let mut wide_warehouse = HashMap::with_capacity(2 * self.warehouse.len());

        for (&[y, x], &obstacle) in &self.warehouse {
            match obstacle {
                Obstacle::Box => {
                    _ = wide_warehouse.insert([y, 2 * x], Obstacle::BoxLeft);
                    _ = wide_warehouse.insert([y, (2 * x) + 1], Obstacle::BoxRight);
                }
                Obstacle::Wall => {
                    _ = wide_warehouse.insert([y, 2 * x], Obstacle::Wall);
                    _ = wide_warehouse.insert([y, (2 * x) + 1], Obstacle::Wall);
                }
                Obstacle::BoxLeft | Obstacle::BoxRight => {}
            }
        }

        let wide_robot = [self.robot[0], self.robot[1] * 2];

        let (_robot, warehouse) = self.instructions.iter().fold(
            (wide_robot, wide_warehouse),
            |(robot, mut warehouse), &direction| {
                let [d_y, d_x]: [isize; 2] = match direction {
                    Direction::Up => [-1, 0],
                    Direction::Right => [0, 1],
                    Direction::Down => [1, 0],
                    Direction::Left => [0, -1],
                };

                let new_robot = [
                    robot[0].saturating_add_signed(d_y),
                    robot[1].saturating_add_signed(d_x),
                ];

                let mut visited = HashSet::new();
                let mut to_check = BTreeSet::from([new_robot]);
                while let Some([y, x]) = to_check.pop_first() {
                    if visited.contains(&[y, x]) {
                        continue;
                    }

                    _ = visited.insert([y, x]);

                    let next_loc = [y.saturating_add_signed(d_y), x.saturating_add_signed(d_x)];
                    match warehouse.get(&[y, x]) {
                        Some(Obstacle::Wall) => {
                            return (robot, warehouse);
                        }
                        Some(Obstacle::BoxLeft) => {
                            _ = to_check.insert(next_loc);
                            _ = to_check.insert([next_loc[0], next_loc[1] + 1]);
                        }
                        Some(Obstacle::BoxRight) => {
                            _ = to_check.insert(next_loc);
                            _ = to_check.insert([next_loc[0], next_loc[1].saturating_sub(1)]);
                        }
                        Some(Obstacle::Box) | None => {}
                    }
                }

                let mut removed = BTreeSet::from([(robot, None)]);
                while let Some(([y, x], obstacle)) = removed.pop_first() {
                    let target = [y.saturating_add_signed(d_y), x.saturating_add_signed(d_x)];

                    let prev_target_value = if let Some(obstacle) = obstacle {
                        warehouse.insert(target, obstacle)
                    } else {
                        warehouse.remove(&target)
                    };

                    if prev_target_value.is_some() {
                        _ = removed.insert((target, prev_target_value));
                    }

                    if d_y == 0 {
                        continue;
                    }

                    match prev_target_value {
                        Some(Obstacle::BoxLeft) => {
                            let neighbour = [target[0], target[1] + 1];
                            _ = removed.insert((neighbour, warehouse.remove(&neighbour)));
                        }
                        Some(Obstacle::BoxRight) => {
                            let neighbour = [target[0], target[1].saturating_sub(1)];
                            _ = removed.insert((neighbour, warehouse.remove(&neighbour)));
                        }
                        Some(Obstacle::Box | Obstacle::Wall) | None => {}
                    }
                }

                (new_robot, warehouse)
            },
        );

        warehouse
            .iter()
            .filter_map(|([y, x], obstacle)| match obstacle {
                Obstacle::BoxLeft => Some((100 * y) + x),
                Obstacle::Wall | Obstacle::Box | Obstacle::BoxRight => None,
            })
            .sum()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Obstacle {
    Box,
    Wall,
    BoxLeft,
    BoxRight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[cfg(test)]
mod day_15_tests {
    use std::collections::HashMap;

    use super::*;

    const DATA_DIR: &str = "../../data";

    mod from_data {
        use super::*;

        struct Case<'c> {
            input: &'c str,
            expected: Input,
        }

        #[test]
        fn smaller_example() {
            run(&Case {
                input: super::smaller_example().0,
                expected: super::smaller_example().1,
            });
        }

        #[test]
        fn doubled_example() {
            run(&Case {
                input: super::doubled_example().0,
                expected: super::doubled_example().1,
            });
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
        fn smaller_example() {
            run(&Case {
                data: super::smaller_example().1,
                expected: 2028,
            });
        }

        #[test]
        fn example() {
            run(&Case {
                data: super::example().1,
                expected: 10092,
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
        fn doubled_example() {
            run(&Case {
                data: super::doubled_example().1,
                expected: 618,
            });
        }

        #[test]
        fn example() {
            run(&Case {
                data: super::example().1,
                expected: 9021,
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

    fn smaller_example() -> (&'static str, Input) {
        (
            "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<",
            Input {
                warehouse: HashMap::from([
                    ([0, 0], Obstacle::Wall),
                    ([0, 1], Obstacle::Wall),
                    ([0, 2], Obstacle::Wall),
                    ([0, 3], Obstacle::Wall),
                    ([0, 4], Obstacle::Wall),
                    ([0, 5], Obstacle::Wall),
                    ([0, 6], Obstacle::Wall),
                    ([0, 7], Obstacle::Wall),
                    ([1, 0], Obstacle::Wall),
                    ([1, 3], Obstacle::Box),
                    ([1, 5], Obstacle::Box),
                    ([1, 7], Obstacle::Wall),
                    ([2, 0], Obstacle::Wall),
                    ([2, 1], Obstacle::Wall),
                    ([2, 4], Obstacle::Box),
                    ([2, 7], Obstacle::Wall),
                    ([3, 0], Obstacle::Wall),
                    ([3, 4], Obstacle::Box),
                    ([3, 7], Obstacle::Wall),
                    ([4, 0], Obstacle::Wall),
                    ([4, 2], Obstacle::Wall),
                    ([4, 4], Obstacle::Box),
                    ([4, 7], Obstacle::Wall),
                    ([5, 0], Obstacle::Wall),
                    ([5, 4], Obstacle::Box),
                    ([5, 7], Obstacle::Wall),
                    ([6, 0], Obstacle::Wall),
                    ([6, 7], Obstacle::Wall),
                    ([7, 0], Obstacle::Wall),
                    ([7, 1], Obstacle::Wall),
                    ([7, 2], Obstacle::Wall),
                    ([7, 3], Obstacle::Wall),
                    ([7, 4], Obstacle::Wall),
                    ([7, 5], Obstacle::Wall),
                    ([7, 6], Obstacle::Wall),
                    ([7, 7], Obstacle::Wall),
                ]),
                robot: [2, 2],
                instructions: vec![
                    Direction::Left,
                    Direction::Up,
                    Direction::Up,
                    Direction::Right,
                    Direction::Right,
                    Direction::Right,
                    Direction::Down,
                    Direction::Down,
                    Direction::Left,
                    Direction::Down,
                    Direction::Right,
                    Direction::Right,
                    Direction::Down,
                    Direction::Left,
                    Direction::Left,
                ],
            },
        )
    }

    fn doubled_example() -> (&'static str, Input) {
        (
            "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^",
            Input {
                warehouse: HashMap::from([
                    ([0, 0], Obstacle::Wall),
                    ([0, 1], Obstacle::Wall),
                    ([0, 2], Obstacle::Wall),
                    ([0, 3], Obstacle::Wall),
                    ([0, 4], Obstacle::Wall),
                    ([0, 5], Obstacle::Wall),
                    ([0, 6], Obstacle::Wall),
                    ([1, 0], Obstacle::Wall),
                    ([1, 4], Obstacle::Wall),
                    ([1, 6], Obstacle::Wall),
                    ([2, 0], Obstacle::Wall),
                    ([2, 6], Obstacle::Wall),
                    ([3, 0], Obstacle::Wall),
                    ([3, 3], Obstacle::Box),
                    ([3, 4], Obstacle::Box),
                    ([3, 6], Obstacle::Wall),
                    ([4, 0], Obstacle::Wall),
                    ([4, 3], Obstacle::Box),
                    ([4, 6], Obstacle::Wall),
                    ([5, 0], Obstacle::Wall),
                    ([5, 6], Obstacle::Wall),
                    ([6, 0], Obstacle::Wall),
                    ([6, 1], Obstacle::Wall),
                    ([6, 2], Obstacle::Wall),
                    ([6, 3], Obstacle::Wall),
                    ([6, 4], Obstacle::Wall),
                    ([6, 5], Obstacle::Wall),
                    ([6, 6], Obstacle::Wall),
                ]),
                robot: [3, 5],
                instructions: vec![
                    Direction::Left,
                    Direction::Down,
                    Direction::Down,
                    Direction::Left,
                    Direction::Left,
                    Direction::Up,
                    Direction::Up,
                    Direction::Left,
                    Direction::Left,
                    Direction::Up,
                    Direction::Up,
                ],
            },
        )
    }

    #[expect(clippy::too_many_lines)]
    fn example() -> (&'static str, Input) {
        (
            "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^",
            Input {
                warehouse: HashMap::from([
                    ([0, 0], Obstacle::Wall),
                    ([0, 1], Obstacle::Wall),
                    ([0, 2], Obstacle::Wall),
                    ([0, 3], Obstacle::Wall),
                    ([0, 4], Obstacle::Wall),
                    ([0, 5], Obstacle::Wall),
                    ([0, 6], Obstacle::Wall),
                    ([0, 7], Obstacle::Wall),
                    ([0, 8], Obstacle::Wall),
                    ([0, 9], Obstacle::Wall),
                    ([1, 0], Obstacle::Wall),
                    ([1, 3], Obstacle::Box),
                    ([1, 6], Obstacle::Box),
                    ([1, 8], Obstacle::Box),
                    ([1, 9], Obstacle::Wall),
                    ([2, 0], Obstacle::Wall),
                    ([2, 7], Obstacle::Box),
                    ([2, 9], Obstacle::Wall),
                    ([3, 0], Obstacle::Wall),
                    ([3, 2], Obstacle::Box),
                    ([3, 3], Obstacle::Box),
                    ([3, 6], Obstacle::Box),
                    ([3, 8], Obstacle::Box),
                    ([3, 9], Obstacle::Wall),
                    ([4, 0], Obstacle::Wall),
                    ([4, 3], Obstacle::Box),
                    ([4, 7], Obstacle::Box),
                    ([4, 9], Obstacle::Wall),
                    ([5, 0], Obstacle::Wall),
                    ([5, 1], Obstacle::Box),
                    ([5, 2], Obstacle::Wall),
                    ([5, 5], Obstacle::Box),
                    ([5, 9], Obstacle::Wall),
                    ([6, 0], Obstacle::Wall),
                    ([6, 1], Obstacle::Box),
                    ([6, 4], Obstacle::Box),
                    ([6, 7], Obstacle::Box),
                    ([6, 9], Obstacle::Wall),
                    ([7, 0], Obstacle::Wall),
                    ([7, 2], Obstacle::Box),
                    ([7, 3], Obstacle::Box),
                    ([7, 5], Obstacle::Box),
                    ([7, 7], Obstacle::Box),
                    ([7, 8], Obstacle::Box),
                    ([7, 9], Obstacle::Wall),
                    ([8, 0], Obstacle::Wall),
                    ([8, 5], Obstacle::Box),
                    ([8, 9], Obstacle::Wall),
                    ([9, 0], Obstacle::Wall),
                    ([9, 1], Obstacle::Wall),
                    ([9, 2], Obstacle::Wall),
                    ([9, 3], Obstacle::Wall),
                    ([9, 4], Obstacle::Wall),
                    ([9, 5], Obstacle::Wall),
                    ([9, 6], Obstacle::Wall),
                    ([9, 7], Obstacle::Wall),
                    ([9, 8], Obstacle::Wall),
                    ([9, 9], Obstacle::Wall),
                ]),
                robot: [4, 4],
                instructions: vec![
                    Direction::Left,
                    Direction::Down,
                    Direction::Down,
                    Direction::Right,
                    Direction::Up,
                    Direction::Left,
                    Direction::Down,
                    Direction::Up,
                    Direction::Right,
                    Direction::Down,
                    Direction::Right,
                    Direction::Up,
                    Direction::Down,
                    Direction::Down,
                    Direction::Up,
                    Direction::Down,
                    Direction::Right,
                    Direction::Down,
                    Direction::Left,
                    Direction::Right,
                    Direction::Down,
                    Direction::Up,
                    Direction::Down,
                    Direction::Left,
                    Direction::Down,
                    Direction::Left,
                    Direction::Up,
                    Direction::Down,
                    Direction::Down,
                    Direction::Left,
                    Direction::Left,
                    Direction::Left,
                    Direction::Up,
                    Direction::Right,
                    Direction::Left,
                    Direction::Left,
                    Direction::Right,
                    Direction::Left,
                    Direction::Right,
                    Direction::Right,
                    Direction::Down,
                    Direction::Left,
                    Direction::Down,
                    Direction::Down,
                    Direction::Down,
                    Direction::Left,
                    Direction::Right,
                    Direction::Up,
                    Direction::Down,
                    Direction::Up,
                    Direction::Right,
                    Direction::Up,
                    Direction::Left,
                    Direction::Left,
                    Direction::Left,
                    Direction::Right,
                    Direction::Left,
                    Direction::Left,
                    Direction::Down,
                    Direction::Left,
                    Direction::Left,
                    Direction::Left,
                    Direction::Down,
                    Direction::Up,
                    Direction::Down,
                    Direction::Down,
                    Direction::Up,
                    Direction::Down,
                    Direction::Right,
                    Direction::Up,
                    Direction::Down,
                    Direction::Down,
                    Direction::Down,
                    Direction::Left,
                    Direction::Left,
                    Direction::Up,
                    Direction::Right,
                    Direction::Up,
                    Direction::Down,
                    Direction::Up,
                    Direction::Up,
                    Direction::Right,
                    Direction::Left,
                    Direction::Left,
                    Direction::Right,
                    Direction::Right,
                    Direction::Right,
                    Direction::Left,
                    Direction::Right,
                    Direction::Up,
                    Direction::Left,
                    Direction::Left,
                    Direction::Right,
                    Direction::Left,
                    Direction::Up,
                    Direction::Down,
                    Direction::Down,
                    Direction::Up,
                    Direction::Up,
                    Direction::Left,
                    Direction::Right,
                    Direction::Down,
                    Direction::Down,
                    Direction::Down,
                    Direction::Left,
                    Direction::Right,
                    Direction::Right,
                    Direction::Left,
                    Direction::Up,
                    Direction::Up,
                    Direction::Down,
                    Direction::Right,
                    Direction::Up,
                    Direction::Right,
                    Direction::Down,
                    Direction::Down,
                    Direction::Left,
                    Direction::Right,
                    Direction::Down,
                    Direction::Left,
                    Direction::Left,
                    Direction::Left,
                    Direction::Left,
                    Direction::Down,
                    Direction::Left,
                    Direction::Up,
                    Direction::Down,
                    Direction::Right,
                    Direction::Up,
                    Direction::Left,
                    Direction::Up,
                    Direction::Up,
                    Direction::Right,
                    Direction::Right,
                    Direction::Right,
                    Direction::Up,
                    Direction::Left,
                    Direction::Down,
                    Direction::Left,
                    Direction::Down,
                    Direction::Right,
                    Direction::Left,
                    Direction::Right,
                    Direction::Down,
                    Direction::Down,
                    Direction::Right,
                    Direction::Down,
                    Direction::Up,
                    Direction::Down,
                    Direction::Up,
                    Direction::Left,
                    Direction::Right,
                    Direction::Right,
                    Direction::Left,
                    Direction::Right,
                    Direction::Right,
                    Direction::Right,
                    Direction::Right,
                    Direction::Left,
                    Direction::Up,
                    Direction::Up,
                    Direction::Right,
                    Direction::Down,
                    Direction::Down,
                    Direction::Right,
                    Direction::Down,
                    Direction::Left,
                    Direction::Up,
                    Direction::Up,
                    Direction::Up,
                    Direction::Right,
                    Direction::Right,
                    Direction::Down,
                    Direction::Up,
                    Direction::Down,
                    Direction::Up,
                    Direction::Left,
                    Direction::Up,
                    Direction::Up,
                    Direction::Right,
                    Direction::Down,
                    Direction::Up,
                    Direction::Up,
                    Direction::Right,
                    Direction::Down,
                    Direction::Up,
                    Direction::Left,
                    Direction::Up,
                    Direction::Down,
                    Direction::Right,
                    Direction::Down,
                    Direction::Left,
                    Direction::Right,
                    Direction::Right,
                    Direction::Down,
                    Direction::Up,
                    Direction::Down,
                    Direction::Up,
                    Direction::Left,
                    Direction::Down,
                    Direction::Right,
                    Direction::Down,
                    Direction::Up,
                    Direction::Up,
                    Direction::Left,
                    Direction::Up,
                    Direction::Up,
                    Direction::Down,
                    Direction::Down,
                    Direction::Left,
                    Direction::Left,
                    Direction::Left,
                    Direction::Down,
                    Direction::Left,
                    Direction::Up,
                    Direction::Right,
                    Direction::Right,
                    Direction::Up,
                    Direction::Up,
                    Direction::Up,
                    Direction::Up,
                    Direction::Right,
                    Direction::Right,
                    Direction::Right,
                    Direction::Down,
                    Direction::Up,
                    Direction::Left,
                    Direction::Right,
                    Direction::Down,
                    Direction::Down,
                    Direction::Down,
                    Direction::Up,
                    Direction::Right,
                    Direction::Left,
                    Direction::Down,
                    Direction::Left,
                    Direction::Left,
                    Direction::Left,
                    Direction::Right,
                    Direction::Up,
                    Direction::Up,
                    Direction::Up,
                    Direction::Down,
                    Direction::Down,
                    Direction::Up,
                    Direction::Left,
                    Direction::Down,
                    Direction::Down,
                    Direction::Down,
                    Direction::Right,
                    Direction::Up,
                    Direction::Right,
                    Direction::Down,
                    Direction::Left,
                    Direction::Up,
                    Direction::Up,
                    Direction::Up,
                    Direction::Up,
                    Direction::Down,
                    Direction::Left,
                    Direction::Right,
                    Direction::Up,
                    Direction::Right,
                    Direction::Down,
                    Direction::Down,
                    Direction::Down,
                    Direction::Down,
                    Direction::Right,
                    Direction::Left,
                    Direction::Right,
                    Direction::Right,
                    Direction::Down,
                    Direction::Up,
                    Direction::Left,
                    Direction::Left,
                    Direction::Up,
                    Direction::Up,
                    Direction::Up,
                    Direction::Up,
                    Direction::Up,
                    Direction::Up,
                    Direction::Right,
                    Direction::Left,
                    Direction::Up,
                    Direction::Right,
                    Direction::Left,
                    Direction::Right,
                    Direction::Right,
                    Direction::Right,
                    Direction::Left,
                    Direction::Right,
                    Direction::Up,
                    Direction::Up,
                    Direction::Left,
                    Direction::Left,
                    Direction::Up,
                    Direction::Up,
                    Direction::Down,
                    Direction::Right,
                    Direction::Right,
                    Direction::Right,
                    Direction::Left,
                    Direction::Up,
                    Direction::Left,
                    Direction::Down,
                    Direction::Right,
                    Direction::Up,
                    Direction::Left,
                    Direction::Down,
                    Direction::Down,
                    Direction::Right,
                    Direction::Right,
                    Direction::Down,
                    Direction::Right,
                    Direction::Right,
                    Direction::Right,
                    Direction::Up,
                    Direction::Down,
                    Direction::Right,
                    Direction::Left,
                    Direction::Right,
                    Direction::Up,
                    Direction::Down,
                    Direction::Right,
                    Direction::Left,
                    Direction::Left,
                    Direction::Left,
                    Direction::Left,
                    Direction::Down,
                    Direction::Right,
                    Direction::Right,
                    Direction::Down,
                    Direction::Left,
                    Direction::Down,
                    Direction::Left,
                    Direction::Down,
                    Direction::Right,
                    Direction::Down,
                    Direction::Down,
                    Direction::Down,
                    Direction::Right,
                    Direction::Up,
                    Direction::Left,
                    Direction::Right,
                    Direction::Left,
                    Direction::Left,
                    Direction::Right,
                    Direction::Up,
                    Direction::Right,
                    Direction::Left,
                    Direction::Up,
                    Direction::Right,
                    Direction::Right,
                    Direction::Left,
                    Direction::Right,
                    Direction::Up,
                    Direction::Down,
                    Direction::Left,
                    Direction::Right,
                    Direction::Left,
                    Direction::Up,
                    Direction::Down,
                    Direction::Down,
                    Direction::Down,
                    Direction::Left,
                    Direction::Up,
                    Direction::Up,
                    Direction::Left,
                    Direction::Right,
                    Direction::Left,
                    Direction::Down,
                    Direction::Left,
                    Direction::Left,
                    Direction::Left,
                    Direction::Left,
                    Direction::Left,
                    Direction::Right,
                    Direction::Left,
                    Direction::Up,
                    Direction::Down,
                    Direction::Left,
                    Direction::Left,
                    Direction::Left,
                    Direction::Right,
                    Direction::Left,
                    Direction::Left,
                    Direction::Left,
                    Direction::Up,
                    Direction::Up,
                    Direction::Left,
                    Direction::Down,
                    Direction::Left,
                    Direction::Up,
                    Direction::Up,
                    Direction::Up,
                    Direction::Right,
                    Direction::Left,
                    Direction::Up,
                    Direction::Right,
                    Direction::Right,
                    Direction::Up,
                    Direction::Left,
                    Direction::Down,
                    Direction::Up,
                    Direction::Right,
                    Direction::Left,
                    Direction::Left,
                    Direction::Left,
                    Direction::Up,
                    Direction::Right,
                    Direction::Right,
                    Direction::Up,
                    Direction::Down,
                    Direction::Left,
                    Direction::Down,
                    Direction::Up,
                    Direction::Down,
                    Direction::Left,
                    Direction::Down,
                    Direction::Up,
                    Direction::Right,
                    Direction::Up,
                    Direction::Right,
                    Direction::Right,
                    Direction::Up,
                    Direction::Down,
                    Direction::Right,
                    Direction::Down,
                    Direction::Down,
                    Direction::Right,
                    Direction::Up,
                    Direction::Left,
                    Direction::Left,
                    Direction::Up,
                    Direction::Down,
                    Direction::Left,
                    Direction::Right,
                    Direction::Right,
                    Direction::Left,
                    Direction::Left,
                    Direction::Right,
                    Direction::Left,
                    Direction::Left,
                    Direction::Down,
                    Direction::Left,
                    Direction::Left,
                    Direction::Down,
                    Direction::Right,
                    Direction::Left,
                    Direction::Right,
                    Direction::Down,
                    Direction::Left,
                    Direction::Up,
                    Direction::Down,
                    Direction::Down,
                    Direction::Left,
                    Direction::Left,
                    Direction::Left,
                    Direction::Right,
                    Direction::Up,
                    Direction::Up,
                    Direction::Down,
                    Direction::Up,
                    Direction::Right,
                    Direction::Up,
                    Direction::Up,
                    Direction::Right,
                    Direction::Right,
                    Direction::Right,
                    Direction::Left,
                    Direction::Left,
                    Direction::Up,
                    Direction::Down,
                    Direction::Right,
                    Direction::Right,
                    Direction::Down,
                    Direction::Up,
                    Direction::Down,
                    Direction::Right,
                    Direction::Left,
                    Direction::Up,
                    Direction::Up,
                    Direction::Right,
                    Direction::Right,
                    Direction::Up,
                    Direction::Left,
                    Direction::Right,
                    Direction::Down,
                    Direction::Down,
                    Direction::Up,
                    Direction::Left,
                    Direction::Right,
                    Direction::Left,
                    Direction::Up,
                    Direction::Up,
                    Direction::Right,
                    Direction::Up,
                    Direction::Up,
                    Direction::Up,
                    Direction::Left,
                    Direction::Right,
                    Direction::Left,
                    Direction::Down,
                    Direction::Down,
                    Direction::Down,
                    Direction::Down,
                    Direction::Down,
                    Direction::Up,
                    Direction::Down,
                    Direction::Left,
                    Direction::Down,
                    Direction::Left,
                    Direction::Left,
                    Direction::Right,
                    Direction::Up,
                    Direction::Down,
                    Direction::Left,
                    Direction::Down,
                    Direction::Right,
                    Direction::Down,
                    Direction::Left,
                    Direction::Left,
                    Direction::Up,
                    Direction::Right,
                    Direction::Left,
                    Direction::Left,
                    Direction::Right,
                    Direction::Left,
                    Direction::Left,
                    Direction::Right,
                    Direction::Left,
                    Direction::Left,
                    Direction::Left,
                    Direction::Up,
                    Direction::Up,
                    Direction::Left,
                    Direction::Left,
                    Direction::Left,
                    Direction::Up,
                    Direction::Left,
                    Direction::Left,
                    Direction::Right,
                    Direction::Right,
                    Direction::Left,
                    Direction::Left,
                    Direction::Right,
                    Direction::Left,
                    Direction::Up,
                    Direction::Up,
                    Direction::Up,
                    Direction::Right,
                    Direction::Up,
                    Direction::Up,
                    Direction::Left,
                    Direction::Right,
                    Direction::Up,
                    Direction::Right,
                    Direction::Down,
                    Direction::Left,
                    Direction::Right,
                    Direction::Up,
                    Direction::Up,
                    Direction::Right,
                    Direction::Down,
                    Direction::Down,
                    Direction::Left,
                    Direction::Up,
                    Direction::Down,
                    Direction::Up,
                    Direction::Down,
                    Direction::Left,
                    Direction::Down,
                    Direction::Down,
                    Direction::Right,
                    Direction::Up,
                    Direction::Left,
                    Direction::Right,
                    Direction::Left,
                    Direction::Down,
                    Direction::Left,
                    Direction::Up,
                    Direction::Down,
                    Direction::Right,
                    Direction::Up,
                    Direction::Up,
                    Direction::Up,
                    Direction::Right,
                    Direction::Right,
                    Direction::Right,
                    Direction::Up,
                    Direction::Up,
                    Direction::Down,
                    Direction::Down,
                    Direction::Down,
                    Direction::Up,
                    Direction::Right,
                    Direction::Down,
                    Direction::Down,
                    Direction::Down,
                    Direction::Left,
                    Direction::Right,
                    Direction::Right,
                    Direction::Right,
                    Direction::Up,
                    Direction::Left,
                    Direction::Up,
                    Direction::Right,
                    Direction::Right,
                    Direction::Right,
                    Direction::Right,
                    Direction::Right,
                    Direction::Up,
                    Direction::Left,
                    Direction::Left,
                    Direction::Up,
                    Direction::Down,
                    Direction::Right,
                    Direction::Up,
                    Direction::Down,
                    Direction::Down,
                    Direction::Down,
                    Direction::Left,
                    Direction::Right,
                    Direction::Up,
                    Direction::Left,
                    Direction::Right,
                    Direction::Left,
                    Direction::Left,
                    Direction::Down,
                    Direction::Right,
                    Direction::Down,
                    Direction::Up,
                    Direction::Up,
                    Direction::Right,
                    Direction::Right,
                    Direction::Right,
                    Direction::Left,
                    Direction::Left,
                    Direction::Up,
                    Direction::Up,
                    Direction::Left,
                    Direction::Right,
                    Direction::Right,
                    Direction::Up,
                    Direction::Down,
                    Direction::Up,
                    Direction::Left,
                    Direction::Down,
                    Direction::Up,
                    Direction::Down,
                    Direction::Down,
                    Direction::Left,
                    Direction::Right,
                    Direction::Down,
                    Direction::Up,
                    Direction::Left,
                    Direction::Left,
                    Direction::Right,
                    Direction::Up,
                    Direction::Left,
                    Direction::Up,
                    Direction::Down,
                    Direction::Up,
                    Direction::Down,
                    Direction::Right,
                    Direction::Left,
                    Direction::Up,
                    Direction::Left,
                    Direction::Left,
                    Direction::Left,
                    Direction::Right,
                    Direction::Left,
                    Direction::Left,
                    Direction::Up,
                    Direction::Left,
                    Direction::Down,
                    Direction::Right,
                    Direction::Left,
                    Direction::Down,
                    Direction::Left,
                    Direction::Right,
                    Direction::Down,
                    Direction::Down,
                    Direction::Right,
                    Direction::Right,
                    Direction::Down,
                    Direction::Right,
                    Direction::Left,
                    Direction::Down,
                    Direction::Up,
                    Direction::Left,
                    Direction::Down,
                    Direction::Down,
                    Direction::Left,
                    Direction::Right,
                    Direction::Down,
                    Direction::Up,
                    Direction::Left,
                    Direction::Left,
                    Direction::Up,
                ],
            },
        )
    }
}
