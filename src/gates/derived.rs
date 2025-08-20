use crate::gates::{not, and, or};

/*
fn xor is a logic gate that returns XOR.

- XOR returns true only if one (and only one) input is true.

--- Truth Table ---
a   b   XOR(a, b)
0   0       0
0   1       1
1   0       1
1   1       0
*/
pub fn xor(bit_a: u8, bit_b: u8) -> u8 {
    or(and(bit_a, not(bit_b)), and(not(bit_a), bit_b))
}

/*
fn xnor is a logic gate that returns NOT XOR.

- XOR returns true only if one (and only one) input is true.
- NOT inverts the result.

--- Truth Table ---
a   b   XOR(a, b)   XNOR(a, b)
0   0       0           1
0   1       1           0
1   0       1           0
1   1       0           1
*/
pub fn xnor(bit_a: u8, bit_b: u8) -> u8 {
    not(xor(bit_a, bit_b)) & 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn computes_bit_by_bit_xor_logic() {
        assert_eq!(xor(0, 0), (0));
        assert_eq!(xor(0, 1), (1));
        assert_eq!(xor(1, 0), (1));
        assert_eq!(xor(1, 1), (0));
    }

    #[test]
    fn computes_bit_by_bit_xnor_logic() {
        assert_eq!(xnor(0, 0), (1));
        assert_eq!(xnor(0, 1), (0));
        assert_eq!(xnor(1, 0), (0));
        assert_eq!(xnor(1, 1), (1));
    }
}