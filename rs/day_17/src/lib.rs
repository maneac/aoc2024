use std::{fs::read_to_string, path::Path};

use rayon::prelude::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

pub const PART_1: &str = "6,5,4,7,1,6,0,3,1";
pub const PART_2: usize = 0;

#[must_use]
pub fn read_data(data_dir: &str) -> String {
    read_to_string(Path::new(data_dir).join("day_17.txt"))
        .unwrap()
        .trim()
        .to_owned()
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Input {
    registers: [usize; 3],
    instructions: Vec<u8>,
}

impl Input {
    #[must_use]
    pub fn from_data(data: &str) -> Self {
        let (register_str, program) = data.trim().split_once("\n\n").unwrap();

        let register_values = register_str
            .trim()
            .lines()
            .map(|line| line.split_once(": ").unwrap().1.parse().unwrap())
            .collect::<Vec<_>>();

        let registers = core::array::from_fn(|idx| {
            #[expect(clippy::indexing_slicing)]
            register_values[idx]
        });

        let instructions = program
            .trim()
            .strip_prefix("Program: ")
            .unwrap()
            .split(',')
            .map(|num| num.parse().unwrap())
            .collect();

        Self {
            registers,
            instructions,
        }
    }

    #[must_use]
    pub fn part_1(&self) -> String {
        ProgramIterator::from(self).fold(String::new(), |mut acc, val| {
            if !acc.is_empty() {
                acc.push(',');
            }
            acc.push_str(&val.to_string());
            acc
        })
    }

    #[must_use]
    pub fn part_2(&self) -> usize {
        (0..usize::MAX)
            // .into_par_iter()
            // .by_uniform_blocks(100_000)
            // .find_first(|&register_a| {
            .find(|&register_a| {
                let prog_iter = ProgramIterator {
                    registers: [register_a, self.registers[1], self.registers[2]],
                    instruction_ptr: 0,
                    instructions: &self.instructions,
                };

                prog_iter
                    .zip(self.instructions.iter().copied())
                    .take_while(|(lhs, rhs)| lhs == rhs)
                    .count()
                    == self.instructions.len()
            })
            .unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProgramIterator<'i> {
    registers: [usize; 3],
    instruction_ptr: usize,
    instructions: &'i [u8],
}

impl<'i> From<&'i Input> for ProgramIterator<'i> {
    fn from(value: &'i Input) -> Self {
        Self {
            registers: value.registers,
            instruction_ptr: 0,
            instructions: &value.instructions,
        }
    }
}

impl Iterator for ProgramIterator<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let instruction = self
                .instructions
                .get(self.instruction_ptr)
                .and_then(|&opcode| {
                    self.instructions
                        .get(self.instruction_ptr + 1)
                        .map(|&operand| Instruction::from([opcode, operand]))
                })?;

            match instruction {
                Instruction::Adv(combo_operand) => {
                    self.registers[0] >>= combo_operand.value(&self.registers);
                }
                Instruction::Bxl(literal_operand) => {
                    self.registers[1] ^= usize::from(literal_operand.0);
                }
                Instruction::Bst(combo_operand) => {
                    self.registers[1] = combo_operand.value(&self.registers) & 7;
                }
                Instruction::Jnz(literal_operand) => {
                    if self.registers[0] != 0 {
                        self.instruction_ptr = usize::from(literal_operand.0);
                        continue;
                    }
                }
                Instruction::Bxc => {
                    self.registers[1] ^= self.registers[2];
                }
                Instruction::Out(combo_operand) => {
                    self.instruction_ptr += 2;
                    return u8::try_from(combo_operand.value(&self.registers) & 7).ok();
                }
                Instruction::Bdv(combo_operand) => {
                    self.registers[1] = self.registers[0] >> combo_operand.value(&self.registers);
                }
                Instruction::Cdv(combo_operand) => {
                    self.registers[2] = self.registers[0] >> combo_operand.value(&self.registers);
                }
            }

            self.instruction_ptr += 2;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Instruction {
    Adv(ComboOperand),
    Bxl(LiteralOperand),
    Bst(ComboOperand),
    Jnz(LiteralOperand),
    Bxc,
    Out(ComboOperand),
    Bdv(ComboOperand),
    Cdv(ComboOperand),
}

impl From<[u8; 2]> for Instruction {
    fn from(value: [u8; 2]) -> Self {
        let [operator, operand] = value;

        debug_assert!(
            (0..=7).contains(&operator),
            "Invalid operator value for instruction: {operator:?}"
        );
        debug_assert!(
            (0..=7).contains(&operand),
            "Invalid operand value for instruction: {operand:?}"
        );

        match operator {
            0 => Self::Adv(ComboOperand::from(operand)),
            1 => Self::Bxl(LiteralOperand::from(operand)),
            2 => Self::Bst(ComboOperand::from(operand)),
            3 => Self::Jnz(LiteralOperand::from(operand)),
            4 => Self::Bxc,
            5 => Self::Out(ComboOperand::from(operand)),
            6 => Self::Bdv(ComboOperand::from(operand)),
            7 => Self::Cdv(ComboOperand::from(operand)),
            #[expect(clippy::unreachable)]
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct LiteralOperand(u8);

impl From<u8> for LiteralOperand {
    fn from(value: u8) -> Self {
        debug_assert!(
            (0..=7).contains(&value),
            "Invalid literal operand value: {value:?}"
        );
        Self(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum ComboOperand {
    Zero,
    One,
    Two,
    Three,
    RegisterA,
    RegisterB,
    RegisterC,
}

impl ComboOperand {
    const fn value(self, registers: &[usize; 3]) -> usize {
        match self {
            Self::Zero => 0,
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::RegisterA => registers[0],
            Self::RegisterB => registers[1],
            Self::RegisterC => registers[2],
        }
    }
}

impl From<u8> for ComboOperand {
    fn from(value: u8) -> Self {
        assert!(
            (0..7).contains(&value),
            "Invalid combo operand value: {value:?}"
        );
        match value {
            0 => Self::Zero,
            1 => Self::One,
            2 => Self::Two,
            3 => Self::Three,
            4 => Self::RegisterA,
            5 => Self::RegisterB,
            6 => Self::RegisterC,
            #[expect(clippy::unreachable)]
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod day_17_tests {
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

        struct Case<'c> {
            data: Input,
            expected: &'c str,
        }

        #[test]
        fn example_1() {
            run(&Case {
                data: super::example_1().1,
                expected: "4,6,3,5,6,3,5,2,1,0",
            });
        }

        #[test]
        fn actual() {
            run(&Case {
                data: Input::from_data(&read_data(DATA_DIR)),
                expected: PART_1,
            });
        }

        fn run(test: &Case<'_>) {
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
        fn example_2() {
            run(&Case {
                data: super::example_2().1,
                expected: 117_440,
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
            "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0",
            Input {
                registers: [729, 0, 0],
                instructions: vec![0, 1, 5, 4, 3, 0],
            },
        )
    }

    fn example_2() -> (&'static str, Input) {
        (
            "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0",
            Input {
                registers: [2024, 0, 0],
                instructions: vec![0, 3, 5, 4, 3, 0],
            },
        )
    }
}
