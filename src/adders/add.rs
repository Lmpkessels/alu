use crate::gates::{or, and, xor};

// eight bit full adder, receiving as argument two arrays.
// Fn returns a byte + overflow.
pub fn add_32bit(left_operand: [u8; 32], right_operand: [u8; 32]) -> [u8; 32] {
    let mut sum = [0u8; 32];
    let mut cout_bit = 0;

    for bit in (0..32).rev() {
        let sum_bit = xor(xor(left_operand[bit], right_operand[bit]), 
            cout_bit);

        let cin_bit = or(and(left_operand[bit], right_operand[bit]), 
            and(cout_bit, xor(left_operand[bit], right_operand[bit])));
            
        sum[bit] = sum_bit;
        cout_bit = cin_bit;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_32bit_computes_left_operand_by_right_opperand_then_returns_sum() {
        let bit_a = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
        0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0];
        let bit_b = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
        0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0];
        let expected = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
            0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0];
        
        
        assert_eq!(add_32bit(bit_a, bit_b), (expected));
    }
}
