use crate::gates::{not, and, or};

/// XOR gate.
/// 
/// - Returns 1 only if exactly one input is 1.
/// 
/// --- Truth Table ---
/// a   b   XOR(a, b)
/// 0   0       0
/// 0   1       1
/// 1   0       1
/// 1   1       0
pub fn xor(bit_a: u8, bit_b: u8) -> u8 {
    or(and(bit_a, not(bit_b)), and(not(bit_a), bit_b))
}

/// XNOR gate.
/// 
/// - Inverse of XOR.
/// - Returns 1 when both inputs are equal.
/// 
/// --- Truth Table ---
/// a   b   XOR   XNOR
/// 0   0    0     1
/// 0   1    1     0
/// 1   0    1     0
/// 1   1    0     1
pub fn xnor(bit_a: u8, bit_b: u8) -> u8 {
    not(xor(bit_a, bit_b)) & 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn computes_xor_truth_table() {
        assert_eq!(xor(0, 0), 0);
        assert_eq!(xor(0, 1), 1);
        assert_eq!(xor(1, 0), 1);
        assert_eq!(xor(1, 1), 0);
    }

    #[test]
    fn computes_xnor_truth_table() {
        assert_eq!(xnor(0, 0), 1);
        assert_eq!(xnor(0, 1), 0);
        assert_eq!(xnor(1, 0), 0);
        assert_eq!(xnor(1, 1), 1);
    }
}
