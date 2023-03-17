//! Insertion sort

/// Sorts a mutable slice using in-place insertion sort algorithm.
///
/// Time complexity is `O(n^2)`, where `n` is the number of elements.
/// Space complexity is `O(1)` as it sorts elements in-place.
pub fn sort<T: Ord + Copy>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        let cur = arr[i];
        while j > 0 && cur < arr[j - 1] {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = cur;
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
