/// Checks if a slice is sorted in ascending order
pub fn is_sorted<T: Ord>(data: &[T]) -> bool {
    data.windows(2).all(|w| w[0] <= w[1])
}

/// Checks if a slice is monotonic
pub fn is_strictly_monotonic<T: Ord>(data: &[T]) -> bool {
    if data.len() <= 2 {
        return true;
    }
    let direction = data[0].cmp(&data[1]);
    data.windows(2).all(|w| w[0].cmp(&w[1]) == direction)
}

/// Checks if a slice is strictly increasing
pub fn is_strictly_increasing<T: Ord>(data: &[T]) -> bool {
    if data.len() <= 2 {
        return true;
    }
    data.windows(2).all(|w| w[0].cmp(&w[1]) == std::cmp::Ordering::Less)
}

/// Checks if a slice is strictly decreasing
pub fn is_strictly_decreasing<T: Ord>(data: &[T]) -> bool {
    if data.len() <= 2 {
        return true;
    }
    data.windows(2).all(|w| w[0].cmp(&w[1]) == std::cmp::Ordering::Greater)
}
