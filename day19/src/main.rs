use std::collections::{HashMap, VecDeque};

use rayon::prelude::*;

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
struct TowelSet {
    available_patterns: Vec<String>,
    designs: Vec<String>,
}
impl TowelSet {
    pub fn from_file(path: &str) -> Self {
        //Read first line as CSV to find the combinations
        let file = std::fs::read_to_string(path).unwrap();
        let mut lines = file.lines();

        let mut available_patterns: Vec<String> = lines
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        available_patterns.sort_by_key(|a| a.len());

        lines.next(); // Drop spacer

        // All remaining lines are the requested patterns
        let designs: Vec<String> = lines.map(|s| s.trim().to_string()).collect();

        Self {
            available_patterns,
            designs,
        }
    }
    fn count_solutions_to_pattern<'a>(
        &self,
        lookup_cache: &mut HashMap<&'a str, i64>,
        pattern: &'a str,
    ) -> i64 {
        if let Some(&entry) = lookup_cache.get(pattern) {
            return entry;
        }
        // No pattern remaining to match, we have found a solution
        if pattern.is_empty() {
            return 1;
        }

        // For each segment, we recursively check if we can match the pattern by adding it
        // If we can, we add the number of ways to make the remaining pattern
        let ways = self
            .available_patterns
            .iter()
            .filter(|extra| pattern.len() >= extra.len() && pattern.starts_with(*extra))
            .map(|extra| self.count_solutions_to_pattern(lookup_cache, &pattern[extra.len()..]))
            .sum();
        // Cache the result for future lookups up to this point
        lookup_cache.insert(pattern, ways);
        ways
    }
    fn is_pattern_makeable(&self, pattern: &str) -> bool {
        let mut queue = VecDeque::new();
        queue.push_back("".to_string());

        while let Some(progress) = queue.pop_back() {
            // pop at the back to make it depth-first to keep ram level sane
            if progress.len() > pattern.len() {
                continue;
            }
            let next_pattern_chunk = &pattern[progress.len()..];
            let possible_next_patterns = self
                .available_patterns
                .iter()
                .filter(|p| next_pattern_chunk.starts_with(*p));
            for p in possible_next_patterns {
                let new_progress = progress.to_owned() + p;
                if new_progress == pattern {
                    return true;
                }
                queue.push_back(new_progress);
            }
        }
        false
    }
}
fn part_a(path: &str) -> i64 {
    let towel_set = TowelSet::from_file(path);
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(8)
        .build()
        .unwrap();

    pool.install(|| {
        towel_set
            .designs
            .par_iter()
            .map(|d| match towel_set.is_pattern_makeable(d) {
                true => 1,
                false => 0,
            })
            .sum()
    })
}
fn part_b(path: &str) -> i64 {
    let towel_set = TowelSet::from_file(path);
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(8)
        .build()
        .unwrap();

    pool.install(|| {
        towel_set
            .designs
            .par_iter()
            .filter(|p| towel_set.is_pattern_makeable(p))
            .map(|d| towel_set.count_solutions_to_pattern(&mut HashMap::with_capacity(2048), d))
            .sum()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a_demo() {
        let results = part_a("test.txt");
        assert_eq!(results, 6);
    }
    #[test]
    fn test_part_a_real() {
        let results = part_a("input.txt");
        assert_eq!(results, 216);
    }
    #[test]
    fn test_part_b_demo() {
        let results = part_b("test.txt");
        assert_eq!(results, 16);
    }
    #[test]
    fn test_part_b_real() {
        let results = part_b("input.txt");
        assert_eq!(results, 603191454138773);
    }
}
