use crate::gates::basic::{and, or, xor};
use crate::operators::addition::{three_input_sum_bit, three_input_carry_out_bit};

// Returns one byte and overflow if true, after applying sum and carry out logic 
// bit-by-bit on byte_a, b and c.
fn full_adder_3input_8bit(byte_a: [u8; 8], byte_b: [u8; 8], byte_c: [u8; 8]) -> 
([u8; 8], u8) {

    let mut result_byte = [0; 8];
    let mut carry_out = 0;

    for bit in 0..8 {
        result_byte[bit] = three_input_sum_bit(byte_a[bit], byte_b[bit], 
            byte_c[bit], carry_out);
                                    
        carry_out = three_input_carry_out_bit(byte_a[bit], byte_b[bit], 
            byte_c[bit], carry_out);
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
