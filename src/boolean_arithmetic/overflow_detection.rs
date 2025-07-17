use crate::operators::addition::{sum, carry_out};

// 4 Bit Adder returning a 4 bit array + overflow confirmation.
fn bit4addr(a: [u8; 4], b: [u8; 4]) -> ([u8; 4], u8) {
    let mut return4bit = [0; 4];
    let mut cout = 0;
    
    for i in (0..4).rev() {
        let sum = sum(a[i], b[i], cout);
        cout = carry_out(a[i], b[i], cout);
        return4bit[i] = sum;
    }

    (return4bit, cout)
}

#[cfg (test)]
mod test {
    use super::*;

    #[test]
    fn return_array_and_overflow_after_simulating_a_and_b_through_sum_and_carry_out() {
        let a = [1, 0, 1, 0];
        let b = [0, 0, 1, 0];
        let expected = ([1, 1, 0, 0], 0);

        assert_eq!(bit4addr(a, b), (expected));
    }
}