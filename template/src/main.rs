use rayon::prelude::*;

fn main() {
    println!("PART A: {}", part_a("input.txt"));
    println!("PART B: {}", part_b("input.txt"));
}
fn part_a(path: &str) -> i64 {
    0
}
fn part_b(path: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a_demo() {
        let results = part_a("test.txt");
        assert_eq!(results, 0);
    }
    #[test]
    fn test_part_a_real() {
        let results = part_a("input.txt");
        assert_eq!(results, 0);
    }
    #[test]
    fn test_part_b_demo() {
        let results = part_b("test.txt");
        assert_eq!(results, 0);
    }
    #[test]
    fn test_part_b_real() {
        let results = part_b("input.txt");
        assert_eq!(results, 0);
    }
}
