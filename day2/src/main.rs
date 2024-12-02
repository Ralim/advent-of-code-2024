use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use shared::read_whitespace_separated_numbers_by_row;

fn main() {
    println!("PART A: {}", part_a("input.txt"));
    println!("PART B: {}", part_b("input.txt"));
}

fn part_a(path: &str) -> i64 {
    let input = read_whitespace_separated_numbers_by_row(path).unwrap();

    input
        .par_iter()
        .map(|row: &Vec<i64>| match is_row_safe(row) {
            true => 1,
            false => 0,
        })
        .sum()
}
fn part_b(path: &str) -> i64 {
    let input = read_whitespace_separated_numbers_by_row(path).unwrap();

    input
        .par_iter()
        .map(|row: &Vec<i64>| {
            // For each row, we want to check the step size of each pair in the data
            match test_row_with_removal(row) {
                true => 1,
                false => 0,
            }
        })
        .sum()
}

fn is_row_safe(row: &[i64]) -> bool {
    let mut last_direction = row[0] - row[1];

    if last_direction.abs() > 3 || last_direction.abs() < 1 {
        return false;
    }
    for i in 1..row.len() - 1 {
        let step = row[i] - row[i + 1];

        if step.signum() != last_direction.signum() {
            return false;
        }

        if step.abs() > 3 || step.abs() < 1 {
            return false;
        }
        last_direction = step;
    }
    true
}
fn test_row_with_removal(row: &[i64]) -> bool {
    if is_row_safe(row) {
        return true;
    }
    for i in 0..row.len() {
        if is_row_safe(&omit_one_entry(row, i)) {
            return true;
        }
    }
    false
}
// Return a copy of the slice with the item at the index removed
fn omit_one_entry(data: &[i64], index_removed: usize) -> Vec<i64> {
    let mut new_data = Vec::with_capacity(data.len() - 1);
    for (i, item) in data.iter().enumerate() {
        if i != index_removed {
            new_data.push(*item);
        }
    }
    new_data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a() {
        let results = part_a("test.txt");
        assert_eq!(results, 2);
    }
    #[test]
    fn test_part_a_real() {
        let results = part_a("input.txt");
        assert_eq!(results, 463);
    }
    #[test]
    fn test_part_b() {
        let results = part_b("test.txt");
        assert_eq!(results, 4);
    }
    #[test]
    fn test_part_b_real() {
        let results = part_b("input.txt");
        assert_eq!(results, 514);
    }
}
