// Takes a decimal number as argument and returns and array of bits with the
// value of the decimal argument.
fn decimal_to_binary(mut decimal_num: u8) -> [u8; 8] {
    let mut result = [0; 8];
    
    for i in (0..8).rev() {
        result[i] = decimal_num % 2;
        decimal_num /= 2;
    }

    result
}

// Takes a binary array starts a MSB, index 0, and returns its decimal value.
// Each bit is multiplied by 2^(7 - index) using a left shift, subtracting the index
// is used to decrease the shift as you go left.
fn binary_to_decimal(bits: [u8; 8]) -> u8 {
    let mut result = 0;

    for i in 0..8 {
        result += bits[i] << (7 - i);
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn goes_from_decimal_to_binary_returned_in_an_array() {
        let decimal = 34;
        let result = decimal_to_binary(decimal);
        let expected = [0, 0, 1, 0, 0, 0, 1, 0];

        assert_eq!((result), (expected));
    }

    #[test]
    fn goes_from_array_with_byte_to_returning_its_decimal() {
        let byte = [1, 0, 1, 1, 0, 1, 0, 1];
        let result = binary_to_decimal(byte);
        let expected = 181;

        assert_eq!((result), (expected));
    }
}