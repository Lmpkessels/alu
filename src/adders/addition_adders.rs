use crate::operators::addition::{sum_bit, carry_out_bit};

pub fn one_bit_full_adder(bit_a: u8, bit_b: u8, carry_in_bit: u8) -> (u8, u8) {
    (sum_bit(bit_a, bit_b, carry_in_bit), carry_out_bit(bit_a, bit_b, carry_in_bit))
}

// eight bit full adder, receiving as argument two arrays.
// Fn returns a byte + overflow.
pub fn addition_32_bit(byte_a: [u8; 32], byte_b: [u8; 32]) -> [u8; 32] {
    let mut result_byte = [0; 32];
    let mut carry_out_bit = 0;

    for bit in (0..32).rev() {
        let (sum_bit, carry_in_bit) = one_bit_full_adder(byte_a[bit], byte_b[bit], 
            carry_out_bit);

        result_byte[bit] = sum_bit;
        
        carry_out_bit = carry_in_bit;
    }

    result_byte
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_bit_adder_returns_correct_sum_and_carry() {
        assert_eq!(one_bit_full_adder(0, 0, 0), (0, 0));
        assert_eq!(one_bit_full_adder(0, 1, 0), (1, 0));
        assert_eq!(one_bit_full_adder(1, 0, 1), (0, 1));
        assert_eq!(one_bit_full_adder(1, 1, 1), (1, 1));
    }

    #[test]
    fn thirty_two_adder_correctly_adds_without_overflow() {
        let bit_a = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
        0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0];
        let bit_b = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
        0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0];
        let expected = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
            0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0];
        
        
        assert_eq!(addition_32_bit(bit_a, bit_b), (expected));
    }
}
