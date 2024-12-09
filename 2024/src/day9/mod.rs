use std::{fs, str::FromStr, ops::{Add, Mul, Div}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Storage {
    FILE, FREE
}

#[derive(Debug, Clone, Copy, Default)]
struct FileBlock {
    index: usize,
    size: u32,
    content: Storage,
}

impl Default for Storage {
    fn default() -> Self {
        Storage::FREE
    }
}


impl FileBlock {

    fn calc(&self, start_idx: usize) -> u64 {
        match self.content {
            Storage::FILE => do_calc(self.size, start_idx as u32, self.index as u32),
            Storage::FREE => 0,
        }
    }

    fn print(&self, symbol: char) {
        (0..self.size).for_each(|_| print!("{symbol}"));
    }

    fn split_at_most(&self, size: u32) -> (u32, u32) {
        let remainder = self.size.checked_sub(size).unwrap_or_default();
        (self.size - remainder, remainder)
    }

}

fn do_calc(size: u32, start_idx: u32, index: u32) -> u64 {
    (0..size).map(|x| x.add(start_idx).mul(index) as u64).sum()
}


fn create_filesystem(input: &str) -> Vec<FileBlock> {
    let mut filesystem = Vec::new();
    for (file_representation_idx, block_size) in input.char_indices() {
        if !block_size.is_digit(10) {
            break;
        }
        if file_representation_idx & 1 == 0 {
            // even is file
            let f = FileBlock {
                index: file_representation_idx.div(2),
                size: block_size.to_digit(10).unwrap(),
                content: Storage::FILE,
            };
            filesystem.push(f);
        } else {
            // uneven is free space
            let f = FileBlock {
                index: file_representation_idx,
                size: block_size.to_digit(10).unwrap(),
                content: Storage::FREE,
            };
            filesystem.push(f);
        }
    }
    filesystem
}


fn calculate_checksum(filesystem: &mut Vec<FileBlock>) -> u64 {
    let mut file_block = 0;
    let mut checksum: u64 = 0;
    while !filesystem.is_empty() {
        let mut current_file = filesystem[0];
        match current_file.content {
            Storage::FILE => {
                checksum += current_file.calc(file_block);
                file_block += usize::try_from(current_file.size).unwrap_or_default();
                filesystem.remove(0);
            },
            Storage::FREE => {
                let mut defrag_element = match filesystem.pop() {
                    Some(f) if f.content.eq(&Storage::FILE) => f,
                    Some(_) => filesystem.pop().unwrap_or_default(),
                    _ => FileBlock { index: 0, size: 0, content: Storage::FREE },
                };
    
                let (take, keep) = defrag_element.split_at_most(current_file.size);
                let c = do_calc(take, file_block as u32, defrag_element.index as u32);
                checksum += c;
                file_block += usize::try_from(take).unwrap_or_default();
    
                defrag_element.size = keep;
                if keep > 0 {
                    filesystem.push(defrag_element);
                }
                current_file.size = current_file.size.checked_sub(take).unwrap();
                if current_file.size == 0 && !filesystem.is_empty() {
                    filesystem.remove(0);
                } else if !filesystem.is_empty() {
                    filesystem[0] = current_file;
                }
            },
        }
    
    }
    checksum
}


fn defrag(filesystem: &mut Vec<FileBlock>) {
    for i in (0..filesystem.len()).rev() {
        let current_file = filesystem[i];
        if current_file.content == Storage::FILE {
            for mv in 0..i {
                let free_space = filesystem[mv];
                if free_space.content == Storage::FREE && current_file.size <= free_space.size {
                    let (used, free_size) = free_space.split_at_most(current_file.size);
                    filesystem[mv].size = used;
                    filesystem.swap(i, mv);
                    filesystem.insert(mv +1, FileBlock { index: free_space.index, size: free_size, content: Storage::FREE });
                    break;
                }
            }
        }
    }
}


fn calculate_checksum2(filesystem: &mut Vec<FileBlock>) -> u64 {
    defrag(filesystem);

    let mut file_block = 0;
    let mut checksum: u64 = 0;

    for current_file in filesystem {
        match current_file.content {
            Storage::FILE => {
                checksum += current_file.calc(file_block);
                file_block += usize::try_from(current_file.size).unwrap_or_default();
            },
            Storage::FREE => {
                file_block += usize::try_from(current_file.size).unwrap_or_default();
            },
        }
    }
    checksum
}



#[cfg(test)]
pub mod day9_tests {
    use super::*;


#[test]
fn example1() {
    let input = "2333133121414131402";
    // represents: 00...111...2...333.44.5555.6666.777.888899
    // reformat: 00...111...2...333.44.5555.6666.777.888899 -> .. -> 009981118882...333.44.5555.6666.777....... -> .. ->
    // 0099811188827773336446555566
    // calc checksum: 0 * 0 = 0, 1 * 0 = 0, 2 * 9 = 18, 3 * 9 = 27, 4 * 8 = 32

    // reformat last to first empty space
    // calculate checksum: 0*0 + 1*0 + 2*9 + 3*9 ...
    let mut filesystem = create_filesystem(input);
    let checksum = calculate_checksum(&mut filesystem);
    assert_eq!(1928, checksum);
}



#[test]
fn part1() {
    let input = fs::read_to_string("src/day9/input.txt").unwrap();
    let mut filesystem = create_filesystem(&input);
    let checksum = calculate_checksum(&mut filesystem);
    assert_eq!(6421128769094, checksum);
}



#[test]
fn example2() {
    let input = "2333133121414131402";
    // represents: 00...111...2...333.44.5555.6666.777.888899
    // reformat: 00...111...2...333.44.5555.6666.777.888899 -> 00992111777.44.333....5555.6666.....8888..
    // calc checksum: 0 * 0 = 0, 1 * 0 = 0, 2 * 9 = 18, 3 * 9 = 27, 4 * 8 = 32

    // reformat last to first empty space
    // calculate checksum
    let mut filesystem = create_filesystem(input);
    let checksum = calculate_checksum2(&mut filesystem);
    assert_eq!(2858, checksum);
}


#[test]
fn part2() {
    let input = fs::read_to_string("src/day9/input.txt").unwrap();
    let mut filesystem = create_filesystem(&input);
    let checksum = calculate_checksum2(&mut filesystem);
    assert_eq!(6448168620520, checksum);
}

}
