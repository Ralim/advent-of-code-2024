use bmp::{px, Image, Pixel};
use core::str;
use rayon::prelude::*;
use regex::Regex;
use std::{collections::HashSet, fs};

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
#[derive(Debug, Clone)]
struct Robot {
    position: (i64, i64), // Row-Column, not like questions broken column-row :wink:
    movement_velocity: (i64, i64), //Cells per tick, as above
}
impl Robot {
    fn from_line(line: &str) -> Self {
        assert!(!line.is_empty());
        // p=9,3 v=2,3
        let regex_pos: Regex = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)$").unwrap();
        let (row, col, vertical, horizontal) = regex_pos
            .captures(line)
            .map(|cap| {
                let col = cap[1].parse::<i64>().unwrap();
                let row = cap[2].parse::<i64>().unwrap();
                let horizontal = cap[3].parse::<i64>().unwrap();
                let vertical = cap[4].parse::<i64>().unwrap();
                (row, col, vertical, horizontal)
            })
            .unwrap();
        Self {
            position: (row, col),
            movement_velocity: (vertical, horizontal),
        }
    }
    fn move_robot_around_grid(&mut self, width: i64, height: i64) {
        self.position.0 += self.movement_velocity.0;
        self.position.1 += self.movement_velocity.1;
        if self.position.0 < 0 {
            self.position.0 += height;
        }
        if self.position.1 < 0 {
            self.position.1 += width;
        }

        self.position.0 %= height;
        self.position.1 %= width;
    }
}

fn part_a(path: &str) -> i64 {
    let mut robots: Vec<Robot> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .filter_map(|line| {
            if line.is_empty() {
                None
            } else {
                Some(Robot::from_line(line))
            }
        })
        .collect();
    // assert_eq!(robots.len(), 12);
    let total_ticks = 100;
    let grid_rows = 103;
    let grid_cols = 101;
    for tick in 0..total_ticks {
        println!("Running tick {tick}");

        robots.par_iter_mut().for_each(|robot| {
            robot.move_robot_around_grid(grid_cols, grid_rows);
        });
    }
    println!("Robots {robots:?}");
    let middle_row = grid_rows / 2;
    let middle_col = grid_cols / 2;
    println!("middle {middle_row} {middle_col}");
    let mut quad_counts = [0, 0, 0, 0];
    for robot in robots {
        if robot.position.0 < middle_row && robot.position.1 < middle_col {
            quad_counts[0] += 1;
        } else if robot.position.0 < middle_row && robot.position.1 > middle_col {
            quad_counts[1] += 1;
        } else if robot.position.0 > middle_row && robot.position.1 < middle_col {
            quad_counts[2] += 1;
        } else if robot.position.0 > middle_row && robot.position.1 > middle_col {
            quad_counts[3] += 1;
        }
    }
    println!("Quad counts {quad_counts:?}");
    quad_counts[0] * quad_counts[1] * quad_counts[2] * quad_counts[3]
}
fn print_bots_grid_if_fairly_black(
    robots: &[Robot],
    num_rows: i64,
    num_cols: i64,
    file_name: &str,
) {
    let set_pixels: HashSet<(i64, i64)> = robots.iter().map(|robot| robot.position).collect();
    let mut img = Image::new(num_cols as u32, num_rows as u32);
    for robot_pos in set_pixels {
        img.set_pixel(
            robot_pos.1 as u32,
            robot_pos.0 as u32,
            px!(0xFF, 0xFF, 0xFF),
        );
    }
    img.save(file_name).unwrap();
}
fn part_b(path: &str) -> i64 {
    let mut robots: Vec<Robot> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .filter_map(|line| {
            if line.is_empty() {
                None
            } else {
                Some(Robot::from_line(line))
            }
        })
        .collect();
    // assert_eq!(robots.len(), 12);
    let total_ticks = 8_000;
    let grid_rows = 103;
    let grid_cols = 101;
    fs::create_dir_all("/tmp/partb/").unwrap();
    for tick in 0..total_ticks {
        robots.par_iter_mut().for_each(|robot| {
            robot.move_robot_around_grid(grid_cols, grid_rows);
        });
        print_bots_grid_if_fairly_black(
            &robots,
            grid_rows,
            grid_cols,
            &format!("/tmp/partb/{tick}.bmp"),
        );
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_part_a_demo() {
    //     let results = part_a("test.txt");
    //     assert_eq!(results, 12);
    // }
    #[test]
    fn test_part_a_real() {
        let results = part_a("input.txt");
        assert_eq!(results, 221655456);
    }
    // Part B is a visual test, we dont unit test
    // #[test]
    // fn test_part_b_demo() {
    //     let results = part_b("test.txt");
    //     assert_eq!(results, 0);
    // }
    // #[test]
    // fn test_part_b_real() {
    //     let results = part_b("input.txt");
    //     assert_eq!(results, 0);
    // }
}
