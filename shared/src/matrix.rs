use core::str;

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

pub fn rotate_array(mut grid: Array2D<u8>) -> Array2D<u8> {
    //Rotate the array 90 deg to the right
    // Do this by transposing the array, then reversing the rows
    assert_eq!(grid.row_len(), grid.column_len());

    //Transpose
    for r in 0..grid.num_rows() {
        for c in 0..r {
            let tmp = *grid.get(r, c).unwrap();
            let tmp2 = *grid.get(c, r).unwrap();
            grid.set(r, c, tmp2).unwrap();
            grid.set(c, r, tmp).unwrap();
        }
    }
    // Flip each row horizontally
    for r in 0..grid.num_rows() {
        for c in 0..grid.num_columns() / 2 {
            let tmp = *grid.get(r, c).unwrap();
            grid.set(r, c, *grid.get(r, grid.num_columns() - c - 1).unwrap())
                .unwrap();
            grid.set(r, grid.num_columns() - c - 1, tmp).unwrap();
        }
    }
    grid
}
pub fn print_array(array: &Array2D<u8>) {
    println!();
    for row_iter in array.rows_iter() {
        for element in row_iter {
            print!("{}", str::from_utf8(&[*element]).unwrap());
        }
        println!();
    }
    println!();
}
