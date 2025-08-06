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


/*--------------- 3 Input ----------------*/
// Returns sum bit after applying XOR logic bit-by-bit on,
// bit_a, b, c and carry in bit.
pub fn three_input_sum_bit(bit_a: u8, bit_b: u8, bit_c: u8, carry_in_bit: u8) 
-> u8 {
    xor(xor(xor(bit_a, bit_b), bit_c), carry_in_bit)
}

// Returns carry out bit after applying AND, and OR logic bit-by-bit on,
// bit_a, b, c and carry in bit.
pub fn three_input_carry_out_bit(bit_a: u8, bit_b: u8, bit_c: u8, carry_in_bit: u8) 
-> u8 {
    or(
        or(
            or(
                or(
                    or(and(bit_a, bit_b), and(bit_a, bit_c)), 
                    and(bit_a, carry_in_bit)
                ), 
                and(bit_b, bit_c)
            ), 
            and(bit_b, carry_in_bit)
        ), 
        and(bit_c, carry_in_bit)
    )
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