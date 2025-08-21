use crate::adders::add::add_32bit;

/// 16×16-bit multiplication.
/// 
/// Uses the shift-and-add method:
/// - For each set bit in the multiplier, shift the multiplicant and add to product.
/// - Shifts proceed from LSB to MSB.
/// 
/// Returns: 32-bit product.
pub fn multiply_16x16bit(multiplicant: [u8; 16], multiplier: [u8; 16]) -> [u8; 32] {
    let mut product = [0u8; 32];
    
    for i in (0..16).rev() {
        // Skip when multiplier[i] == 0 (no partial product)
        if multiplier[i] == 0 {
            continue;
        }

        let mut partial = [0u8; 32];
        for j in (0..16).rev() {
            if multiplicant[j] == 1 {
                // Shift multiplicant bit into 32-bit space (align with i+j position)
                partial[i + 1 + j] = 1;
            }
        } 

        // Add shifted partial into product (carry propagates correctly)
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