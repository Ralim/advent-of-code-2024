use array2d::Array2D;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn main() {
    println!("PART A: {}", part_a("input.txt"));
    println!("PART B: {}", part_b("input.txt"));
}

fn part_a(path: &str) -> i64 {
    let grid = {
        let file_contents = std::fs::read_to_string(path).unwrap();
        let lines: Vec<&str> = file_contents.lines().collect();
        let filtered_file = file_contents.replace("\n", "").replace("\r", "");

        Array2D::from_row_major(filtered_file.as_bytes(), lines[0].len(), lines.len()).unwrap()
    };

    let search_words = ["XMAS", "SAMX"];

    (0..grid.row_len())
        .into_par_iter()
        .map(|row| {
            (0..grid.column_len())
                .into_par_iter()
                .map(|col| {
                    ALL_DIRECTIONS
                        .into_iter()
                        .map(|dir| {
                            search_words
                                .iter()
                                .map(|word| {
                                    if search_at(&grid, word, dir, row, col) {
                                        // println!("Found {} at {},{}", word, row, col);
                                        1
                                    } else {
                                        0
                                    }
                                })
                                .sum::<i64>()
                        })
                        .sum::<i64>()
                })
                .sum::<i64>()
        })
        .sum::<i64>()
}
fn part_b(path: &str) -> i64 {
    let grid = {
        let file_contents = std::fs::read_to_string(path).unwrap();
        let lines: Vec<&str> = file_contents.lines().collect();
        let filtered_file = file_contents.replace("\n", "").replace("\r", "");

        Array2D::from_row_major(filtered_file.as_bytes(), lines[0].len(), lines.len()).unwrap()
    };

    let grid_search = [
        // M_M
        // _A_
        // S_S
        [
            (0, 0, b'M'),
            (0, 2, b'M'), //
            (1, 1, b'A'), //
            (2, 0, b'S'),
            (2, 2, b'S'), //
        ],
        // S_M
        // _A_
        // S_M
        [
            (0, 0, b'S'),
            (0, 2, b'M'), //
            (1, 1, b'A'), //
            (2, 0, b'S'),
            (2, 2, b'M'), //
        ],
        // S_S
        // _A_
        // M_M
        [
            (0, 0, b'S'),
            (0, 2, b'S'), //
            (1, 1, b'A'), //
            (2, 0, b'M'),
            (2, 2, b'M'), //
        ],
        // M_S
        // _A_
        // M_S
        [
            (0, 0, b'M'),
            (0, 2, b'S'), //
            (1, 1, b'A'), //
            (2, 0, b'M'),
            (2, 2, b'S'), //
        ],
    ];

    (0..grid.row_len())
        .into_par_iter()
        .map(|row| {
            (0..grid.column_len())
                .into_par_iter()
                .map(|col| {
                    grid_search
                        .into_iter()
                        .map(|pattern| {
                            if match_grid_pattern(&grid, row, col, &pattern) {
                                1
                            } else {
                                0
                            }
                        })
                        .sum::<i64>()
                })
                .sum::<i64>()
        })
        .sum::<i64>()
}
const ALL_DIRECTIONS: [SearchDirection; 4] = [
    SearchDirection::Horizontal,
    SearchDirection::Vertical,
    SearchDirection::ForwardDiagonal,
    SearchDirection::BackwardDiagonal,
];
#[derive(Debug, Clone, Copy)]
enum SearchDirection {
    Horizontal,
    Vertical,
    ForwardDiagonal,
    BackwardDiagonal,
}
impl SearchDirection {
    // Returns in row,col,value format
    pub fn splat_word(&self, word: &str) -> Vec<(usize, usize, u8)> {
        match self {
            SearchDirection::Horizontal => word
                .as_bytes()
                .iter()
                .enumerate()
                .map(|(i, c)| (0, i, *c))
                .collect(),
            SearchDirection::Vertical => word
                .as_bytes()
                .iter()
                .enumerate()
                .map(|(i, c)| (i, 0, *c))
                .collect(),
            /*
            X
             M
              A
               S
             */
            SearchDirection::ForwardDiagonal => word
                .as_bytes()
                .iter()
                .enumerate()
                .map(|(i, c)| (i, i, *c))
                .collect(),
            SearchDirection::BackwardDiagonal => {
                /*
                   X
                  M
                 A
                S
                */
                let len = word.len() - 1;
                word.as_bytes()
                    .iter()
                    .enumerate()
                    .map(|(i, c)| (i, len - i, *c))
                    .collect()
            }
        }
    }
}

fn match_grid_pattern(
    grid: &Array2D<u8>,
    base_row: usize,
    base_col: usize,
    pattern: &[(usize, usize, u8)],
) -> bool {
    for (check_row, check_col, check_val) in pattern {
        let row = base_row + check_row;
        let col = base_col + check_col;
        let grid_val = grid.get(row, col);
        match grid_val {
            Some(val) => {
                if *val != *check_val {
                    return false;
                }
            }
            None => return false,
        }
    }
    true
}
fn search_at(
    grid: &Array2D<u8>,
    word: &str,
    direction: SearchDirection,
    base_row: usize,
    base_col: usize,
) -> bool {
    match_grid_pattern(grid, base_row, base_col, &direction.splat_word(word))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a_demo() {
        let results = part_a("test.txt");
        assert_eq!(results, 18);
    }
    #[test]
    fn test_part_a_real() {
        let results = part_a("input.txt");
        assert_eq!(results, 2639);
    }
    #[test]
    fn test_part_b_demo() {
        let results = part_b("test.txt");
        assert_eq!(results, 9);
    }
    #[test]
    fn test_part_b_real() {
        let results = part_b("input.txt");
        assert_eq!(results, 2005);
    }
}
