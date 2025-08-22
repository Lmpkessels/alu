use crate::gates::{not, and, or, xor};

/// 32-bit subtraction.
/// 
/// Ripple-borrow subtractor logic (LSB → MSB):
/// - Difference: a[bit] XOR b[bit] XOR borrow_in
/// - Borrow: (NOT(a[bit]) AND b[bit]) OR ((NOT(a[bit] XOR b[bit])) AND diff_bit)
/// 
/// Returns: 32-bit difference word with overflow.
pub fn sub_32bit_u(minuend: [u8; 32], subtrahend: [u8; 32]) -> ([u8; 32], u8) {
    let mut difference = [0; 32];
    let mut bout_bit = 0;

    // Start from least significant bit (rightmost = index 31)
    for bit in (0..32).rev() {
        let diff_bit = xor(xor(minuend[bit], subtrahend[bit]), bout_bit);

        bout_bit = or(
            and(not(minuend[bit]), subtrahend[bit]),
            and(not(xor(minuend[bit], subtrahend[bit])), diff_bit),
        );

        difference[bit] = diff_bit;
    }

    // bout_bit indicates (Underflow)
    (difference, bout_bit)
}

/// 32-bit subtraction without overflow.
///
/// No overflow is returned so that divisor function can work with isolated
/// arrays.
pub fn sub_32bit_nu(minuend: [u8; 32], subtrahend: [u8; 32]) -> [u8; 32] {
    let mut difference = [0; 32];
    let mut bout_bit = 0;

    // Start from least significant bit (rightmost = index 31)
    for bit in (0..32).rev() {
        let diff_bit = xor(xor(minuend[bit], subtrahend[bit]), bout_bit);

        bout_bit = or(
            and(not(minuend[bit]), subtrahend[bit]),
            and(not(xor(minuend[bit], subtrahend[bit])), diff_bit),
        );

        difference[bit] = diff_bit;
    }

    difference
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sub_32bit_computes_a_minus_b_and_returns_difference() {
        let minuend = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
        0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0];
        let subtrahend = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
        0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0];
        
        let result = sub_32bit_u(minuend, subtrahend);
        let expected = ([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0], 0);

        assert_eq!(result, expected);
    }
}
