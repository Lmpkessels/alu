use crate::adders::add::add_32bit;

/*
Multiply 16 x 16, using add and shift left. 

Starting at least significant bit (LSB) rightmost moving to most significant 
bit (MSB) leftmost.
*/
pub fn multiply_16x16bit(multiplicant: [u8; 16], multiplier: [u8; 16]) -> [u8; 32] {
    let mut product = [0u8; 32];
    
    for i in (0..16).rev() {
        // Continue if word_b[i] is 0 because word_a[j] & 0 = 0, 
        // so it's unnecessary to create an entire array for this. 
        if multiplier[i] == 0 {
            continue;
        }

        let mut partial = [0u8; 32];
        for j in (0..16).rev() {
            if multiplicant[j] == 1 {
                // +1 is added to align the shifted word_a in the 32 bit array 
                // when combining i & j.
                partial[i + 1 + j] = 1;
            }
        } 
        // Result is reused for 16 times so it can add up the shifted partial 
        // and puts the right carries in place.
        product = add_32bit(product, partial);
    }
    product
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn multiply_16x16bit_computes_a_x_b_then_returns_product() {
        let a = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0];
        let b = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0]; 
        let result = multiply_16x16bit(a, b);
        let expected = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
        0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0];

        assert_eq!((result), (expected));
    }

    #[test]
    fn multiply_16x16bit_computes_b_x_a_then_returns_product() {
        let b = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0];
        let a = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0];
        let result = multiply_16x16bit(a, b);
        let expected = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
        0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0];

        assert_eq!((result), (expected));
    }

    #[test]
    fn multiply_16x16bit_computes_overlapping_bits_then_returns_product() {
        let a = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1];
        let b = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1];
        let result = multiply_16x16bit(a, b);
        let expected = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
        0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1];

        assert_eq!((result), (expected));
    }
}