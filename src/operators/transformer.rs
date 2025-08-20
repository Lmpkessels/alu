// Takes a decimal number as argument and returns and array of bits with the
// value of the decimal argument.
pub fn int_to_32bit(mut integer: u32) -> [u8; 32] {
    let mut word = [0; 32];
    
    for i in (0..32).rev() {
        word[i] = integer as u8 % 2;
        integer /= 2;
    }

    word
}

pub fn int_to_16bit(mut integer: u32) -> [u8; 16] {
    let mut word = [0; 16];

    for i in (0..16).rev() {
        word[i] = integer as u8 % 2;
        integer /= 2;
    }

    word
}

// Takes a binary array starts a MSB, index 0, and returns its decimal value.
// Each bit is multiplied by 2^(31 - index) using a left shift, subtracting the index
// is used to decrease the shift as you go left.
pub fn t2bit_to_int(word: [u8; 32]) -> u32 {
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
    fn int_to_16bit_takes_integer_then_returns_16_bit_word() {
        let t6bit_word = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
        0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 0, 1];
        let result = t2bit_to_int(t6bit_word);
        let expected = 181;

        assert_eq!((result), (expected));
    }

    #[test]
    fn t2bit_to_int_takes_32_bit_word_then_returns_integer() {
        let t2bit_word = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
        0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0];
        let result = t2bit_to_int(t2bit_word);
        let expected = 212;

        assert_eq!((result), (expected));
    }
}