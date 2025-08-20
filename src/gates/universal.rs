use crate::gates::{not, or};

/*
fn nand is a logic gate that returns NOT AND.

- AND returns true only if both inputs are true.
- NOT inverts the result.

--- Truth Table ---
a   b   AND(a, b)   NAND(a, b)
0   0       0           1
0   1       0           1
1   0       0           1
1   1       1           0
*/
pub fn nand(bit_a: u8, bit_b: u8) -> u8 {
    !(bit_a & bit_b) & 1
}

/*
fn nor is a logic gate that returns NOT OR.

- OR returns true if at least one input is true.
- NOT inverts the result.

--- Truth Table ---
a   b   OR(a, b)   NOR(a, b)
0   0       0           1
0   1       1           0
1   0       1           0
1   1       1           0
*/
pub fn nor(bit_a: u8, bit_b: u8) -> u8 {
    not(or(bit_a, bit_b)) & 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn computes_bit_by_bit_nand_logic() {
        assert_eq!(nand(0, 0), (1));    
        assert_eq!(nand(0, 1), (1));    
        assert_eq!(nand(1, 0), (1));    
        assert_eq!(nand(1, 1), (0));    
    }

    #[test]
    fn computes_bit_by_bit_nor_logic() {
        assert_eq!(nor(0, 0), (1));
        assert_eq!(nor(0, 1), (0));
        assert_eq!(nor(1, 0), (0));
        assert_eq!(nor(1, 1), (0));
    }
}