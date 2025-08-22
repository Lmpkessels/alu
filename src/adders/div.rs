use crate::adders::sub::sub_32bit_nu;

/// 32-bit division.
/// 
/// Implements restoring division (shift-subtract):
/// - Shift remainder left and append the next dividend bit (LSB at index 31).
/// - If remainder >= divisor, subtract divisor and set quotient[i] = 1.
/// - Otherwise, quotient[i] = 0.
/// 
/// Returns: quotient word (32 bits).
pub fn div_32bit(dividend: [u8; 32], divisor: [u8; 32]) -> ([u8; 32], u8) {
    if divisor == [0u8; 32] {
        return ([0u8; 32], 1);
    }
    
    let mut quotient = [0u8; 32];
    let mut remainder = [0u8; 32];

    for i in 0..32 {
        shift_left(&mut remainder, dividend[i]);

        if greater_or_equal(remainder, divisor) {
            // Update remainder for this step
            remainder = sub_32bit_nu(remainder, divisor);
            quotient[i] = 1;
        } else {
            quotient[i] = 0;
        }
    }

    (quotient, 0)
}

// Shift remainder left (MSB to index 0), append next dividend bit at index 31.
fn shift_left(remainder: &mut [u8; 32], next_bit: u8) {
    for i in 0..31 {
        remainder[i] = remainder[i + 1];
    }
    remainder[31] = next_bit;
}

// Return true if remainder >= divisor, else false.
fn greater_or_equal(remainder: [u8; 32], divisor: [u8; 32]) -> bool {
    for i in 0..32 {
        if remainder[i] > divisor[i] {
            return true;
        } else if remainder[i] < divisor[i] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn div_32bit_computes_dividend_by_divisor_then_returns_quotient() {
        let dividend = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
        0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0];
        let divisor = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1];

        let result = div_32bit(dividend, divisor);
        let expected = ([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0], 0);

        assert_eq!((result), (expected));
    }

    #[test]
    fn div_32bit_computes_overflow_since_divisor_is_0() {
        let dividend = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
        0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0];
        let divisor = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

        let result = div_32bit(dividend, divisor);
        let expected = ([0u8; 32], 1);

        assert_eq!((result), (expected));
    }
}