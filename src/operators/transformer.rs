/// Converts a decimal integer into a 32-bit binary word.
///
/// - Input: an unsigned integer (`u32`)
/// - Output: an array of 32 bits (`[u8; 32]`)
///
/// The most significant bit (MSB) is stored at index 0,
/// and the least significant bit (LSB) is at index 31.
/// Each array element is either `0` or `1`.
pub fn int_to_32bit(mut integer: u32) -> [u8; 32] {
    let mut word = [0; 32];
    for i in (0..32).rev() {
        word[i] = (integer % 2) as u8;
        integer /= 2;
    }
    word
}

/// Converts a decimal integer into a 16-bit binary word.
///
/// - Input: an unsigned integer (`u32`)
/// - Output: an array of 16 bits (`[u8; 16]`)
///
/// The most significant bit (MSB) is stored at index 0,
/// and the least significant bit (LSB) is at index 15.
/// Each array element is either `0` or `1`.
pub fn int_to_16bit(mut integer: u32) -> [u8; 16] {
    let mut word = [0; 16];
    for i in (0..16).rev() {
        word[i] = (integer % 2) as u8;
        integer /= 2;
    }
    word
}

/// Converts a 32-bit binary word back into its decimal integer value.
///
/// - Input: a binary word (`[u8; 32]`)
/// - Output: a decimal integer (`u32`)
///
/// The array is interpreted with MSB at index 0 and LSB at index 31.
/// Each bit contributes to the value as:
/// `word[i] * 2^(31 - i)`
pub fn bit32_to_int(word: [u8; 32]) -> u32 {
    let mut integer = 0;
    for i in 0..32 {
        integer += (word[i] as u32) << (31 - i);
    }
    integer
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
        let result = bit32_to_int(word);
        assert_eq!(result, 181);
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
        let word = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
        0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0];
        let result = bit32_to_int(word);
        let expected = 212;

        assert_eq!((result), (expected));
    }
}