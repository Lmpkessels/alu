use crate::operators::subtraction::{diff, borrow_out};

fn subtraction_4_bit(a: [u8; 4], b: [u8; 4]) -> ([u8; 4], u8) {
    let mut output = [0; 4];
    let mut bout = 0;

    for i in (0..4).rev() {
        let diff = diff(a[i], b[i], bout);
        bout = borrow_out(a[i], b[i], bout);

        // Store the result bit in output at position i.
        output[i] = diff;
    }

    (output, bout)
}

fn subtraction_8_bit(a: [u8; 8], b: [u8; 8]) -> ([u8; 8], u8) {
    let mut output = [0; 8];
    let mut bout = 0;

    for i in (0..8).rev() {
        let diff = diff(a[i], b[i], bout);
        bout = borrow_out(a[i], b[i], bout);

        // Store the result bit in output at position i.
        output[i] = diff;
    }

    (output, bout)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn returns_bit_after_applying_nota_and_b_or_not_a_xor_b_and_bin() {
        assert_eq!(diff(1, 0, 1), (0));
    }

    #[test]
    fn returns_bit_after_applying_a_xor_b_xor_bout() {
        assert_eq!(borrow_out(1, 1, 0), (0));
    }

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