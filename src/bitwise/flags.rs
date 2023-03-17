/// Computes carry and overflow flags after an addition.
const fn flags(a: u8, b: u8) -> (bool, bool) {
    // compute carry and overflow
    let result = a.wrapping_add(b);
    let carry = result < a;
    let overflow = (a ^ result) >> 7 != (a ^ b) >> 7;
    (carry, overflow)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_flags() {
        assert_eq!(flags(255, 255), (true, false));
        assert_eq!(flags(127, 1), (false, true));
        assert_eq!(flags(1, 1), (false, false));
    }
}
