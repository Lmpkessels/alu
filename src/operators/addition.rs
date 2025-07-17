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

#[cfg(test)]
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
}