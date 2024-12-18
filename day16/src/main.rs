use array2d::Array2D;
use shared::read_file_to_grid;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::Display,
};

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

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
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
    fn turn_clockwise(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
    fn turn_anti_clockwise(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }
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
// State of a strip, implementing order by cost
#[derive(Debug, Clone, Eq, PartialEq)]
struct PositionState {
    pos: (usize, usize),
    direction: Direction,
    cost: i64,
    path_history: HashSet<(usize, usize)>, // How it got here
}
impl Ord for PositionState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for PositionState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.cost.cmp(&self.cost))
    }
}
struct Map {
    grid: Array2D<u8>,
    start: (usize, usize),
    end: (usize, usize),
    start_direction: Direction,
    cost_forwards: i64,
    cost_rotate: i64,
}
impl Map {
    pub fn from_file(path: &str, cost_forwards: i64, cost_rotate: i64) -> Self {
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
        let start_direction = Direction::Right;
        Self {
            grid,
            start: start_position,
            end: end_position,
            start_direction,
            cost_forwards,
            cost_rotate,
        }
    }
    pub fn find_possible_positions_and_their_cost(
        &self,
        current_position: (usize, usize),
        current_direction: Direction,
    ) -> Vec<(usize, usize, Direction, i64)> {
        let mut results = Vec::new();
        let direction_options = [
            current_direction,
            current_direction.turn_clockwise(),
            current_direction.turn_anti_clockwise(),
        ];

        for direction in direction_options.iter() {
            if let Some(new_position) =
                direction.move_point(&self.grid, current_position.0, current_position.1)
            {
                let value_at_new_position = self.grid[(new_position.0, new_position.1)];
                if value_at_new_position == b'#' {
                    // Walls are NO-STEP
                    continue;
                }
                let cost = if *direction == current_direction {
                    self.cost_forwards
                } else {
                    self.cost_rotate + self.cost_forwards
                };
                assert!(cost > 0);

                results.push((new_position.0, new_position.1, *direction, cost));
            }
        }
        results
    }
    pub fn print(&self, visited: &HashSet<(usize, usize)>) {
        for y in 0..self.grid.num_rows() {
            for x in 0..self.grid.num_columns() {
                if visited.contains(&(y, x)) {
                    print!("*");
                } else {
                    match self.grid.get(y, x) {
                        Some(b'S') => print!("S"),
                        Some(b'#') => print!("#"),
                        Some(b'E') => print!("E"),
                        Some(b'.') => print!("."),
                        _ => panic!("Invalid character"),
                    }
                }
            }
            println!();
        }
    }

    pub fn find_all_paths_to_exit(&self) -> Vec<(i64, HashSet<(usize, usize)>)> {
        //Use binary heap to create a depth-first search across the map
        let mut paths = Vec::new();
        let mut best_final_path_cost = i64::MAX;
        let mut queue = BinaryHeap::new();
        let mut seen = HashMap::new(); // Record each place we have been, and the best seen cost

        queue.push(PositionState {
            pos: self.start,
            direction: self.start_direction,
            cost: 0,
            path_history: HashSet::new(),
        });

        while let Some(PositionState {
            pos,
            direction,
            cost,
            path_history,
        }) = queue.pop()
        {
            //Been here before, truncate path and step backwards if we are at a worse cost

            if let Some(&prev) = seen.get(&(pos, direction)) {
                if cost > prev {
                    continue; // Skip this one as its worse than the last time we were here
                }
                if cost < prev {
                    seen.insert((pos, direction), cost); // Record new value
                }
            } else {
                seen.insert((pos, direction), cost); // Record new value
            }

            if pos == self.end {
                //At end marker
                match cost.cmp(&best_final_path_cost) {
                    Ordering::Greater => {} //its larger than best case so far, yeet it into the distance and keep driving
                    Ordering::Less => {
                        best_final_path_cost = cost;
                        paths.clear(); // New score, so clear older worse ones
                        paths.push((cost, path_history.iter().copied().collect()));
                    }
                    Ordering::Equal => paths.push((cost, path_history.iter().copied().collect())),
                }
            }
            // Not at the end yet, so find all possible next moves and push them to the queue
            for (new_r, new_c, new_dir, new_cost) in
                self.find_possible_positions_and_their_cost(pos, direction)
            {
                let mut new_path_history = path_history.clone();
                new_path_history.insert((new_r, new_c));
                queue.push(PositionState {
                    pos: (new_r, new_c),
                    direction: new_dir,
                    cost: cost + new_cost,
                    path_history: new_path_history,
                });
            }
        }
        paths
    }
}

fn part_a(path: &str) -> i64 {
    let map = Map::from_file(path, 1, 1000);
    map.find_all_paths_to_exit().first().unwrap().0
}
fn part_b(path: &str) -> i64 {
    // Part B requires calculating all the isochrone on the graph

    let map = Map::from_file(path, 1, 1000);
    let all_lowest_paths = map.find_all_paths_to_exit();
    println!("Found {} possible solutions", all_lowest_paths.len());
    //Need to find all tiles covered by the lowest cost path's
    let mut covered_tiles = HashSet::new();
    covered_tiles.insert(map.start);
    for (_, path) in all_lowest_paths.iter() {
        for tile in path.iter() {
            covered_tiles.insert(*tile);
        }
    }
    map.print(&covered_tiles);
    covered_tiles.len() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a_demo() {
        let results = part_a("test.txt");
        assert_eq!(results, 7036);
    }
    #[test]
    fn test_part_a_real() {
        let results = part_a("input.txt");
        assert_eq!(results, 90440);
    }
    #[test]
    fn test_part_b_demo() {
        let results = part_b("test.txt");
        assert_eq!(results, 45);
    }
    #[test]
    fn test_part_b_real() {
        let results = part_b("input.txt");
        assert_eq!(results, 479);
    }
}
