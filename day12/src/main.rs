use array2d::Array2D;
use core::str;
use rayon::prelude::*;
use shared::{print_array, read_file_to_grid, rotate_array, ALL_DIRECTIONS};
use std::collections::HashSet;

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
#[derive(Debug)]
struct Region {
    cells: HashSet<(usize, usize)>, // All the locations in this region
}
impl Region {
    pub fn find_and_subtract(grid: &mut Array2D<u8>, start: (usize, usize)) -> Self {
        //From the start point, explore out to find all the cells that are the same value as the start point
        let tag = *grid.get(start.0, start.1).unwrap();
        grid.set(start.0, start.1, 0).unwrap(); // Remove start
        let mut cells = Self::explore_grid(grid, start, tag);
        cells.insert(start);
        Self { cells }
    }
    fn explore_grid(
        grid: &mut Array2D<u8>,
        point: (usize, usize),
        target_value: u8,
    ) -> HashSet<(usize, usize)> {
        ALL_DIRECTIONS
            .iter()
            .filter_map(|dir| {
                if let Some(new_point) = dir.move_point(grid, point.0, point.1) {
                    let value = grid.get(new_point.0, new_point.1).unwrap();
                    if *value == target_value {
                        //This location had a valid spot
                        grid.set(new_point.0, new_point.1, 0).unwrap(); // Nop out the point as we have "taken" it
                        let mut new_points = Self::explore_grid(grid, new_point, target_value);
                        assert!(new_points.insert(new_point));
                        Some(new_points)
                    } else {
                        None // End of the line bud
                    }
                } else {
                    None
                }
            })
            .flatten()
            .collect()
    }
    pub fn area(&self) -> usize {
        self.cells.len()
    }
    pub fn side_count(&self, row_num: usize, col_num: usize) -> usize {
        // All of the cells make up a polygon, the side count is the number of distinct sides
        // i.e. two cells above each other share a side

        // Find all top-horizontal edges
        // We have an edge when there is a cell in this row, and no cell in row above
        // And we count increase counter on gaps
        (0..row_num)
            .into_par_iter()
            .map(|r| {
                let mut seen_cell = false;
                let mut edge_count = 0;
                for c in 0..col_num {
                    // If there is a cell above, skip, not an edge
                    if r > 0 && self.cells.contains(&(r - 1, c)) {
                        seen_cell = false;
                        continue;
                    }
                    if self.cells.contains(&(r, c)) {
                        if !seen_cell {
                            //Start of new edge
                            // println!(
                            //     "New edge{} at {r} {c}",
                            //     str::from_utf8(&[self.tag]).unwrap()
                            // );
                            edge_count += 1;
                        }
                        seen_cell = true;
                    } else {
                        seen_cell = false;
                    }
                }
                edge_count
            })
            .sum()
    }
    pub fn circumference(&self) -> usize {
        //Calculate the circumference of the region including inner holes
        // For each cell, check if it has a neighbour that is not part of the region
        // If it does, then that "direction" is an edge
        self.cells
            .par_iter()
            .map(|(r, c)| {
                let mut circumference = 0;
                if !self.cells.contains(&(r + 1, *c)) {
                    circumference += 1;
                }
                //This is a gaurenteed edge or test
                if *r == 0 || !self.cells.contains(&(r - 1, *c)) {
                    circumference += 1;
                }
                if !self.cells.contains(&(*r, c + 1)) {
                    circumference += 1;
                }
                //This is a gaurenteed edge or test
                if *c == 0 || !self.cells.contains(&(*r, c - 1)) {
                    circumference += 1;
                }
                circumference
            })
            .sum()
    }
}
fn find_next_grid_spot(grid: &Array2D<u8>) -> Option<(usize, usize)> {
    for r in 0..grid.column_len() {
        for c in 0..grid.row_len() {
            if *grid.get(r, c).unwrap() > 0 {
                return Some((r, c));
            }
        }
    }
    None
}
fn part_a(path: &str) -> i64 {
    let mut grid = read_file_to_grid(path);
    // Find all regions in the matrix
    let mut regions = Vec::with_capacity(100);
    while let Some(start) = find_next_grid_spot(&grid) {
        let region = Region::find_and_subtract(&mut grid, start);
        regions.push(region);
    }
    regions
        .iter()
        .map(|r| r.area() * r.circumference())
        .sum::<usize>() as i64
}
fn regionise_and_get_top_edge_of_grid(grid: &mut Array2D<u8>) -> i64 {
    // Find all regions in the matrix
    let mut regions = Vec::with_capacity(100);
    while let Some(start) = find_next_grid_spot(grid) {
        let region = Region::find_and_subtract(grid, start);
        regions.push(region);
    }
    regions
        .iter()
        .map(|r| {
            let area = r.area();
            let side_count = r.side_count(grid.num_rows(), grid.num_columns());
            area * side_count
        })
        .sum::<usize>() as i64
}
fn part_b(path: &str) -> i64 {
    let mut grid = read_file_to_grid(path);
    let mut grid_a = rotate_array(grid.clone());
    let mut grid_b = rotate_array(grid_a.clone());
    let mut grid_c = rotate_array(grid_b.clone());
    print_array(&grid);
    print_array(&grid_a);
    // print_array(&grid_b);
    regionise_and_get_top_edge_of_grid(&mut grid)
        + regionise_and_get_top_edge_of_grid(&mut grid_a)
        + regionise_and_get_top_edge_of_grid(&mut grid_b)
        + regionise_and_get_top_edge_of_grid(&mut grid_c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a_demo() {
        let results = part_a("test.txt");
        assert_eq!(results, 1930);
    }
    #[test]
    fn test_part_a_real() {
        let results = part_a("input.txt");
        assert_eq!(results, 1457298);
    }
    #[test]
    fn test_part_b_demo() {
        let results = part_b("test.txt");
        assert_eq!(results, 1206);
    }
    #[test]
    fn test_part_b_real() {
        let results = part_b("input.txt");
        assert_eq!(results, 921636);
    }
}
