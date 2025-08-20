use crate::gates::{not, and, or, xor};

// Subtraction 8 bit full adder starting at (LSB) moving to (MSB) while applying
// bit-by-bit difference and borrow out.
//
// Returns the result byte and underflow if detected.
pub fn sub_32bit(minuend: [u8; 32], subtrahend: [u8; 32]) -> [u8; 32] {
    let mut difference = [0; 32];
    let mut bout_bit = 0;

    for bit in (0..32).rev() {
        let diff_bit = xor(xor(minuend[bit], subtrahend[bit]), bout_bit);

        bout_bit = or(and(not(minuend[bit]), subtrahend[bit]), 
        and(not(xor(minuend[bit], subtrahend[bit])), diff_bit));

        difference[bit] = diff_bit;
    }

    difference
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sub_32bit_computes_minuend_by_subtrahend_then_returns_difference() {
        let minuend = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
        0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0];
        let subtrahend = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
        0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0];
        
        let result = sub_32bit(minuend, subtrahend);
        let expected = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0];

        assert_eq!((result), (expected));
    }
}