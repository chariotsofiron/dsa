//! Computes carry and overflow flags after an addition.

/// Computes the carry flag after an addition.
#[must_use]
pub const fn carry(a: u8, b: u8) -> bool {
    // compute carry
    let result = a.wrapping_add(b);
    result < a
}

/// Computes the overflow flag after an addition.
#[must_use]
pub const fn overflow(a: u8, b: u8) -> bool {
    let result = a.wrapping_add(b);
    (a ^ result).wrapping_shr(31) != (a ^ b).wrapping_shr(31)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_carry() {
        assert_eq!(carry(255, 255), true);
        assert_eq!(carry(127, 1), false);
        assert_eq!(carry(1, 1), false);
    }

    #[test]
    fn test_flags() {
        assert_eq!(overflow(255, 255), false);
        assert_eq!(overflow(127, 1), true);
        assert_eq!(overflow(1, 1), false);
    }
}
