use crate::adders::addition_adders::addition_32_bit;

/*
Multiply 16 x 16, using add and shift left. 

Starting at least significant bit (LSB) rightmost moving to most significant 
bit (MSB) leftmost.
*/
pub fn multiply_16_x_16(word_a: [u8; 16], word_b: [u8; 16]) -> [u8; 32] {
    let mut result = [0u8; 32];
    
    for i in (0..16).rev() {
        // We continue if word_b[i] is 0 because word_a[j] & 0 = 0, 
        // so it's unnecessary to create an entire array for this. 
        if word_b[i] == 0 {
            continue;
        }

        let mut partial = [0u8; 32];
        for j in (0..16).rev() {
            if word_a[j] == 1 {
                // +1 is added to align the shifted word_a in the 32 bit array 
                // when combining i & j.
                partial[i + 1 + j] = 1;
            }
        } 
        // Result is reused for 16 times so it can add up the shifted partial 
        // and puts the right carries in place.
        result = addition_32_bit(result, partial);
    }
    result
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn multiplies_a_by_b() {
        let a = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0];
        let b = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0]; 
        let result = multiply_16_x_16(a, b);
        let expected = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
        0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0];

        assert_eq!((result), (expected));
    }

    #[test]
    fn multiplies_b_by_a() {
        let b = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0];
        let a = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0];
        let result = multiply_16_x_16(a, b);
        let expected = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
        0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0];

        assert_eq!((result), (expected));
    }

    #[test]
    fn multiplies_a_and_b_with_overlapping_bits_case() {
        let a = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1];
        let b = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1];
        let result = multiply_16_x_16(a, b);
        let expected = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
        0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1];

        assert_eq!((result), (expected));
    }
}