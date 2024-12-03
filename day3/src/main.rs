use lazy_static::lazy_static;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use regex::Regex;
use shared::read_whitespace_separated_numbers_by_row;

fn main() {
    println!("PART A: {}", part_a("input.txt"));
    println!("PART B: {}", part_b("input.txt"));
}

fn part_a(path: &str) -> i64 {
    let data = std::fs::read_to_string(path).unwrap();

    // iterate over all matches
    let r: Regex = Regex::new(r"(mul\([0-9]{1,3},[0-9]{1,3}\))").unwrap();
    let matches: Vec<&str> = r
        .find_iter(&data)
        // try to parse the string matches as i64 (inferred from fn type signature)
        // and filter out the matches that can't be parsed (e.g. if there are too many digits to store in an i64).
        .map(|val| val.as_str())
        // collect the results in to a Vec<i64> (inferred from fn type signature)
        .collect();
    // Stage 2, break down the individual matches in to the two numbers and multiply them

    matches.par_iter().map(|m| compute_mul(m)).sum()
}
fn part_b(path: &str) -> i64 {
    let mut data = std::fs::read_to_string(path).unwrap();
    data = data.replace("\r", "");
    data = data.replace("\n", "");

    let regex_find_instructions: Regex = Regex::new(r"(mul\([0-9]{1,3},[0-9]{1,3}\))").unwrap();
    let regex_find_do: Regex = Regex::new(r"do\(\)").unwrap();
    let regex_find_dont: Regex = Regex::new(r"don't\(\)").unwrap();
    // Find every match; and its index in the data
    let mul_instructions: Vec<(usize, &str)> = regex_find_instructions
        .find_iter(&data)
        .map(|val| (val.start(), val.as_str()))
        .collect();
    let indicies_of_dos: Vec<(usize, bool)> = regex_find_do
        .find_iter(&data)
        .map(|val| (val.start(), true))
        .collect();
    let indicies_of_donts: Vec<(usize, bool)> = regex_find_dont
        .find_iter(&data)
        .map(|val| (val.start(), false))
        .collect();

    let mut control_map: Vec<&(usize, bool)> = indicies_of_dos
        .iter()
        .chain(indicies_of_donts.iter())
        .collect();
    // Sort by index 0->N
    control_map.sort_by(|a, b| a.0.cmp(&b.0));

    // Can now iterate over the mul instructions and do the mul if its in a region of do's
    let mut enabled = true;
    let mut control_map_iter = control_map.iter().peekable();

    let mut total = 0;

    for (index, instr) in mul_instructions {
        //Check if new control map
        if let Some(next_control) = control_map_iter.peek() {
            if next_control.0 < index {
                enabled = next_control.1;
                control_map_iter.next();
            }
        }
        if enabled {
            let result = compute_mul(instr);
            total += result;
        }
    }
    total
}

fn compute_mul(instr: &str) -> i64 {
    lazy_static! {
        static ref regex_2: Regex = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    }

    regex_2
        .captures(instr)
        .map(|cap| {
            let a = cap[1].parse::<i64>().unwrap();
            let b = cap[2].parse::<i64>().unwrap();
            a * b
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a_demo() {
        let results = part_a("test.txt");
        assert_eq!(results, 161);
    }
    #[test]
    fn test_part_a_real() {
        let results = part_a("input.txt");
        assert_eq!(results, 190604937);
    }
    #[test]
    fn test_part_b_demo() {
        let results = part_b("test2.txt");
        assert_eq!(results, 48);
    }
    #[test]
    fn test_part_b_real() {
        let results = part_b("input.txt");
        assert_eq!(results, 82857512);
    }
}
