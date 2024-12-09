use core::iter;
use std::{fs::read_to_string, path::Path};

pub const PART_1: usize = 6_334_655_979_668;
pub const PART_2: usize = 6_349_492_251_099;

#[must_use]
pub fn read_data(data_dir: &str) -> String {
    read_to_string(Path::new(data_dir).join("day_09.txt"))
        .unwrap()
        .trim()
        .to_owned()
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Input {
    disk_map: Vec<u8>,
}

impl Input {
    #[must_use]
    pub fn from_data(data: &str) -> Self {
        let disk_map = data.trim().bytes().map(|byte| byte - b'0').collect();
        Self { disk_map }
    }

    #[must_use]
    pub fn part_1(&self) -> usize {
        let mut map =
            self.disk_map
                .iter()
                .copied()
                .enumerate()
                .fold(Vec::new(), |mut acc, (idx, digit)| {
                    if idx & 1 > 0 {
                        acc.extend(iter::repeat_n(None, digit.into()));
                    } else {
                        acc.extend(iter::repeat_n(Some(idx >> 1_usize), digit.into()));
                    }
                    acc
                });

        let mut i = 0;
        while i < map.len() {
            #[expect(clippy::indexing_slicing)]
            if map[i].is_none() {
                _ = map.swap_remove(i);
            }
            while Some(&None) == map.last() {
                _ = map.pop();
            }
            i += 1;
        }

        map.iter()
            .enumerate()
            .map(|(idx, num)| {
                let Some(id) = num else { return 0 };
                idx * id
            })
            .sum()
    }

    #[must_use]
    pub fn part_2(&self) -> usize {
        let mut map = self
            .disk_map
            .iter()
            .copied()
            .enumerate()
            .map(|(idx, val)| {
                if idx & 1 > 0 {
                    DiskMapEntry::Space(val)
                } else {
                    DiskMapEntry::File {
                        id: idx >> 1_usize,
                        length: val,
                    }
                }
            })
            .collect::<Vec<_>>();

        let mut map_idx = map.len();
        while map_idx > 0 {
            map_idx -= 1;
            #[expect(clippy::indexing_slicing)]
            let entry = map[map_idx];

            match entry {
                DiskMapEntry::File { length, .. } => {
                    #[expect(clippy::indexing_slicing)]
                    if let Some((idx, len)) =
                        map.iter().enumerate().find_map(|(idx, val)| match val {
                            DiskMapEntry::File { .. } => None,
                            DiskMapEntry::Space(len) => {
                                (idx < map_idx && length.le(len)).then_some((idx, len))
                            }
                        })
                    {
                        if length.eq(len) {
                            map.swap(map_idx, idx);
                            continue;
                        }

                        map[idx] = DiskMapEntry::Space(len - length);

                        map.insert(idx, DiskMapEntry::Space(length));
                        map_idx += 1;

                        map.swap(map_idx, idx);
                    }
                }
                DiskMapEntry::Space(_) => {}
            }
        }

        map.iter()
            .copied()
            .fold((0_usize, 0_usize), |(mut acc, idx), entry| match entry {
                DiskMapEntry::File { id, length } => {
                    let len_usize = usize::from(length);
                    for i in 0..len_usize {
                        acc += id * (idx + i);
                    }
                    (acc, idx + len_usize)
                }
                DiskMapEntry::Space(length) => (acc, idx + usize::from(length)),
            })
            .0
    }
}

#[derive(Debug, Clone, Copy)]
#[expect(variant_size_differences)]
enum DiskMapEntry {
    File { id: usize, length: u8 },
    Space(u8),
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
                expected: 1928,
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
                expected: 2858,
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
            "2333133121414131402",
            Input {
                disk_map: vec![2, 3, 3, 3, 1, 3, 3, 1, 2, 1, 4, 1, 4, 1, 3, 1, 4, 0, 2],
            },
        )
    }
}
