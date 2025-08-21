use crate::gates::{not, or};

/// NAND gate.
/// 
/// - Inverse of AND.
/// - Returns 1 except when both inputs are 1.
/// 
/// --- Truth Table ---
/// a   b   AND   NAND
/// 0   0    0     1
/// 0   1    0     1
/// 1   0    0     1
/// 1   1    1     0
pub fn nand(bit_a: u8, bit_b: u8) -> u8 {
    !(bit_a & bit_b) & 1
}

/// NOR gate.
/// 
/// - Inverse of OR.
/// - Returns 1 only if both inputs are 0.
/// 
/// --- Truth Table ---
/// a   b   OR   NOR
/// 0   0   0    1
/// 0   1   1    0
/// 1   0   1    0
/// 1   1   1    0
pub fn nor(bit_a: u8, bit_b: u8) -> u8 {
    not(or(bit_a, bit_b)) & 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn computes_nand_truth_table() {
        assert_eq!(nand(0, 0), 1);    
        assert_eq!(nand(0, 1), 1);    
        assert_eq!(nand(1, 0), 1);    
        assert_eq!(nand(1, 1), 0);    
    }

    #[test]
    fn computes_nor_truth_table() {
        assert_eq!(nor(0, 0), 1);
        assert_eq!(nor(0, 1), 0);
        assert_eq!(nor(1, 0), 0);
        assert_eq!(nor(1, 1), 0);
    }
}
