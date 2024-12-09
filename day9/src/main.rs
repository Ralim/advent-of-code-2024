use std::fmt::format;

fn main() {
    let t_a = std::thread::spawn(|| {
        println!("PART A: {}", part_a("input.txt"));
    });
    let t_b = std::thread::spawn(|| {
        println!("PART B: {}", part_b("input.txt"));
    });
    t_a.join().unwrap();
    t_b.join().unwrap();
}
#[derive(Debug, Clone, Copy)]
struct Block {
    original_id: i64,
}
#[derive(Debug, Clone)]
struct BlockWiseDisk {
    blocks: Vec<Block>,
    file_ids: Vec<i64>,
}
impl BlockWiseDisk {
    pub fn from_str(input: &str) -> Self {
        let mut blocks = Vec::new();
        let mut file_ids = Vec::new();
        for (i, char) in input.trim().chars().enumerate() {
            let size = char.to_digit(10).unwrap() as i64;
            if i % 2 == 0 {
                //used block indicator
                for _ in 0..size {
                    blocks.push(Block {
                        original_id: (i / 2) as i64,
                    });
                }
                file_ids.push((i / 2) as i64);
            } else {
                //Insert free blocks
                for _ in 0..size {
                    blocks.push(Block { original_id: -1 });
                }
            }
        }
        BlockWiseDisk { blocks, file_ids }
    }
    #[allow(unused)]
    pub fn print(&self) {
        let mut line = "".to_owned();
        for block in self.blocks.iter().as_ref() {
            if block.original_id == -1 {
                line += ".";
            } else {
                line += &format(format_args!("{}", block.original_id));
            }
        }
        println!("Disk> `{line}`")
    }
    pub fn get_checksum(&self) -> i64 {
        // To calculate the checksum, add up the result of multiplying each of these blocks' position with the file ID number it contains.
        // The leftmost block is in position 0. If a block contains free space, skip it instead.

        self.blocks
            .iter()
            .enumerate()
            .map(|(pos, b)| {
                if b.original_id >= 0 {
                    pos as i64 * b.original_id
                } else {
                    0
                }
            })
            .sum()
    }
    pub fn defragment_chunkwise(&mut self) {
        // Move all blocks from the right to the left-most empty spot

        loop {
            let right_most_used_block_position = self
                .blocks
                .iter()
                .rposition(|&b| b.original_id != -1)
                .unwrap();
            let left_most_free_block_position = self
                .blocks
                .iter()
                .position(|&b| b.original_id == -1)
                .unwrap();
            if left_most_free_block_position > right_most_used_block_position {
                //DOne, all blocks moved left
                return;
            }
            // println!(
            //     "Defragmenting {} with {}",
            //     left_most_free_block_position, right_most_used_block_position
            // );

            self.blocks.swap(
                left_most_free_block_position,
                right_most_used_block_position,
            );
            // self.print();
        }
    }
    pub fn defragment_filewise(&mut self) {
        // Attempt to move each block once, to the leftmost gap that it fits in
        // Work from highest file id -> lowest

        for &block_id in self.file_ids.iter().rev() {
            let block_start_index = self
                .blocks
                .iter()
                .position(|&b| b.original_id == block_id)
                .unwrap();
            let block_end_index = self
                .blocks
                .iter()
                .rposition(|&b| b.original_id == block_id)
                .unwrap();
            let file_size = block_end_index - block_start_index + 1;

            // println!(
            //     "File {} size {} range {}-{}",
            //     block_id, file_size, block_start_index, block_end_index
            // );
            // Now need to find the first space we can slot it into
            let mut space_start_index = -1;
            for pos in 0..block_start_index {
                if self.blocks[pos].original_id == -1 {
                    //This is a space
                    if space_start_index < 0 {
                        space_start_index = pos as i64;
                    }
                    //We already know the start
                    let space_len = (pos as i64) - space_start_index + 1;
                    // println!("Space at {}-{} len {}", space_start_index, pos, space_len);
                    if space_len >= file_size as i64 && space_start_index < block_start_index as i64
                    {
                        // println!("Can swap file {block_id} of len {file_size} into gap starting at {space_start_index} of len {space_len}");
                        //Swap all positions
                        for i in 0..file_size {
                            self.blocks
                                .swap(block_start_index + i, space_start_index as usize + i);
                        }
                        break;
                    }
                } else {
                    //End of gap
                    space_start_index = -1
                }
            }
            // self.print();
        }
    }
}

fn part_a(path: &str) -> i64 {
    let input = std::fs::read_to_string(path).unwrap();
    let mut disk = BlockWiseDisk::from_str(&input);
    // disk.print();
    disk.defragment_chunkwise();
    disk.get_checksum()
}
fn part_b(path: &str) -> i64 {
    let input = std::fs::read_to_string(path).unwrap();
    let mut disk = BlockWiseDisk::from_str(&input);
    // disk.print();
    disk.defragment_filewise();
    disk.get_checksum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a_demo() {
        let results = part_a("test.txt");
        assert_eq!(results, 1928);
    }
    #[test]
    fn test_part_a_real() {
        let results = part_a("input.txt");
        assert_eq!(results, 6225730762521);
    }
    #[test]
    fn test_part_b_demo() {
        let results = part_b("test.txt");
        assert_eq!(results, 2858);
    }
    #[test]
    fn test_part_b_real() {
        let results = part_b("input.txt");
        assert_eq!(results, 6250605700557);
    }
}
