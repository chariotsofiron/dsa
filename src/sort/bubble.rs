//! Bubble sort

/// Sorts a mutable slice using in-place bubble sort algorithm.
pub fn sort<T: Ord>(arr: &mut [T]) {
    if arr.is_empty() {
        return;
    }
    let mut sorted = false;
    let mut n = arr.len();
    while !sorted {
        sorted = true;
        for i in 0..n - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                sorted = false;
            }
        }
        n -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::sort;
    use crate::sort::tests::test_sorting_algorithm;

    #[test]
    fn test() {
        test_sorting_algorithm(sort);
    }
}
