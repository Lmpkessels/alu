use crate::operators::addition::{sum_bit, carry_out_bit};

// 4 Bit Adder returning a 4 bit array + overflow confirmation.
fn bit4addr(for_bit_a: [u8; 4], for_bit_b: [u8; 4]) -> ([u8; 4], u8) {
    let mut result_for_bit = [0; 4];
    let mut carry_out = 0;
    
    for bit in (0..4).rev() {
        let sum_bit = sum_bit(for_bit_a[bit], for_bit_b[bit], carry_out);
        carry_out = carry_out_bit(for_bit_a[bit], for_bit_b[bit], carry_out);
        result_for_bit[bit] = sum_bit;
    }

    (result_for_bit, carry_out)
}

#[cfg (test)]
mod test {
    use super::*;

    #[test]
    fn return_array_and_overflow_after_simulating_a_and_b_through_sum_and_carry_out() {
        let for_bit_a = [1, 0, 1, 0];
        let for_bit_b = [0, 0, 1, 0];
        let expected_for_bit = ([1, 1, 0, 0], 0);

        assert_eq!(bit4addr(for_bit_a, for_bit_b), (expected_for_bit));
    }
}