use std::fs::read_to_string;
use std::io;

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
}
