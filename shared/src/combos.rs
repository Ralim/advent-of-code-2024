use itertools::{repeat_n, Itertools};

pub fn create_all_possible_operations<T>(set: &[T], num_elements: usize) -> Vec<Vec<T>>
where
    T: Clone,
{
    //Create all combinations of operations for the number of elements
    let mut results = vec![];
    for combination in repeat_n(set.iter(), num_elements).multi_cartesian_product() {
        // println!("{:?}", combination);
        results.push(combination.into_iter().cloned().collect());
    }
    results
}
