use crate::gates::basic::{and, or, xor};

// Returns sum bit after applying XOR logic bit-by-bit on,
// bit_a, b, c and carry in bit.
fn sum_bit(bit_a: u8, bit_b: u8, bit_c: u8, carry_in_bit: u8) -> u8 {
    xor(xor(xor(bit_a, bit_b), bit_c), carry_in_bit)
}

// Returns carry out bit after applying AND, and OR logic bit-by-bit on,
// bit_a, b, c and carry in bit.
fn carry_out_bit(bit_a: u8, bit_b: u8, bit_c: u8, carry_in_bit: u8) -> u8 {
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


// Returns one byte and overflow if true, after applying sum and carry out logic 
// bit-by-bit on byte_a, b and c.
fn full_adder_3input_8bit(byte_a: [u8; 8], byte_b: [u8; 8], byte_c: [u8; 8]) -> 
([u8; 8], u8) {

    let mut result_byte = [0; 8];
    let mut carry_out = 0;

    for bit in 0..8 {
        result_byte[bit] = sum_bit(byte_a[bit], byte_b[bit], byte_c[bit], carry_out);
                                    
        carry_out = carry_out_bit(byte_a[bit], byte_b[bit], byte_c[bit], carry_out);
    }

    (result_byte, carry_out)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn apply_sum_and_carry_out_return_result_array_with_last_carry() {
        let byte_a = [0, 1, 0, 1, 0, 1, 0, 1]; 
        let byte_b = [1, 1, 0, 0, 1, 1, 0, 1];
        let byte_c = [0, 0, 0, 1, 1, 0, 0, 1];
        let expected = ([1, 0, 1, 0, 1, 1, 1, 1], 1);

        assert_eq!(full_adder_3input_8bit(byte_a, byte_b, byte_c), expected);
    }
}
