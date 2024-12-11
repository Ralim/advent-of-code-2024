use cached::proc_macro::cached;
use count_digits::CountDigits;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
fn main() {
    println!("PART A: {}", part_a("input.txt"));
    println!("PART B: {}", part_b("input.txt"));
}

type Stone = u64;

// Perform a rule update, if a new stone is made return it
pub fn blink(stone: &mut Stone) -> Option<Stone> {
    //Rule 0
    if *stone == 0 {
        *stone = 1;
        return None;
    }
    //Rule 1
    let num_digits = stone.count_digits();
    if num_digits % 2 == 0 {
        //Event number, split in half, this stone gets the first half and return a new stone with second half
        let left = *stone / 10u64.pow((num_digits / 2) as u32);
        let right = *stone % 10u64.pow((num_digits / 2) as u32);
        let new_stone = right;
        *stone = left;
        return Some(new_stone);
    }
    // (final) Rule 2
    *stone *= 2024;
    None
}

//Cache as we hammer this at the low layers of depth
#[cached]
fn blink_stone_len_depth_recurse(mut stone: Stone, mut steps: usize) -> usize {
    if steps == 0 {
        return 1; // We are but one stone
    }
    steps -= 1;

    match blink(&mut stone) {
        Some(s2) => {
            blink_stone_len_depth_recurse(s2, steps) + blink_stone_len_depth_recurse(stone, steps)
        }
        None => blink_stone_len_depth_recurse(stone, steps),
    }
}

pub fn blink_to_count_parallel(stones: &[Stone], blinks: usize) -> usize {
    stones
        .par_iter()
        .map(|start_stone| blink_stone_len_depth_recurse(*start_stone, blinks))
        .sum()
}

fn part_a(path: &str) -> i64 {
    let stones: Vec<Stone> = std::fs::read_to_string(path)
        .unwrap()
        .split_whitespace()
        .map(|l| l.parse().unwrap())
        .collect();
    blink_to_count_parallel(&stones, 25) as i64
}
fn part_b(path: &str) -> i64 {
    let stones: Vec<Stone> = std::fs::read_to_string(path)
        .unwrap()
        .split_whitespace()
        .map(|l| l.parse().unwrap())
        .collect();
    blink_to_count_parallel(&stones, 75) as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a_demo() {
        let results = part_a("test.txt");
        assert_eq!(results, 55312);
    }
    #[test]
    fn test_part_a_real() {
        let results = part_a("input.txt");
        assert_eq!(results, 233875);
    }
    #[test]
    fn test_part_b_demo() {
        let results = part_b("test.txt");
        assert_eq!(results, 65601038650482);
    }
    #[test]
    fn test_part_b_real() {
        let results = part_b("input.txt");
        assert_eq!(results, 277444936413293);
    }
}
