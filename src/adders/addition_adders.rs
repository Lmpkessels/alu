use crate::operators::addition::{sum_bit, carry_out_bit};

fn one_bit_full_adder(bit_a: u8, bit_b: u8, carry_in_bit: u8) -> (u8, u8) {
    (sum_bit(bit_a, bit_b, carry_in_bit), carry_out_bit(bit_a, bit_b, carry_in_bit))
}

// eight bit full adder, receiving as argument two arrays.
// Fn returns a array with 8 boolean expressions + overflow.
fn eight_bit_full_adder(byte_a: [u8; 8], byte_b: [u8; 8]) -> ([u8; 8], u8) {
    let mut result_byte = [0; 8];
    // Carry out, by default false.
    let mut carry_out_bit = 0;

    // Get (i) in range 0..8. Then reverse to start at righ-most-bit (lsb).
    for bit in (0..8).rev() {
        // Function creating a total then assigning it to sum and cin.
        let (sum_bit, carry_in_bit) = one_bit_full_adder(byte_a[bit], byte_b[bit], carry_out_bit);
        // Implement addr in total[i].
        result_byte[bit] = sum_bit;
        // Carry out is overflow.
        carry_out_bit = carry_in_bit;
    }

    (result_byte, carry_out_bit)
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
    fn eight_bit_adder_correctly_adds_without_overflow() {
        let bit_a = [0, 1, 1, 0, 1, 0, 1, 0];
        let bit_b = [1, 0, 1, 0, 1, 0, 1, 0];
        let expected = ([0, 0, 0, 1, 0, 1, 0, 0], 1);
        
        assert_eq!(eight_bit_full_adder(bit_a, bit_b), (expected));
    }

    #[test]
    fn eight_bit_adder_handles_full_overflow() {
        let byte_a = [1; 8];
        let byte_b = [1; 8];
        let expected_sum_byte = [1, 1, 1, 1, 1, 1, 1, 0];
        let expected_carry_out_bit = 1;

        assert_eq!(eight_bit_full_adder(byte_a, byte_b), 
        (expected_sum_byte, expected_carry_out_bit));
    }
}
