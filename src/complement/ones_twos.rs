use crate::adders::addition_adders::{eight_bit_full_adder};

// Applies ones complement by receiving an array and applying XOR bit-by-bit
// against an array of only active bit positions.
fn ones_complement(input_byte: [u8; 8]) -> [u8; 8] {
    let mut result = [1; 8];

    for i in 0..8 {
        result[i] ^= input_byte[i];
    }

    result
}

// Two's complement applied using 8-bit full adder with sum and carry out, 
// starting at (LSB) moving to (MSB). 
fn twos_complement(input_byte: [u8; 8]) -> ([u8; 8], u8) {
    let increment = [0, 0, 0, 0, 0, 0, 0, 1];

    eight_bit_full_adder(input_byte, increment)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn applies_ones_complement_on_array() {
        let input = [0, 0, 1, 1, 0, 1, 1, 0];
        let result = ones_complement(input);
        let expected = [1, 1, 0, 0, 1, 0, 0, 1];

        assert_eq!((result), (expected));
    }

    #[test]
    fn applies_twos_complement_on_array() {
        let input = [0, 0, 1, 1, 0, 1, 1, 0];
        let ones_complement = ones_complement(input);
        let result = twos_complement(ones_complement);
        let expected = ([1, 1, 0, 0, 1, 0, 1, 0], 0);

        assert_eq!((result), (expected));
    }
}