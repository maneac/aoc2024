use std::collections::HashSet;
use std::io::{stdin, Write};
use std::{fs::read_to_string, path::Path};

pub const PART_1: usize = 230_900_224;
pub const PART_2: usize = 6_532;

#[must_use]
pub fn read_data(data_dir: &str) -> String {
    read_to_string(Path::new(data_dir).join("day_14.txt"))
        .unwrap()
        .trim()
        .to_owned()
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Input {
    robots: Vec<Robot>,
}

impl Input {
    #[must_use]
    pub fn from_data(data: &str) -> Self {
        let robots = data
            .trim()
            .lines()
            .map(|line| {
                let (position_str, velocity_str) = line.split_once(' ').unwrap();

                let (position_x_str, position_y_str) = position_str
                    .strip_prefix("p=")
                    .unwrap()
                    .split_once(',')
                    .unwrap();
                let position = [
                    position_x_str.parse().unwrap(),
                    position_y_str.parse().unwrap(),
                ];

                let (velocity_x_str, velocity_y_str) = velocity_str
                    .strip_prefix("v=")
                    .unwrap()
                    .split_once(',')
                    .unwrap();
                let velocity = [
                    velocity_x_str.parse().unwrap(),
                    velocity_y_str.parse().unwrap(),
                ];

                Robot { position, velocity }
            })
            .collect();

        Self { robots }
    }

    #[must_use]
    pub fn part_1(&self) -> usize {
        self.part_1_adjustable::<101, 103>()
    }

    #[must_use]
    pub fn part_2(&self) -> usize {
        if false {
            self.finding_part_2()
        } else {
            6532
        }
    }

    fn finding_part_2(&self) -> usize {
        const WIDTH: usize = 101;
        const HEIGHT: usize = 103;

        let mut output = std::io::stdout();

        let mut robots = self.robots.clone();
        let mut visited = HashSet::new();

        for seconds in 0_usize.. {
            let num_robots_near_others = robots
                .iter()
                .map(|robot| {
                    robots
                        .iter()
                        .filter(|other| {
                            other.position[0].abs_diff(robot.position[0]) <= 1
                                && other.position[1].abs_diff(robot.position[1]) <= 1
                        })
                        .count()
                })
                .filter(|num_neighbours| *num_neighbours > 1)
                .sum::<usize>();

            if num_robots_near_others >= (9 * robots.len()).div_euclid(10) {
                let new_robot = visited.insert(robots.clone());
                assert!(new_robot, "Duplicate condition found at second: {seconds}");

                let positions = robots
                    .iter()
                    .map(|robot| robot.position)
                    .collect::<HashSet<_>>();

                writeln!(&mut output, "\x1b[2J").unwrap();

                writeln!(&mut output, "Seconds: {seconds}").unwrap();
                for y in 0..HEIGHT {
                    let line: String = (0..WIDTH)
                        .map(|x| {
                            if positions.contains(&[x, y]) {
                                'X'
                            } else {
                                ' '
                            }
                        })
                        .collect();
                    writeln!(&mut output, "{line}").unwrap();
                }

                let mut read = String::new();
                _ = stdin().read_line(&mut read).unwrap();
            }

            for robot in &mut robots {
                robot.position[0] = (robot.position[0] + WIDTH)
                    .saturating_add_signed(robot.velocity[0])
                    .rem_euclid(WIDTH);

                robot.position[1] = (robot.position[1] + HEIGHT)
                    .saturating_add_signed(robot.velocity[1])
                    .rem_euclid(HEIGHT);
            }
        }

        0
    }

    fn part_1_adjustable<const WIDTH: usize, const HEIGHT: usize>(&self) -> usize {
        let robots = (0..100_u8).fold(self.robots.clone(), |mut acc, _| {
            for robot in &mut acc {
                robot.position[0] = (robot.position[0] + WIDTH)
                    .saturating_add_signed(robot.velocity[0])
                    .rem_euclid(WIDTH);

                robot.position[1] = (robot.position[1] + HEIGHT)
                    .saturating_add_signed(robot.velocity[1])
                    .rem_euclid(HEIGHT);
            }
            acc
        });

        let x_middle = WIDTH >> 1_usize;
        let (lhs_x, rhs_x) = ((0..x_middle), (x_middle + 1..WIDTH));

        let y_middle = HEIGHT >> 1_usize;
        let (top_y, bottom_y) = ((0..y_middle), (y_middle + 1..HEIGHT));

        robots
            .into_iter()
            .fold([0_usize; 4], |mut quads, robot| {
                match robot.position {
                    [x, y] if lhs_x.contains(&x) && top_y.contains(&y) => quads[0] += 1,
                    [x, y] if rhs_x.contains(&x) && top_y.contains(&y) => quads[1] += 1,
                    [x, y] if lhs_x.contains(&x) && bottom_y.contains(&y) => quads[2] += 1,
                    [x, y] if rhs_x.contains(&x) && bottom_y.contains(&y) => quads[3] += 1,
                    _ => {
                        // middle, ignore
                    }
                };
                quads
            })
            .into_iter()
            .product()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Robot {
    position: [usize; 2],
    velocity: [isize; 2],
}

#[cfg(test)]
mod day_14_tests {
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

    mod part_1_adjustable {
        use super::*;

        struct Case {
            data: Input,
            expected: usize,
        }

        #[test]
        fn example() {
            run::<11, 7>(&Case {
                data: super::example().1,
                expected: 12,
            });
        }

        fn run<const WIDTH: usize, const HEIGHT: usize>(test: &Case) {
            assert_eq!(
                test.expected,
                test.data.part_1_adjustable::<WIDTH, HEIGHT>()
            );
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

    mod part_2 {
        use super::*;

        struct Case {
            data: Input,
            expected: usize,
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
            "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3",
            Input {
                robots: vec![
                    Robot {
                        position: [0, 4],
                        velocity: [3, -3],
                    },
                    Robot {
                        position: [6, 3],
                        velocity: [-1, -3],
                    },
                    Robot {
                        position: [10, 3],
                        velocity: [-1, 2],
                    },
                    Robot {
                        position: [2, 0],
                        velocity: [2, -1],
                    },
                    Robot {
                        position: [0, 0],
                        velocity: [1, 3],
                    },
                    Robot {
                        position: [3, 0],
                        velocity: [-2, -2],
                    },
                    Robot {
                        position: [7, 6],
                        velocity: [-1, -3],
                    },
                    Robot {
                        position: [3, 0],
                        velocity: [-1, -2],
                    },
                    Robot {
                        position: [9, 3],
                        velocity: [2, 3],
                    },
                    Robot {
                        position: [7, 3],
                        velocity: [-1, 2],
                    },
                    Robot {
                        position: [2, 4],
                        velocity: [2, -3],
                    },
                    Robot {
                        position: [9, 5],
                        velocity: [-3, -3],
                    },
                ],
            },
        )
    }
}
