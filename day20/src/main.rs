use array2d::Array2D;
use itertools::Itertools;
use shared::read_file_to_grid;
use std::{collections::VecDeque, fmt::Display};

fn main() {
    let t_a = std::thread::spawn(|| {
        println!("PART A: {}", part_a("input.txt", 100));
    });
    let t_b = std::thread::spawn(|| {
        println!("PART B: {}", part_b("input.txt", 100));
    });
    t_a.join().unwrap();
    t_b.join().unwrap();
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
const ALL_DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];
impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "^"),
            Direction::Down => write!(f, "v"),
            Direction::Left => write!(f, "<"),
            Direction::Right => write!(f, ">"),
        }
    }
}
impl From<u8> for Direction {
    fn from(value: u8) -> Self {
        match value {
            b'^' => Direction::Up,
            b'v' => Direction::Down,
            b'<' => Direction::Left,
            b'>' => Direction::Right,
            _ => panic!("Invalid direction {value}"),
        }
    }
}
impl From<Direction> for u8 {
    fn from(val: Direction) -> Self {
        match val {
            Direction::Up => b'^',
            Direction::Down => b'v',
            Direction::Left => b'<',
            Direction::Right => b'>',
        }
    }
}
impl Direction {
    fn move_point(&self, grid: &Array2D<u8>, r: usize, c: usize) -> Option<(usize, usize)> {
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

struct Map {
    grid: Array2D<u8>,
    start: (usize, usize),
    end: (usize, usize),
}
impl Map {
    pub fn from_file(path: &str) -> Self {
        let grid = read_file_to_grid(path);
        let start_position: (usize, usize) = grid
            .enumerate_row_major()
            .filter_map(|(p, &v)| if v == b'S' { Some(p) } else { None })
            .next()
            .unwrap();
        let end_position: (usize, usize) = grid
            .enumerate_row_major()
            .filter_map(|(p, &v)| if v == b'E' { Some(p) } else { None })
            .next()
            .unwrap();
        Self {
            grid,
            start: start_position,
            end: end_position,
        }
    }

    fn count_solutions_with_skip_count(
        &self,
        maximum_skip_distance_inclusive: i64,
        target_saving: u32,
    ) -> i64 {
        let (sc, ec) = (
            self.compute_cost_to_exit(self.start),
            self.compute_cost_to_exit(self.end),
        );
        // Cost from Start -> End
        // Cost from End -> Start

        let base_cost = sc[self.end]; // The fastest time to the end with no mods

        let mut out = 0;

        for (pos, tile) in self.grid.enumerate_row_major() {
            // This tile is a wall, or it has no path to the exit
            if *tile == b'#' || sc[pos] == u32::MAX {
                continue;
            }

            for (x, y) in (-maximum_skip_distance_inclusive..=maximum_skip_distance_inclusive)
                .cartesian_product(
                    -maximum_skip_distance_inclusive..=maximum_skip_distance_inclusive,
                )
            {
                let dist = x.abs() + y.abs();

                if dist > maximum_skip_distance_inclusive {
                    continue;
                }
                let res_x = pos.0 as i64 + x;
                let res_y = pos.1 as i64 + y;
                if res_x < 0 || res_y < 0 {
                    continue;
                }

                let result_pos = (res_x as usize, res_y as usize);
                //If result pos is out of the grid, or is a wall, or has no path to the exit
                if result_pos.0 >= self.grid.num_rows()
                    || result_pos.1 >= self.grid.num_columns()
                    || self.grid[result_pos] == b'#'
                    || ec[result_pos] == u32::MAX
                {
                    continue;
                }
                // println!("Sc {} ec {} dist {}", sc[pos], ec[result_pos], dist);
                let cost = sc[pos] + ec[result_pos] + dist as u32;
                out += ((cost + target_saving) <= base_cost) as u32;
            }
        }

        out as i64
    }
    fn compute_cost_to_exit(&self, start_position: (usize, usize)) -> Array2D<u32> {
        let mut costs =
            Array2D::filled_with(u32::MAX, self.grid.num_rows(), self.grid.num_columns());
        let mut queue = VecDeque::new();
        queue.push_back((start_position, 0));

        // Walk from the start onto all possible tiles
        while let Some((pos, dist)) = queue.pop_front() {
            //Already been here
            if costs[pos] != u32::MAX {
                continue;
            }

            costs[pos] = dist;
            for dir in ALL_DIRECTIONS {
                if let Some(next) = dir.move_point(&self.grid, pos.0, pos.1) {
                    if let Some(tile) = self.grid.get(next.0, next.1) {
                        if matches!(tile, b'.' | b'E') {
                            queue.push_back((next, dist + 1));
                        }
                    }
                }
            }
        }

        costs
    }
}

fn part_a(path: &str, target_saving: u32) -> i64 {
    let map = Map::from_file(path);

    map.count_solutions_with_skip_count(2, target_saving)
}
fn part_b(path: &str, target_saving: u32) -> i64 {
    let map = Map::from_file(path);

    map.count_solutions_with_skip_count(20, target_saving)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a_demo() {
        let results = part_a("test.txt", 38);
        assert_eq!(results, 3);
    }
    #[test]
    fn test_part_a_real() {
        let results = part_a("input.txt", 100);
        assert_eq!(results, 1372);
    }
    #[test]
    fn test_part_b_demo() {
        let results = part_b("test.txt", 38);
        assert_eq!(results, 644);
    }
    #[test]
    fn test_part_b_real() {
        let results = part_b("input.txt", 100);
        assert_eq!(results, 979014);
    }
}
