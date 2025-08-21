use crate::gates::nand;

/// NOT gate.
/// 
/// - Inverts the input: 0 to 1, 1 to 0.
/// 
/// --- Truth Table ---
/// a   NOT(a)
/// 0     1
/// 1     0
pub fn not(bit_a: u8) -> u8 {
    nand(bit_a, bit_a) & 1
}

/// AND gate.
/// 
/// - Returns 1 only if both inputs are 1.
/// 
/// --- Truth Table ---
/// a   b   AND(a, b)
/// 0   0       0
/// 0   1       0
/// 1   0       0
/// 1   1       1
pub fn and(bit_a: u8, bit_b: u8) -> u8 {
    (bit_a & bit_b) & 1
}

/// OR gate.
/// 
/// - Returns 1 if at least one input is 1.
/// 
/// --- Truth Table ---
/// a   b   OR(a, b)
/// 0   0       0
/// 0   1       1
/// 1   0       1
/// 1   1       1
pub fn or(bit_a: u8, bit_b: u8) -> u8 {
    (bit_a | bit_b) & 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn computes_not_truth_table() {
        assert_eq!(not(0), 1);
        assert_eq!(not(1), 0);
    }

    #[test]
    fn computes_and_truth_table() {
        assert_eq!(and(0, 0), 0);
        assert_eq!(and(0, 1), 0);
        assert_eq!(and(1, 0), 0);
        assert_eq!(and(1, 1), 1);
    }

    #[test]
    fn computes_or_truth_table() {
        assert_eq!(or(0, 0), 0);
        assert_eq!(or(0, 1), 1);
        assert_eq!(or(1, 0), 1);
        assert_eq!(or(1, 1), 1);
    }
}
