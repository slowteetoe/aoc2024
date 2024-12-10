use std::fmt::Display;

use indexmap::IndexMap;

advent_of_code::solution!(9);

#[derive(Clone, PartialEq, Eq)]
pub enum FileBlock {
    File { id: u32 },
    Empty,
}

impl Display for FileBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileBlock::File { id } => {
                write!(f, "{id}")
            }
            FileBlock::Empty => write!(f, "."),
        }
    }
}

impl FileBlock {
    pub fn new(input: &str) -> Vec<FileBlock> {
        input.chars().fold(vec![], |mut acc, c| {
            match c {
                '.' => acc.push(FileBlock::Empty),
                _ => acc.push(FileBlock::File {
                    id: c.to_digit(10).unwrap(),
                }),
            };
            acc
        })
    }
}

fn decompress(input: &str) -> Vec<FileBlock> {
    let mut id = 0;
    input
        .trim()
        .chars()
        .enumerate()
        .fold(Vec::new(), |mut acc, (idx, ch)| {
            let times = ch.to_digit(10).unwrap();
            if idx % 2 == 0 {
                for _ in 0..times {
                    acc.push(FileBlock::File { id });
                }
                id += 1;
            } else {
                for _ in 0..times {
                    acc.push(FileBlock::Empty)
                }
            }
            acc
        })
}

fn defrag(filesystem: &mut Vec<FileBlock>) {
    let mut right = filesystem.len() - 1;

    for n in 0..filesystem.len() {
        if filesystem[n] == FileBlock::Empty && n < right {
            // grab the rightmost non-empty block and swap it
            while filesystem[right] == FileBlock::Empty {
                right -= 1;
            }
            filesystem[n] = filesystem[right].clone();
            filesystem[right] = FileBlock::Empty;
        }
    }
}

fn defrag_whole_files(filesystem: &mut Vec<FileBlock>) {
    // go through the file system once, backwards, generating a map of:
    // file ids -> number of blocks
    let map = filesystem
        .clone()
        .iter()
        .rev()
        .filter(|fb| **fb != FileBlock::Empty)
        .fold(IndexMap::<u32, u32>::new(), |mut acc, fb| {
            match fb {
                FileBlock::File { id } => acc.entry(*id).and_modify(|v| *v += 1).or_insert(1),
                FileBlock::Empty => todo!(),
            };
            acc
        });

    // keep track of (number of spaces available, starting index)
    let mut holes = vec![];
    filesystem
        .iter()
        .enumerate()
        .fold((0, 0), |acc, (idx, current_block)| match current_block {
            FileBlock::File { id: _ } => {
                if acc != (0, 0) {
                    holes.push(acc);
                };
                (0, 0)
            }
            FileBlock::Empty => {
                if acc == (0, 0) {
                    (1, idx)
                } else {
                    (acc.0 + 1, acc.1)
                }
            }
        });

    // since we're using an indexmap, this is in order of largest file ids to smallest
    for (file_id, num_blocks) in map.iter() {
        if let Some(hole_index) = holes
            .iter()
            .position(|(num_holes, _)| num_holes >= num_blocks)
        {
            let hole = holes.get_mut(hole_index).unwrap();
            if let Some(file_start) = filesystem.iter().position(|b| match b {
                FileBlock::File { id } if id == file_id => true,
                _ => false,
            }) {
                if hole.1 <= file_start {
                    for i in 0..*num_blocks as usize {
                        filesystem[hole.1 + i] = filesystem[file_start + i].clone();
                        filesystem[file_start + i] = FileBlock::Empty;
                    }
                }
            }
            if hole.0 == *num_blocks {
                // hole completely filled, remove it
                holes.remove(hole_index);
            } else {
                // partial hole remains
                hole.0 -= num_blocks;
                hole.1 += *num_blocks as usize;
            }
        }
    }
}

fn generate_checksum(input: &Vec<FileBlock>) -> u64 {
    input
        .iter()
        .enumerate()
        .fold(0u64, |acc, (idx, fb)| match fb {
            FileBlock::Empty => acc,
            FileBlock::File { id } => acc + (idx as u64 * *id as u64),
        })
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut filesystem: Vec<FileBlock> = decompress(input);
    // println!("decompressed: {}", decompressed.iter().join(""));
    defrag(&mut filesystem);
    // println!("defragged: {}", defragged.iter().join(""));
    let checksum = generate_checksum(&filesystem);
    Some(checksum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut filesystem = decompress(input);
    defrag_whole_files(&mut filesystem);
    let checksum = generate_checksum(&filesystem);
    Some(checksum)
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn test_fileblock() {
        let fs = vec![
            FileBlock::File { id: 0 },
            FileBlock::File { id: 0 },
            FileBlock::Empty,
            FileBlock::Empty,
            FileBlock::Empty,
            FileBlock::File { id: 1 },
            FileBlock::File { id: 1 },
            FileBlock::File { id: 1 },
        ];
        println!("{:?}", fs.iter().join(""));
    }

    #[test]
    fn test_decompress() {
        let result = decompress("2333133121414131402");
        let f = result.iter().join("");
        assert_eq!("00...111...2...333.44.5555.6666.777.888899", f);
    }

    #[test]
    fn test_defrag() {
        let mut filesystem = decompress("2333133121414131402");
        defrag(&mut filesystem);
        assert_eq!(
            "0099811188827773336446555566..............",
            filesystem.iter().join("")
        );
    }

    #[test]
    fn test_checksum() {
        let result = generate_checksum(&FileBlock::new(
            "0099811188827773336446555566..............",
        ));
        assert_eq!(1928, result);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
