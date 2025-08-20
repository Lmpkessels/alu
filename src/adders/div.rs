use crate::adders::sub::sub_32bit;

/*
Check if remainder >= divisor, to know what to write down in the quotient.

- If remainder >= divisor write down 1 (true is returned).
- If remainder < divisor write down 0 (false is returned).
*/
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

/*
Shift remainder left to make space at the LSB, then append the next
dividend bit at index 31 (the LSB in our [MSB..LSB] layout).
*/
fn shift_left(remainder: &mut [u8; 32], next_bit: u8) {
    for i in 0..31 {
        remainder[i] = remainder[i + 1];
    }

    remainder[31] = next_bit;
} 

/*
NOTE: Returns only the quotient, without remainder. Fits the ALU return value.

Divide; divisor by the dividend to get quotient and remainder.

Shift remainder to left and append the least significant bit (LSB) dividend
at index 31.

- If remainder >= divisor, quotient is 1.
- If remainder < divisor, quotient is 0.
*/
pub fn div_32bit(dividend: [u8; 32], divisor: [u8; 32]) -> [u8; 32] {
    let mut quotient = [0u8; 32];
    let mut remainder = [0u8; 32];

    for i in 0..32 {
        shift_left(&mut remainder, dividend[i]);

        if greater_or_equal(remainder, divisor) {
            // Get new remainder through subtracting divisor from remainder,
            // at every new loop iteration.
            let new_remainder = sub_32bit(remainder, divisor);

            // Store the new remainder to end up with the final remainder.
            remainder = new_remainder;

            quotient[i] = 1;
        } else {
            quotient[i] = 0;
        }
    }

    quotient
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
        
        let expected = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0];

        assert_eq!((result), (expected));
    }
}