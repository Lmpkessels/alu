/// Converts a decimal integer into a 32-bit binary word.
/// 
/// Process:
/// - Use a bitmask (`& 1`) to extract the least significant bit (LSB).
/// - Shift the integer right (`>>= 1`) to divide by 2 and expose the next bit.
/// - Store the extracted bit into the array at the correct position.
/// 
/// Notes:
/// - The most significant bit (MSB) ends up at index 0.
/// - The least significant bit (LSB) ends up at index 31.
pub fn int_to_32bit(mut integer: u32) -> [u8; 32] {
    let mut word = [0; 32];
    for i in (0..32).rev() {
        word[i] = (integer & 1) as u8;
        integer >>= 1;
    }
    word
}

/// Converts a decimal integer into a 16-bit binary word.
/// 
/// Process:
/// - Use a bitmask (`& 1`) to extract the least significant bit (LSB).
/// - Shift the integer right (`>>= 1`) to divide by 2 and expose the next bit.
/// - Store the extracted bit into the array at the correct position.
/// 
/// Notes:
/// - The most significant bit (MSB) ends up at index 0.
/// - The least significant bit (LSB) ends up at index 15.
pub fn int_to_16bit(mut integer: u32) -> [u8; 16] {
    let mut word = [0; 16];
    for i in (0..16).rev() {
        word[i] = (integer & 1) as u8;
        integer >>= 1;
    }
    word
}

/// Converts a 32-bit binary word back into its decimal integer value.
/// 
/// Process:
/// - Loop over each bit in the array.
/// - Shift the bit left to its proper power-of-two position (`<< (31 - i)`).
/// - Accumulate it into the result integer with bitwise OR (`|=`).
/// 
/// Notes:
/// - The input flag is ignored; only the bits are used.
/// - The output flag is always 0 since conversion cannot overflow.
pub fn bit32_to_int(word: ([u8; 32], u8)) -> (u32, u8) {
    let mut integer: u32 = 0;
    let (bits, _) = word; // ignore flag
    
    for i in 0..32 {
        integer |= (bits[i] as u32) << (31 - i);
    }

    (integer, 0) // flag always 0
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn int_to_32bit_takes_integer_then_returns_32_bit_word() {
        let integer = 34;
        let result = int_to_32bit(integer);
        let expected = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0];

        assert_eq!((result), (expected));
    }

    #[test]
    fn bit32_to_int_returns_correct_integer() {
        let word = int_to_32bit(181);
        let result = bit32_to_int((word, 0));
        assert_eq!(result, (181, 0));
    }

    #[test]
    fn int_to_16bit_takes_integer_then_returns_16_bit_word() {
        let integer = 181;
        let result = int_to_16bit(integer);
        let expected = [0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 0, 1];

        assert_eq!((result), (expected));
    }

    #[test]
    fn bit32_to_int_takes_32_bit_word_then_returns_integer() {
        let word = ([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
        0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0], 0);
        let result = bit32_to_int(word);
        let expected = (212, 0);

        assert_eq!((result), (expected));
    }
}