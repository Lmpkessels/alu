use crate::gates::basic::{xor, or, and};

// Sum logic, (a XOR b) XOR Carry in, return bit (0 or 1) after logic.
pub fn sum_bit(bit_a: u8, bit_b: u8, carry_in_bit: u8) -> u8{
    xor(xor(bit_a, bit_b), carry_in_bit)
}

// Carry out logic, (a AND b) OR (Carry in AND (a XOR b)), 
// returns bit (0 or 1) after logic.
pub fn carry_out_bit(bit_a: u8, bit_b: u8, carry_in_bit: u8) -> u8 {
    or(and(bit_a, bit_b), and(carry_in_bit, xor(bit_a, bit_b)))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn a_xor_b_is_output_then_output_xor_carry_in_is_sum() {
        let bit_a = 1;
        let bit_b = 0;
        let carry_in_bit = 0;
        let expected_bit = 1;

        assert_eq!(sum_bit(bit_a, bit_b, carry_in_bit), (expected_bit));
    }

    #[test]
    fn a_and_b_is_outp_a_xor_b_and_cin_is_oupt1_then_outp_or_outp1_is_return_value() {
        let bit_a = 0;
        let bit_b = 1;
        let carry_in_bit = 1;
        let expected_bit = 1;

        assert_eq!(carry_out_bit(bit_a, bit_b, carry_in_bit), (expected_bit));
    }
}