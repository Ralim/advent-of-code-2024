use array2d::Array2D;

pub fn read_file_to_grid(path: &str) -> Array2D<u8> {
    let file_contents = std::fs::read_to_string(path).unwrap();
    let lines: Vec<&str> = file_contents.lines().collect();
    let filtered_file = file_contents.replace("\n", "").replace("\r", "");

    Array2D::from_row_major(filtered_file.as_bytes(), lines[0].len(), lines.len()).unwrap()
}

pub fn read_file_to_num_grid(path: &str) -> Array2D<i64> {
    let file_contents = std::fs::read_to_string(path).unwrap();
    let lines: Vec<&str> = file_contents.lines().collect();
    let filtered_file = file_contents.replace("\n", "").replace("\r", "");
    let values: Vec<i64> = filtered_file
        .as_bytes()
        .iter()
        .map(|&a| (a - b'0') as i64)
        .collect();
    Array2D::from_row_major(&values, lines[0].len(), lines.len()).unwrap()
}
