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
}