use crate::operators::subtraction::{difference_bit, borrow_out_bit};

pub fn subtraction_1_bit_diff(a: u8, b: u8, borrow_out: u8) -> u8 {
    difference_bit(a, b, borrow_out)
}

pub fn subtraction_1_bit(a: u8, b: u8, borrow_out: u8) -> (u8, u8) {
    (difference_bit(a, b, borrow_out), borrow_out_bit(a, b, borrow_out))
}

fn subtraction_4_bit(byte_a: [u8; 4], byte_b: [u8; 4]) -> ([u8; 4], u8) {
    let mut result_for_bit = [0; 4];
    let mut borrow_out = 0;

    for bit in (0..4).rev() {
        let difference_bit = difference_bit(byte_a[bit], byte_b[bit], borrow_out);

        borrow_out = borrow_out_bit(byte_a[bit], byte_b[bit], borrow_out);

        // Store the result bit in output at position i.
        result_for_bit[bit] = difference_bit;
    }

    (result_for_bit, borrow_out)
}

fn subtraction_8_bit(byte_a: [u8; 8], byte_b: [u8; 8]) -> ([u8; 8], u8) {
    let mut result_byte = [0; 8];
    let mut borrow_out = 0;

    for bit in (0..8).rev() {
        let difference_bit = difference_bit(byte_a[bit], byte_b[bit], borrow_out);

        borrow_out = borrow_out_bit(byte_a[bit], byte_b[bit], borrow_out);

        // Store the result bit in output at position i.
        result_byte[bit] = difference_bit;
    }

    (result_byte, borrow_out)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn returns_4_bit_array_after_applying_difference_and_cout_logic() {
        let array_a = [1, 0, 0, 1];
        let array_b = [0, 0, 1, 0];
        let expected = ([0, 1, 1, 1], 0);

        assert_eq!(subtraction_4_bit(array_a, array_b), expected);
    }

    #[test]
    fn returns_8_bit_array_after_applying_difference_and_cout_logic() {
        let array_a = [0, 1, 1, 0, 0, 0, 1, 0];
        let array_b = [0, 0, 1, 0, 0, 1, 0, 0];
        let expected = ([0, 0, 1, 1, 1, 1, 1, 0], 0);

        assert_eq!(subtraction_8_bit(array_a, array_b), (expected));
    }
}