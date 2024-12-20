use array2d::Array2D;
use hibitset::BitSet;
use rayon::prelude::*;
use shared::read_file_to_grid;

fn main() {
    println!("PART A: {}", part_a("input.txt"));
    println!("PART B: {}", part_b("input.txt"));
}
#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
    fn move_point(&self, row: i64, col: i64) -> (i64, i64) {
        match self {
            Direction::Up => (row - 1, col),
            Direction::Right => (row, col + 1),
            Direction::Down => (row + 1, col),
            Direction::Left => (row, col - 1),
        }
    }
    fn as_usize(&self) -> usize {
        match self {
            Direction::Up => 0,
            Direction::Right => 1,
            Direction::Down => 2,
            Direction::Left => 3,
        }
    }
}
#[derive(Clone, Copy)]
struct Guard {
    current_row: i64,
    current_col: i64,
    current_direction: Direction,
}
impl Guard {
    pub fn walk_matrix_count_steps(&mut self, mut grid: Array2D<u8>) -> i64 {
        let mut steps = 1;
        grid.set(self.current_row as usize, self.current_col as usize, b'x')
            .unwrap();
        loop {
            let (row, col) = self
                .current_direction
                .move_point(self.current_row, self.current_col);
            // If out of bounds, we are done
            if row < 0
                || col < 0
                || row >= grid.num_rows() as i64
                || col >= grid.num_columns() as i64
            {
                break;
            }
            if grid.get(row as usize, col as usize).unwrap() == &b'#' {
                self.current_direction = self.current_direction.turn_right();
            } else {
                self.current_row = row;
                self.current_col = col;
                if grid.get(row as usize, col as usize).unwrap() == &b'#' {
                    panic!("collision");
                }
                if grid.get(row as usize, col as usize).unwrap() != &b'x' {
                    steps += 1;
                    grid.set(row as usize, col as usize, b'x').unwrap();
                }
            }
        }
        steps
    }
    pub fn walk_matrix_does_loop(&mut self, grid: &Array2D<u8>) -> bool {
        // Only goes up to like 130 rows, ln2(130) = 7,So allocate 8 bits for row, 8 bits for column, 2 bits for direction
        //
        let max_value = 1 << (8 + 8 + 2);
        let mut position_back_buffer: BitSet = BitSet::with_capacity(max_value as u32);

        loop {
            let (row, col) = self
                .current_direction
                .move_point(self.current_row, self.current_col);
            // If out of bounds, we are done
            if row < 0
                || col < 0
                || row >= grid.num_rows() as i64
                || col >= grid.num_columns() as i64
            {
                return false; // Doesn't loop as we hit the edge
            }
            if grid.get(row as usize, col as usize).unwrap() == &b'#' {
                self.current_direction = self.current_direction.turn_right();
            } else {
                self.current_row = row;
                self.current_col = col;
                //Encode current position
                let encoded: u32 = (self.current_row as u32) << 10
                    | (self.current_col as u32) << 2
                    | self.current_direction.as_usize() as u32;
                if position_back_buffer.add(encoded) {
                    return true;
                }
            }
        }
    }
}
fn part_a(path: &str) -> i64 {
    let grid = read_file_to_grid(path);
    //Find guard init state
    // Walk grid to find the '^' character
    let (current_row, current_col) = grid
        .enumerate_row_major()
        .filter_map(|((row, col), val)| match val == &b'^' {
            true => Some((row as i64, col as i64)),
            false => None,
        })
        .next()
        .unwrap();
    println!("Guard starts at {current_row} {current_col}");
    let mut guard = Guard {
        current_row,
        current_col,
        current_direction: Direction::Up,
    };

    guard.walk_matrix_count_steps(grid)
}
fn part_b(path: &str) -> i64 {
    let grid = read_file_to_grid(path);
    //Find guard init state
    // Walk grid to find the '^' character
    let (current_row, current_col) = grid
        .enumerate_row_major()
        .filter_map(|((row, col), val)| match val == &b'^' {
            true => Some((row as i64, col as i64)),
            false => None,
        })
        .next()
        .unwrap();
    println!("Guard starts at {current_row} {current_col}");
    let guard = Guard {
        current_row,
        current_col,
        current_direction: Direction::Up,
    };

    // For all positions, that are empty, test putting obstruction there
    let empty_spots: Vec<(usize, usize)> = grid
        .enumerate_row_major()
        .filter_map(
            |((row, col), val)| {
                if val == &b'.' {
                    Some((row, col))
                } else {
                    None
                }
            },
        )
        .collect();
    empty_spots
        .par_iter()
        .map(|(row, col)| {
            let mut new_grid = grid.clone();
            new_grid.set(*row, *col, b'#').unwrap();
            let mut new_guard = guard;
            new_guard.walk_matrix_does_loop(&new_grid)
        })
        .filter(|x| *x)
        .count() as i64 // Count how many are true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a_demo() {
        let results = part_a("test.txt");
        assert_eq!(results, 41);
    }
    #[test]
    fn test_part_a_real() {
        let results = part_a("input.txt");
        assert_eq!(results, 4826);
    }
    #[test]
    fn test_part_b_demo() {
        let results = part_b("test.txt");
        assert_eq!(results, 6);
    }
    #[test]
    fn test_part_b_real() {
        let results = part_b("input.txt");
        assert_eq!(results, 1721);
    }
}
