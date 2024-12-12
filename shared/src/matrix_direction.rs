use array2d::Array2D;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn move_point<T>(&self, grid: &Array2D<T>, r: usize, c: usize) -> Option<(usize, usize)> {
        match self {
            Direction::Up => {
                if r > 0 {
                    Some((r - 1, c))
                } else {
                    None
                }
            }
            Direction::Down => {
                if r < grid.column_len() - 1 {
                    Some((r + 1, c))
                } else {
                    None
                }
            }
            Direction::Left => {
                if c > 0 {
                    Some((r, c - 1))
                } else {
                    None
                }
            }
            Direction::Right => {
                if c < grid.row_len() - 1 {
                    Some((r, c + 1))
                } else {
                    None
                }
            }
        }
    }
}
pub const ALL_DIRECTIONS: [Direction; 4] = [
    Direction::Down,
    Direction::Left,
    Direction::Right,
    Direction::Up,
];
