use core::cmp::Ordering;
use std::{fs::read_to_string, path::Path};

pub const PART_1: usize = 5091;
pub const PART_2: usize = 4681;

#[must_use]
pub fn read_data(data_dir: &str) -> String {
    read_to_string(Path::new(data_dir).join("day_05.txt"))
        .unwrap()
        .trim()
        .to_owned()
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Input {
    page_ordering_rules: Vec<[u8; 2]>,
    pages: Vec<Vec<u8>>,
}

impl Input {
    #[must_use]
    pub fn from_data(data: &str) -> Self {
        let (rules, pages) = data.trim().split_once("\n\n").unwrap();

        let mut page_ordering_rules: Vec<[u8; 2]> = rules
            .lines()
            .map(|line| {
                let (lhs, rhs) = line.split_once('|').unwrap();
                [lhs.parse().unwrap(), rhs.parse().unwrap()]
            })
            .collect();
        page_ordering_rules.sort_unstable();

        let pages = pages
            .lines()
            .map(|line| line.split(',').map(|num| num.parse().unwrap()).collect())
            .collect();

        Self {
            page_ordering_rules,
            pages,
        }
    }

    #[must_use]
    pub fn part_1(&self) -> usize {
        self.pages
            .iter()
            .filter_map(|page| {
                self.page_ordering_rules
                    .iter()
                    .all(|&[lhs, rhs]| {
                        page.iter()
                            .position(|entry| lhs.eq(entry))
                            .and_then(|left_idx| {
                                page.iter()
                                    .position(|entry| rhs.eq(entry))
                                    .map(|rhs_idx| [left_idx, rhs_idx])
                            })
                            .is_none_or(|[left_idx, right_idx]| left_idx < right_idx)
                    })
                    .then_some(
                        #[allow(clippy::indexing_slicing)]
                        usize::from(page[page.len() >> 1_u8]),
                    )
            })
            .sum()
    }

    #[must_use]
    pub fn part_2(&self) -> usize {
        self.pages
            .iter()
            .filter(|page| {
                !self.page_ordering_rules.iter().all(|&[lhs, rhs]| {
                    page.iter()
                        .position(|entry| lhs.eq(entry))
                        .and_then(|left_idx| {
                            page.iter()
                                .position(|entry| rhs.eq(entry))
                                .map(|rhs_idx| [left_idx, rhs_idx])
                        })
                        .is_none_or(|[left_idx, right_idx]| left_idx < right_idx)
                })
            })
            .map(|page| {
                let mut sorted_page = page.clone();
                sorted_page.sort_unstable_by(|&page_lhs, &page_rhs| {
                    if self
                        .page_ordering_rules
                        .binary_search(&[page_lhs, page_rhs])
                        .is_ok()
                    {
                        return Ordering::Less;
                    }
                    if self
                        .page_ordering_rules
                        .binary_search(&[page_rhs, page_lhs])
                        .is_ok()
                    {
                        return Ordering::Greater;
                    }
                    Ordering::Equal
                });
                #[allow(clippy::indexing_slicing)]
                let res = usize::from(sorted_page[sorted_page.len() >> 1_u8]);
                res
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
                expected: 143,
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
                expected: 123,
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
            "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
            Input {
                page_ordering_rules: vec![
                    [29, 13],
                    [47, 13],
                    [47, 29],
                    [47, 53],
                    [47, 61],
                    [53, 13],
                    [53, 29],
                    [61, 13],
                    [61, 29],
                    [61, 53],
                    [75, 13],
                    [75, 29],
                    [75, 47],
                    [75, 53],
                    [75, 61],
                    [97, 13],
                    [97, 29],
                    [97, 47],
                    [97, 53],
                    [97, 61],
                    [97, 75],
                ],
                pages: vec![
                    vec![75, 47, 61, 53, 29],
                    vec![97, 61, 53, 29, 13],
                    vec![75, 29, 13],
                    vec![75, 97, 47, 61, 53],
                    vec![61, 13, 29],
                    vec![97, 13, 75, 29, 47],
                ],
            },
        )
    }
}
