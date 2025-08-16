use crate::adders::subtraction_adders::subtraction_32_bit;

/*
Check if remainder >= divisor, to know what to write down in the quotient.

- If remainder >= divisor write down 1 (true is returned).
- If remainder < divisor write down 0 (false is returned).
*/
fn is_greater_or_equal(remainder: [u8; 32], devisor: [u8; 32]) -> bool {
    for i in 0..32 {
        if remainder[i] > devisor[i] {
            return true;
        } else if remainder[i] < devisor[i] {
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
pub fn division_32(devidend: [u8; 32], devisor: [u8; 32]) -> [u8; 32] {
    let mut quotient = [0u8; 32];
    let mut remainder = [0u8; 32];

    for i in 0..32 {
        shift_left(&mut remainder, devidend[i]);

        if is_greater_or_equal(remainder, devisor) {
            // Get new remainder through subtracting divisor from remainder,
            // at every new loop iteration.
            let new_remainder = subtraction_32_bit(remainder, devisor);

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
    fn divides_dividend_by_divisor_and_returns_quotient_and_remainder() {
        let devidend = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
        0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0];
        let devisor = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1];
        let result = division_32(devidend, devisor);
        
        let expected = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0];

        assert_eq!((result), (expected));
    }
}