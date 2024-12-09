use std::fmt::Display;

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

fn defrag(mut filesystem: Vec<FileBlock>) -> Vec<FileBlock> {
    let fs_len = filesystem.len() - 1;
    filesystem
        .clone()
        .iter()
        .rev()
        .enumerate()
        .for_each(|(idx, fb)| {
            if *fb != FileBlock::Empty {
                // find the first empty block in filesystem and swap it
                if let Some(idx_empty_block) =
                    filesystem.iter().position(|f| *f == FileBlock::Empty)
                {
                    if idx_empty_block < fs_len - idx {
                        // swap
                        filesystem[idx_empty_block] = fb.clone();
                        filesystem[fs_len - idx] = FileBlock::Empty;
                    }
                }
            }
        });
    filesystem
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
    let decompressed = decompress(input);
    // println!("decompressed: {}", decompressed.iter().join(""));
    let defragged = defrag(decompressed);
    // println!("defragged: {}", defragged.iter().join(""));
    let checksum = generate_checksum(&defragged);
    Some(checksum)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
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
        let decompress_result = decompress("2333133121414131402");
        let result = defrag(decompress_result);
        assert_eq!(
            "0099811188827773336446555566..............",
            result.iter().join("")
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
        assert_eq!(result, None);
    }
}
