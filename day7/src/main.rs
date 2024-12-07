use count_digits::CountDigits;
use rayon::prelude::*;
use shared::create_all_possible_operations;

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
fn test_equation_solve_by_operations(set: &[Operations], equation: &Equation) -> bool {
    let mut all_operations = create_all_possible_operations(set, equation.inputs.len() - 1);
    all_operations.any(|ops| equation.solve_matches(&ops, equation.test_value))
}
fn part_a(path: &str) -> i64 {
    let equations = read_file_to_vec(path);
    // Count how many are solvable with base operations
    equations
        .par_iter()
        .filter(|x| test_equation_solve_by_operations(&ALL_OPERATIONS_A, x))
        .map(|x| x.test_value)
        .sum::<i64>() as i64
}
fn part_b(path: &str) -> i64 {
    let equations = read_file_to_vec(path);
    // Count how many are solvable with concat
    equations
        .par_iter()
        .filter(|x| test_equation_solve_by_operations(&ALL_OPERATIONS_B, x))
        .map(|x| x.test_value)
        .sum::<i64>() as i64
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Operations {
    Add,
    Multiply,
    Concatenate,
}
const ALL_OPERATIONS_A: [Operations; 2] = [Operations::Add, Operations::Multiply];
const ALL_OPERATIONS_B: [Operations; 3] = [
    Operations::Add,
    Operations::Multiply,
    Operations::Concatenate,
];
struct Equation {
    test_value: i64,
    inputs: Vec<i64>,
}
impl Equation {
    fn solve_matches(&self, operations: &[&Operations], target: i64) -> bool {
        let mut total = self.inputs[0];
        for (i, op) in operations.iter().enumerate() {
            match op {
                Operations::Add => total += self.inputs[i + 1],
                Operations::Multiply => total *= self.inputs[i + 1],
                Operations::Concatenate => {
                    //Shift the total by the number of digits in the next number
                    let next = self.inputs[i + 1];
                    let shift = next.count_digits();
                    total = (total * 10_i64.pow(shift as u32)) + next;
                }
            }
            if total > target {
                return false;
            }
        }
        total == target
    }
}
impl From<&str> for Equation {
    fn from(input: &str) -> Self {
        // Split test_value from the front by ':' and then split the rest by ' '
        let mut parts = input.split(":");
        let test_value = parts.next().unwrap().parse().unwrap();
        let inputs = parts
            .next()
            .unwrap()
            .split(" ")
            .filter(|l| !l.is_empty())
            .map(|x| x.parse().unwrap())
            .collect();
        Equation { test_value, inputs }
    }
}
fn read_file_to_vec(path: &str) -> Vec<Equation> {
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .filter(|l| !l.is_empty())
        .map(|x| x.into())
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a_demo() {
        let results = part_a("test.txt");
        assert_eq!(results, 3749);
    }
    #[test]
    fn test_part_a_real() {
        let results = part_a("input.txt");
        assert_eq!(results, 303766880536);
    }
    #[test]
    fn test_part_b_demo() {
        let results = part_b("test.txt");
        assert_eq!(results, 11387);
    }
    #[test]
    fn test_part_b_real() {
        let results = part_b("input.txt");
        assert_eq!(results, 337041851384440);
    }
}
