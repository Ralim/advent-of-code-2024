use std::collections::{HashSet, VecDeque};

use array2d::Array2D;

fn main() {
    let t_a = std::thread::spawn(|| {
        println!("PART A: {}", part_a("input.txt", 71, 1024));
    });
    let t_b = std::thread::spawn(|| {
        println!("PART B: {}", part_b("input.txt", 71));
    });
    t_a.join().unwrap();
    t_b.join().unwrap();
}
struct MemorySpace {
    grid: Array2D<bool>, //Is corrupt, default false
    instructions: Vec<(usize, usize)>,
    step: usize,
}
impl MemorySpace {
    pub fn new(rows: usize, cols: usize, instructions_file: &str) -> Self {
        //Instructions file is a list of ( col,row) pairs
        let instructions = std::fs::read_to_string(instructions_file)
            .expect("Failed to read file")
            .lines()
            .filter(|l| !l.is_empty())
            .map(|line| {
                let (a, b) = line.split_once(',').unwrap();
                let col = a.parse().unwrap();
                let row = b.parse().unwrap();
                (col, row)
            })
            .collect();
        Self {
            grid: Array2D::filled_with(false, rows, cols),
            instructions,
            step: 0,
        }
    }
    fn get_last_instruction(&self) -> (usize, usize) {
        self.instructions[self.step - 1]
    }
    fn simulate_until(&mut self, steps: usize) {
        while self.step < steps {
            self.next_step();
        }
    }
    fn next_step(&mut self) {
        self.grid[self.instructions[self.step]] = true;
        self.step += 1;
    }
    fn find_steps_to_exit(&self) -> Option<i64> {
        let mut queue = VecDeque::new();
        let mut seen = HashSet::new();
        queue.push_back(((0, 0), 0));

        while let Some((pos, depth)) = queue.pop_front() {
            if pos == (self.grid.num_rows() - 1, self.grid.num_columns() - 1) {
                return Some(depth as i64);
            }

            if !seen.insert(pos) {
                continue;
            }
            // Queue moving in all 4 directions
            // if the grid is false in that location, add it to the queue

            if pos.0 > 0 && self.grid.get(pos.0 - 1, pos.1) == Some(&false) {
                queue.push_back(((pos.0 - 1, pos.1), depth + 1));
            }
            if pos.0 < self.grid.num_rows() - 1 && self.grid.get(pos.0 + 1, pos.1) == Some(&false) {
                queue.push_back(((pos.0 + 1, pos.1), depth + 1));
            }
            if pos.1 > 0 && self.grid.get(pos.0, pos.1 - 1) == Some(&false) {
                queue.push_back(((pos.0, pos.1 - 1), depth + 1));
            }
            if pos.1 < self.grid.num_columns() - 1
                && self.grid.get(pos.0, pos.1 + 1) == Some(&false)
            {
                queue.push_back(((pos.0, pos.1 + 1), depth + 1));
            }
        }

        None
    }
}

fn part_a(path: &str, grid_size: usize, steps: usize) -> i64 {
    let mut memory = MemorySpace::new(grid_size, grid_size, path);
    memory.simulate_until(steps);
    memory.find_steps_to_exit().unwrap()
}
fn part_b(path: &str, grid_size: usize) -> String {
    //TODO, could binary search this, but it takes sub 1 second so who cares
    let mut memory = MemorySpace::new(grid_size, grid_size, path);
    while memory.find_steps_to_exit().is_some() {
        memory.next_step();
    }
    let last_pair = memory.get_last_instruction();
    format!("{},{}", last_pair.0, last_pair.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a_demo() {
        let results = part_a("test.txt", 7, 12);
        assert_eq!(results, 22);
    }
    #[test]
    fn test_part_a_real() {
        let results = part_a("input.txt", 71, 1024);
        assert_eq!(results, 432);
    }
    #[test]
    fn test_part_b_demo() {
        let results = part_b("test.txt", 7);
        assert_eq!(results, "6,1");
    }
    #[test]
    fn test_part_b_real() {
        let results = part_b("input.txt", 71);
        assert_eq!(results, "56,27");
    }
}
