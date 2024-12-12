use std::fs::read_to_string;
use std::io;
mod matrix;
pub use matrix::*;
mod combos;
pub use combos::*;
mod matrix_direction;
pub use matrix_direction::*;

pub fn read_whitespace_separated_numbers_by_column(
    file_path: &str,
    expected_width: usize,
) -> io::Result<Vec<Vec<i64>>> {
    // Read in the file, for each line split on whitespace and parse each number as i64
    let mut columns = vec![Vec::new(); expected_width];

    for line in read_to_string(file_path)?.lines() {
        let cols: Vec<i64> = line
            .split_whitespace()
            .map(|num| num.parse::<i64>().unwrap())
            .collect();
        for (i, col) in cols.iter().enumerate() {
            columns[i].push(*col);
        }
    }
    Ok(columns)
}

pub fn read_whitespace_separated_numbers_by_row(file_path: &str) -> io::Result<Vec<Vec<i64>>> {
    // Read in the file, for each line split on whitespace and parse each number as i64

    Ok(read_to_string(file_path)?
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect()
        })
        .collect())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_whitespace_separated_numbers() {
        let results = read_whitespace_separated_numbers_by_column(
            "./src/test_data/whitespace_numbers.txt",
            3,
        )
        .unwrap();
        assert_eq!(results, vec![vec![1, 4], vec![2, 5], vec![3, 6]]);
    }
    #[test]
    fn test_read_whitespace_separated_numbers_by_row() {
        let results =
            read_whitespace_separated_numbers_by_row("./src/test_data/whitespace_numbers_rows.txt")
                .unwrap();
        assert_eq!(
            results,
            vec![
                vec![7, 6, 4, 2, 1],
                vec![1, 2, 7, 8, 9],
                vec![9, 7, 6, 2, 1],
                vec![1, 3, 2, 4, 5],
                vec![8, 6, 4, 4, 1],
                vec![1, 3, 6, 7, 9]
            ]
        );
    }
}
