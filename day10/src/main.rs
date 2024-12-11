use array2d::Array2D;
use shared::read_file_to_num_grid;
use std::collections::HashSet;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn move_point(&self, grid: &Array2D<i64>, r: usize, c: usize) -> Option<(usize, usize)> {
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
const ALL_DIRECTIONS: [Direction; 4] = [
    Direction::Down,
    Direction::Left,
    Direction::Right,
    Direction::Up,
];
#[derive(Debug, Clone)]
struct TopoHeightMap {
    map: Array2D<i64>,
}
impl TopoHeightMap {
    pub fn get_trail_heads(&self) -> Vec<(usize, usize)> {
        //Find the x/y position of all trail heads
        self.map
            .enumerate_row_major()
            .filter_map(|((row, col), val)| match val {
                0 => Some((row, col)),
                _ => None,
            })
            .collect()
    }

    // Returns count of ends reached
    pub fn walk_uphill_to_end(
        &self,
        position: (usize, usize),
        mut history: HashSet<(usize, usize)>,
        ends_seen: &mut HashSet<(usize, usize)>,
        filter_distinct: bool,
    ) -> i64 {
        let current_position_height = self.map.get(position.0, position.1).unwrap();
        let target_height = current_position_height + 1;
        if !history.insert(position) {
            return 0; // Have been here before, dont step backwards
        }
        ALL_DIRECTIONS
            .iter()
            .filter_map(|d| d.move_point(&self.map, position.0, position.1))
            .map(|point| {
                let point_height = self.map.get(point.0, point.1).unwrap();

                if *point_height == target_height {
                    if *point_height == 9 {
                        // At the end target
                        // println!("Got to the end via {history:?}");
                        if filter_distinct {
                            if ends_seen.insert(point) {
                                1
                            } else {
                                0
                            }
                        } else {
                            1
                        }
                    } else {
                        //Recurse
                        println!(
                            "Point test {point:?} = {}",
                            self.map.get(point.0, point.1).unwrap()
                        );
                        self.walk_uphill_to_end(point, history.clone(), ends_seen, filter_distinct)
                    }
                } else {
                    // Not able to walk uphill here
                    0
                }
            })
            .sum()
    }
}
fn main() {
    let t_a = std::thread::spawn(|| {
        println!("PART A: {}", part_a("input.txt"));
    });
    let t_b = std::thread::spawn(|| {
        println!("PART B: {}", part_b("input.txt"));
    });
    t_a.join().unwrap();
    t_b.join().unwrap();
}
fn part_a(path: &str) -> i64 {
    let map = TopoHeightMap {
        map: read_file_to_num_grid(path),
    };

    let trail_heads = map.get_trail_heads();
    trail_heads
        .iter()
        .map(|&head| {
            let mut ends = HashSet::new();
            let paths = map.walk_uphill_to_end(head, HashSet::new(), &mut ends, true);
            println!("Path from {head:?} has {paths} ends -> {ends:?}");
            paths
        })
        .sum()
}
fn part_b(path: &str) -> i64 {
    let map = TopoHeightMap {
        map: read_file_to_num_grid(path),
    };

    let trail_heads = map.get_trail_heads();
    trail_heads
        .iter()
        .map(|&head| {
            let mut ends = HashSet::new();
            let paths = map.walk_uphill_to_end(head, HashSet::new(), &mut ends, false);
            println!("Path from {head:?} has {paths} ends -> {ends:?}");
            paths
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a_demo() {
        let results = part_a("test.txt");
        assert_eq!(results, 36);
    }
    #[test]
    fn test_part_a_real() {
        let results = part_a("input.txt");
        assert_eq!(results, 624);
    }
    #[test]
    fn test_part_b_demo() {
        let results = part_b("test.txt");
        assert_eq!(results, 81);
    }
    #[test]
    fn test_part_b_real() {
        let results = part_b("input.txt");
        assert_eq!(results, 1483);
    }
}
