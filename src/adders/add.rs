use crate::gates::{or, and, xor};

/// 32-bit addition.
/// 
/// Ripple-carry adder logic (LSB → MSB):
/// - Sum:  a[bit] XOR b[bit] XOR carry_in
/// - Carry: (a[bit] AND b[bit]) OR (carry_in AND (a[bit] XOR b[bit]))
/// 
/// Returns: 32-bit sum word.
pub fn add_32bit_o(left_operand: [u8; 32], right_operand: [u8; 32]) -> ([u8; 32], u8) {
    let mut sum = [0u8; 32];
    let mut cout_bit = 0;

    // Start from least significant bit (rightmost = index 31)
    for bit in (0..32).rev() {
        let sum_bit = xor(xor(left_operand[bit], right_operand[bit]), cout_bit);

        let cin_bit = or(
            and(left_operand[bit], right_operand[bit]),
            and(cout_bit, xor(left_operand[bit], right_operand[bit])),
        );

        sum[bit] = sum_bit;
        cout_bit = cin_bit;
    }

    // Cout bit indicates (Overflow)
    (sum, cout_bit)
}

/// 32-bit addition without overflow.
///
/// No overflow is returned so that divisor function can work with isolated
/// arrays.
pub fn add_32bit_no(left_operand: [u8; 32], right_operand: [u8; 32]) -> [u8; 32] {
    let mut sum = [0u8; 32];
    let mut cout_bit = 0;

    // Start from least significant bit (rightmost = index 31)
    for bit in (0..32).rev() {
        let sum_bit = xor(xor(left_operand[bit], right_operand[bit]), cout_bit);

        let cin_bit = or(
            and(left_operand[bit], right_operand[bit]),
            and(cout_bit, xor(left_operand[bit], right_operand[bit])),
        );

        sum[bit] = sum_bit;
        cout_bit = cin_bit;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_32bit_computes_a_plus_b_and_returns_sum() {
        let left_operand = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
        0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0];
        let right_operand = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
        0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0];
        
        let result = add_32bit_o(left_operand, right_operand);
        let expected = ([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
            0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0], 0);
        
        assert_eq!((result), (expected));
    }
}
