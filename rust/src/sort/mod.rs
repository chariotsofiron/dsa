//! Sorting algorithms
#![allow(clippy::integer_arithmetic, clippy::arithmetic_side_effects)]
pub mod bubble;
pub mod insertion;

#[cfg(test)]
mod tests {
    pub fn test_sorting_algorithm(sort: fn(&mut [i32])) {
        let test_cases = [
            (vec![], vec![]),
            (vec![1], vec![1]),
            (vec![1, 2], vec![1, 2]),
            (vec![2, 1], vec![1, 2]),
        ];
        for (input, expected) in test_cases.iter() {
            let mut arr = input.clone();
            sort(&mut arr);
            assert_eq!(arr, *expected);
        }
    }
}
