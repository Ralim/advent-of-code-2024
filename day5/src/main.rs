use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

fn main() {
    println!("PART A: {}", part_a("input.txt"));
    println!("PART B: {}", part_b("input.txt"));
}
pub struct OrderRule {
    pub first: i64,
    pub second: i64,
}
pub struct Input {
    rules: HashMap<i64, Vec<OrderRule>>,
    sequences: Vec<Vec<i64>>,
}
impl Input {
    pub fn read_file(file_path: &str) -> Self {
        let mut split = false;
        let mut sequences = Vec::new();
        let mut rules: HashMap<i64, Vec<OrderRule>> = HashMap::new();

        for mut line in std::fs::read_to_string(file_path).unwrap().lines() {
            line = line.trim();
            if line.is_empty() {
                split = true;
                continue;
            }
            if split {
                let mut seq: Vec<i64> = Vec::new();
                for num in line.split(",") {
                    seq.push(num.parse().unwrap());
                }
                sequences.push(seq);
            } else {
                let mut parts = line.split("|");
                let first = parts.next().unwrap().parse().unwrap();
                let second = parts.next().unwrap().parse().unwrap();
                let rule = OrderRule { first, second };
                match rules.get_mut(&second) {
                    Some(existing) => existing.push(rule),
                    None => {
                        rules.insert(second, vec![rule]);
                    }
                };
            }
        }

        Self { rules, sequences }
    }
}
fn check_rules_on_sequence(rules: &HashMap<i64, Vec<OrderRule>>, seq: &Vec<i64>) -> bool {
    let all_values: HashSet<i64> = HashSet::from_iter(seq.iter().cloned());
    assert_eq!(all_values.len(), seq.len());

    let mut seen: HashSet<i64> = HashSet::new();
    //We can check in order now
    for item in seq {
        // Get all rules by this key, to find any numbers that should have come first
        if let Some(matching_rules) = rules.get(item) {
            for matching_rule in matching_rules {
                if all_values.contains(&matching_rule.first) && !seen.contains(&matching_rule.first)
                {
                    return false;
                }
            }
        }
        seen.insert(*item); // We are pst this one now
    }
    true
}
fn do_one_correction_swap(
    rules: &HashMap<i64, Vec<OrderRule>>,
    new_seq: &mut Vec<i64>,
    all_values: &HashSet<i64>,
) -> bool {
    let mut seen: HashSet<i64> = HashSet::with_capacity(new_seq.len());
    for index in 0..new_seq.len() {
        let item = new_seq[index];
        if let Some(matching_rules) = rules.get(&item) {
            for matching_rule in matching_rules {
                if all_values.contains(&matching_rule.first) {
                    //This rule could apply
                    if !seen.contains(&matching_rule.first) {
                        // Rule has been broken, look forward for the violation and swap
                        let index2 = new_seq
                            .iter()
                            .rposition(|&i| i == matching_rule.first)
                            .unwrap();
                        let x = new_seq.remove(index2);
                        new_seq.insert(index, x);

                        // println!("Swapping {} for {} Gave {:?}", index, index2, new_seq);
                        return true;
                    }
                }
            }
        }
        seen.insert(item);
    }
    false
}
fn fix_sequence_by_rules(rules: &HashMap<i64, Vec<OrderRule>>, seq: &[i64]) -> Vec<i64> {
    //Need to put the elements into order when they are out of order
    let mut new_seq = seq.to_vec();
    let all_values: HashSet<i64> = HashSet::from_iter(new_seq.iter().cloned());
    assert_eq!(all_values.len(), new_seq.len());

    // Fuck it, bubble it
    while do_one_correction_swap(rules, &mut new_seq, &all_values) {
        // println!("Seq{:?}", new_seq);
    }

    new_seq
}
fn sequence_middle(seq: &[i64]) -> i64 {
    if seq.len() % 2 == 0 {
        seq[seq.len() / 2 + 1]
    } else {
        seq[seq.len() / 2]
    }
}

fn part_a(path: &str) -> i64 {
    let input = Input::read_file(path);
    println!(
        "Loaded {} fules, and {} sequences",
        input.rules.len(),
        input.sequences.len()
    );

    input
        .sequences
        .par_iter()
        .filter(|seq| check_rules_on_sequence(&input.rules, seq))
        .map(|x| sequence_middle(x))
        .sum::<i64>()
}
fn part_b(path: &str) -> i64 {
    let input = Input::read_file(path);
    println!(
        "Loaded {} fules, and {} sequences",
        input.rules.len(),
        input.sequences.len()
    );

    input
        .sequences
        // .par_iter()
        .iter()
        .filter(|seq| !check_rules_on_sequence(&input.rules, seq))
        .map(|seq| sequence_middle(&fix_sequence_by_rules(&input.rules, seq)))
        .sum::<i64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a_demo() {
        let results = part_a("test.txt");
        assert_eq!(results, 143);
    }
    #[test]
    fn test_part_a_real() {
        let results = part_a("input.txt");
        assert_eq!(results, 7024);
    }
    #[test]
    fn test_part_b_demo() {
        let results = part_b("test.txt");
        assert_eq!(results, 123);
    }
    #[test]
    fn test_part_b_real() {
        let results = part_b("input.txt");
        assert_eq!(results, 4151);
    }
}
