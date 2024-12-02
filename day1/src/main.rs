use itertools::Itertools;

fn main() {
    part_a();
    part_b();
}

fn part_a() -> i64 {
    let mut input = shared::read_whitespace_separated_numbers_by_column("input.txt", 2).unwrap();
    assert_eq!(input[0].len(), input[1].len());

    input[0].sort();
    input[1].sort();
    //Elements are sorted small -> large
    // Pair up each set of the two
    let mut distance_total = 0;
    for (a, b) in input[0].iter().zip(input[1].iter()) {
        let distance = (a - b).abs();
        distance_total += distance;
    }
    println!("PART A: Total distance: {}", distance_total);
    distance_total
}
fn part_b() -> i64 {
    let (slice_0, slice_1) = {
        let input = shared::read_whitespace_separated_numbers_by_column("input.txt", 2).unwrap();
        assert_eq!(input[0].len(), input[1].len());
        let mut slice_0 = input[0].clone();
        let mut slice_1 = input[1].clone();
        slice_0.sort();
        slice_1.sort();
        (slice_0, slice_1)
    };

    let indicies: Vec<i64> = slice_0.iter().cloned().unique().collect();

    let mut counters = vec![0; indicies.len()];
    for value in slice_1 {
        if let Ok(index_in_0) = indicies.binary_search(&value) {
            counters[index_in_0] += 1;
        }
    }
    // We now multiply the index with  the matching counter to get the total

    let total = indicies
        .iter()
        .zip(counters.iter())
        .map(|(a, b)| a * b)
        .sum::<i64>();
    println!("PART B: Total distance: {}", total);
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a() {
        let results = part_a();
        assert_eq!(results, 1579939);
    }
    #[test]
    fn test_part_b() {
        let results = part_b();
        assert_eq!(results, 20351745);
    }
}
