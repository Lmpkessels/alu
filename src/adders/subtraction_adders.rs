use crate::operators::subtraction::{difference_bit, borrow_out_bit};

// Subtraction 8 bit full adder starting at (LSB) moving to (MSB) while applying
// bit-by-bit difference and borrow out.
//
// Returns the result byte and underflow if detected.
fn subtraction_8_bit(byte_a: [u8; 8], byte_b: [u8; 8]) -> ([u8; 8], u8) {
    let mut result_byte = [0; 8];
    let mut borrow_out = 0;

    for bit in (0..8).rev() {
        let difference_bit = difference_bit(byte_a[bit], byte_b[bit],
             borrow_out);

        borrow_out = borrow_out_bit(byte_a[bit], byte_b[bit], 
            borrow_out);

        result_byte[bit] = difference_bit;
    }

    (result_byte, borrow_out)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn returns_8_bit_array_after_applying_difference_and_cout_logic() {
        let array_a = [0, 1, 1, 0, 0, 0, 1, 0];
        let array_b = [0, 0, 1, 0, 0, 1, 0, 0];
        let expected = ([0, 0, 1, 1, 1, 1, 1, 0], 0);

        assert_eq!(subtraction_8_bit(array_a, array_b), (expected));
    }
}