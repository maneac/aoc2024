use std::{fs::read_to_string, path::Path};

pub const PART_1: usize = 26_810;
pub const PART_2: usize = 108_713_182_988_244;

#[must_use]
pub fn read_data(data_dir: &str) -> String {
    read_to_string(Path::new(data_dir).join("day_13.txt"))
        .unwrap()
        .trim()
        .to_owned()
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Input {
    machines: Vec<Machine>,
}

impl Input {
    #[must_use]
    pub fn from_data(data: &str) -> Self {
        let machines = data
            .trim()
            .split("\n\n")
            .map(|machine_chunk| {
                let mut lines = machine_chunk.lines();
                let a_str = lines.next().unwrap().strip_prefix("Button A: ").unwrap();

                let button_a = {
                    let (x_str, y_str) = a_str.split_once(", ").unwrap();
                    let a_x = x_str.strip_prefix("X+").unwrap().parse().unwrap();
                    let a_y = y_str.strip_prefix("Y+").unwrap().parse().unwrap();
                    [a_x, a_y]
                };

                let b_str = lines.next().unwrap().strip_prefix("Button B: ").unwrap();
                let button_b = {
                    let (x_str, y_str) = b_str.split_once(", ").unwrap();
                    let b_x = x_str.strip_prefix("X+").unwrap().parse().unwrap();
                    let b_y = y_str.strip_prefix("Y+").unwrap().parse().unwrap();
                    [b_x, b_y]
                };

                let prize_str = lines.next().unwrap().strip_prefix("Prize: ").unwrap();
                let prize = {
                    let (x_str, y_str) = prize_str.split_once(", ").unwrap();
                    let prize_x = x_str.strip_prefix("X=").unwrap().parse().unwrap();
                    let prize_y = y_str.strip_prefix("Y=").unwrap().parse().unwrap();
                    [prize_x, prize_y]
                };

                Machine {
                    button_a,
                    button_b,
                    prize,
                }
            })
            .collect();

        Self { machines }
    }

    #[must_use]
    pub fn part_1(&self) -> usize {
        self.machines.iter().filter_map(tokens_for_prize).sum()
    }

    #[must_use]
    pub fn part_2(&self) -> usize {
        self.machines
            .iter()
            .filter_map(|machine| {
                tokens_for_prize(&Machine {
                    button_a: machine.button_a,
                    button_b: machine.button_b,
                    prize: [
                        machine.prize[0] + 10_000_000_000_000,
                        machine.prize[1] + 10_000_000_000_000,
                    ],
                })
            })
            .sum()
    }
}

#[expect(clippy::similar_names)]
fn tokens_for_prize(machine: &Machine) -> Option<usize> {
    /*
    [xa xb][a] = [tx]
    [ya yb][b] = [ty]

    [a] = 1/          [yb -xb][tx]
    [b] = xa.yb-xb.ya [-ya xa][ty]

    [x] = 1/          [ yb.tx - xb.ty]
    [y] = xa.yb-xb.ya [-ya.tx + xa.ty]

    x = (yb.tx - xb.ty) / (xa.yb-xb.ya)
    y = (xa.ty - ya.tx) / (xa.yb-xb.ya)

    // OR

    [ya yb][b] = [ty]
    [xa xb][a] = [tx]

    [x] = 1/          [xb -yb][ty]
    [y] = xb.ya-xa.yb [-xa ya][tx]

    [x] = 1/          [ xb.ty - yb.tx]
    [y] = xb.ya-xa.yb [-xa.ty + ya.tx]

    x = (xb.ty - yb.tx) / (xb.ya-xa.yb)
    y = (ya.tx - xa.ty) / (xb.ya-xa.yb)
    */

    let xa_yb = machine.button_a[0] * machine.button_b[1];
    let xb_ya = machine.button_b[0] * machine.button_a[1];

    let xa_ty = machine.button_a[0] * machine.prize[1];
    let xb_ty = machine.button_b[0] * machine.prize[1];
    let ya_tx = machine.button_a[1] * machine.prize[0];
    let yb_tx = machine.button_b[1] * machine.prize[0];

    assert_ne!(xa_yb, xb_ya, "Cannot divide by a discriminant of zero");

    let (x_mul, y_mul, discriminant) = if xb_ya < xa_yb && xb_ty < yb_tx && ya_tx < xa_ty {
        (yb_tx - xb_ty, xa_ty - ya_tx, xa_yb - xb_ya)
    } else if xb_ya >= xa_yb && xb_ty >= yb_tx && ya_tx >= xa_ty {
        (xb_ty - yb_tx, ya_tx - xa_ty, xb_ya - xa_yb)
    } else {
        return None;
    };

    (x_mul % discriminant == 0 && y_mul % discriminant == 0)
        .then_some((3 * x_mul.div_euclid(discriminant)) + y_mul.div_euclid(discriminant))
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct Machine {
    button_a: [usize; 2],
    button_b: [usize; 2],
    prize: [usize; 2],
}

#[cfg(test)]
mod day_13_tests {
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
                expected: 480,
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
            "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279",
            Input {
                machines: vec![
                    Machine {
                        button_a: [94, 34],
                        button_b: [22, 67],
                        prize: [8400, 5400],
                    },
                    Machine {
                        button_a: [26, 66],
                        button_b: [67, 21],
                        prize: [12748, 12176],
                    },
                    Machine {
                        button_a: [17, 86],
                        button_b: [84, 37],
                        prize: [7870, 6450],
                    },
                    Machine {
                        button_a: [69, 23],
                        button_b: [27, 71],
                        prize: [18641, 10279],
                    },
                ],
            },
        )
    }
}
