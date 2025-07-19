use crate::adders::addition_adders::{one_bit_full_adder_sum};
use crate::adders::subtraction_adders::{subtraction_1_bit_diff};
use crate::gates::basic::{and, or, xor, not};

fn alu(
    control_pattern: [u8; 6], 
    bit_a: u8, 
    bit_b: u8, 
    carry_or_borrow_in: u8, 
) -> u8 {

    match control_pattern {
        [0, 0, 0, 0, 0, 1] => one_bit_full_adder_sum(bit_a, bit_b, carry_or_borrow_in),
        [0, 0, 0, 0, 1, 0] => subtraction_1_bit_diff(bit_a, bit_b, carry_or_borrow_in),
        [0, 0, 0, 0, 1, 1] => and(bit_a, bit_b),
        [0, 0, 0, 1, 0, 0] => or(bit_a, bit_b),
        [0, 0, 0, 1, 0, 1] => xor(bit_a, bit_b),
        [0, 0, 0, 1, 1, 0] => not(bit_a),
        [0, 0, 0, 1, 1, 1] => not(bit_b),
        _ => panic!("ALU Does not have this feature"),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn matches_one_bit_full_adder_and_returns_1() {
        let control_pattern = [0, 0, 0, 0, 0, 1];
        let bit_a = 0;
        let bit_b = 1;
        let carry = 0;
        
        let result = alu(control_pattern, bit_a, bit_b, carry);
        let expected = 1;

        assert_eq!((result), (expected));
    }
}