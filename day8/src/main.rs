use std::collections::HashMap;

use array2d::Array2D;
use itertools::Itertools;
use rayon::prelude::*;
use shared::read_file_to_grid;

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
    let grid = read_file_to_grid(path);
    let antennas = all_unqiue_antennas_with_more_than_one(&grid);
    // For each antenna, find the antinodes
    let all_antinodes: Vec<(i64, i64)> = antennas
        .par_iter()
        .map(|antenna| find_all_antinodes(&grid, *antenna))
        .flatten()
        .collect();
    // Count number of unqiue antinodes
    all_antinodes.iter().unique().count() as i64
}
fn part_b(path: &str) -> i64 {
    let grid = read_file_to_grid(path);
    let antennas = all_unqiue_antennas_with_more_than_one(&grid);
    // For each antenna, find the antinodes
    let all_antinodes: Vec<(i64, i64)> = antennas
        .par_iter()
        .map(|antenna| find_all_lined_nodes(&grid, *antenna))
        .flatten()
        .collect();
    // Count number of unqiue antinodes
    all_antinodes.iter().unique().count() as i64
}
fn find_location_of_antenna(data: &Array2D<u8>, antenna: u8) -> Vec<(i64, i64)> {
    //Find all rows,columns in data that have antenna
    data.enumerate_row_major()
        .filter(|(_, &x)| x == antenna)
        .map(|(i, _)| (i.0 as i64, i.1 as i64))
        .collect()
}

fn find_all_lined_nodes(data: &Array2D<u8>, antenna: u8) -> Vec<(i64, i64)> {
    //The signal only applies its nefarious effect at specific antinodes based on the resonant frequencies of the antennas.
    // In particular, an antinode occurs at any point that is perfectly in line with two antennas of the same frequency -
    // but only when one of the antennas is twice as far away as the other.
    // This means that for any pair of antennas with the same frequency, there are two antinodes, one on either side of them.

    // Therefore an antinode occurs when  d(a1) =2* d(a2)
    // i.e we take the vector from a1 to a2, and then that distance again from a2

    let grid_rows = data.num_rows() as i64;
    let grid_columns = data.num_columns() as i64;

    let all_locations = find_location_of_antenna(data, antenna);

    // If on the line from A->B cross product shall be 0

    all_locations
        .iter()
        .combinations(2)
        .flat_map(|p: Vec<&(i64, i64)>| {
            let (x1, y1) = p[0];
            let (x2, y2) = p[1];

            (0..grid_rows).map(move |row| {
                (0..grid_columns)
                    .filter_map(move |col| {
                        let dxc = row - x1;
                        let dyc = col - y1;
                        let dxl = x2 - x1;
                        let dyl = y2 - y1;

                        let cross_product = dxc * dyl - dyc * dxl;
                        if cross_product == 0 {
                            Some((row, col))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<(i64, i64)>>()
            })
        })
        .flatten()
        .collect::<Vec<(i64, i64)>>()
}
fn find_all_antinodes(data: &Array2D<u8>, antenna: u8) -> Vec<(i64, i64)> {
    //The signal only applies its nefarious effect at specific antinodes based on the resonant frequencies of the antennas.
    // In particular, an antinode occurs at any point that is perfectly in line with two antennas of the same frequency -
    // but only when one of the antennas is twice as far away as the other.
    // This means that for any pair of antennas with the same frequency, there are two antinodes, one on either side of them.

    // Therefore an antinode occurs when  d(a1) =2* d(a2)
    // i.e we take the vector from a1 to a2, and then that distance again from a2

    let all_locations = find_location_of_antenna(data, antenna);
    // println!("Anntena locations for {} {:?}", antenna, all_locations);
    let a2_jump = all_locations
        .iter()
        .combinations(2)
        .map(|p: Vec<&(i64, i64)>| {
            let (x1, y1) = p[0];
            let (x2, y2) = p[1];
            let dx = x2 - x1;
            let dy = y2 - y1;
            // println!("{} {} {} {} -> {:?}", x1, y1, x2, y2, (x2 + dx, y2 + dy));
            (x2 + dx, y2 + dy)
        });
    let a1_jump = all_locations
        .iter()
        .combinations(2)
        .map(|p: Vec<&(i64, i64)>| {
            let (x1, y1) = p[0];
            let (x2, y2) = p[1];
            let dx = x2 - x1;
            let dy = y2 - y1;
            (x1 - dx, y1 - dy)
        });
    // Filter out the points that are outside the bounds of the grid
    a1_jump
        .chain(a2_jump)
        .filter(|(x, y)| {
            (*x >= 0 && *x < data.num_rows() as i64) && (*y >= 0 && *y < data.num_columns() as i64)
        })
        .collect()
}
fn all_unqiue_antennas_with_more_than_one(data: &Array2D<u8>) -> Vec<u8> {
    let mut items: HashMap<u8, usize> = HashMap::with_capacity(128);
    data.elements_row_major_iter().for_each(|x| {
        let count = items.entry(*x).or_insert(0);
        *count += 1;
    });

    items
        .iter()
        .filter(|(_, v)| **v > 1)
        .filter(|(x, _)| **x != b'.')
        .map(|(k, _)| *k)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a_demo() {
        let results = part_a("test.txt");
        assert_eq!(results, 14);
    }
    #[test]
    fn test_part_a_real() {
        let results = part_a("input.txt");
        assert_eq!(results, 396);
    }
    #[test]
    fn test_part_b_demo() {
        let results = part_b("test.txt");
        assert_eq!(results, 34);
    }
    #[test]
    fn test_part_b_real() {
        let results = part_b("input.txt");
        assert_eq!(results, 1200);
    }
}
