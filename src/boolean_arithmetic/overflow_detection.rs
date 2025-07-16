use crate::gates::basic::{xor, or, and};

// Sum logic, (a XOR b) XOR Carry in, return bit (0 or 1) after logic.
pub fn sum(a: u8, b: u8, cin: u8) -> u8{
    let sum = xor(xor(a, b), cin);
    
    sum
}

// Carry out logic, (a AND b) OR (Carry in AND (a XOR b)), 
// returns bit (0 or 1) after logic.
pub fn carry_out(a: u8, b: u8, cin: u8) -> u8 {
    let cout = or(and(a, b), and(cin, xor(a, b)));
    
    cout
}

// 4 Bit Adder returning a 4 bit array + overflow confirmation.
pub fn bit4addr(a: [u8; 4], b: [u8; 4]) -> ([u8; 4], u8) {
    let mut return4bit = [0; 4];
    let mut carry = 0;
    
    for i in (0..4).rev() {
        let sum = sum(a[i], b[i], carry);
        carry = carry_out(a[i], b[i], carry);
        return4bit[i] = sum;
    }

    // Confirm if theres overflow at the (MSB).
    if carry != 0 {
        println!("Overflow detected");
    } else {
        println!("No overflow")
    }

    (return4bit, carry)
}

#[cfg (test)]
mod test {
    use super::*;

    #[test]
    fn a_xor_b_is_output_then_output_xor_carry_in_is_sum() {
        let a = 1;
        let b = 0;
        let cin = 0;
        let expected = 1;

        assert_eq!(sum(a, b, cin), (expected));
    }

    #[test]
    fn a_and_b_is_outp_a_xor_b_and_cin_is_oupt1_then_outp_or_outp1_is_return_value() {
        let a = 0;
        let b = 1;
        let cin = 1;
        let expected = 1;

        assert_eq!(carry_out(a, b, cin), (expected));
    }

    #[test]
    fn return_array_and_overflow_after_simulating_a_and_b_through_sum_and_carry_out() {
        let a = [1, 0, 1, 0];
        let b = [0, 0, 1, 0];
        let expected = ([1, 1, 0, 0], 0);

        assert_eq!(bit4addr(a, b), (expected));
    }
}