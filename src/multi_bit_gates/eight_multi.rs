/* 
OR, AND, XOR, NAND, and NOR 8Multi functions take in 1 nested borrowed array 
containing 8 arrays of 16 bits.

Each bit is looped out from the nested array, then logic is applied 
bit-by-bit, index-by-index, returning one value per index (i.e., per column).

Each value returned after the logic operation is pushed into a vector
and returned by the function.
*/

// OR 8 Multi: applies OR logic bit-by-bit on all index 0, then 1, etc. 
// Returns a vector of 16 bits.
fn or8multi(input_array: [[u8; 16]; 8]) -> [u8; 16] { 
    let mut word = [0; 16];

    // Loop through each bit index.
    for bit in 0..16 {
        let mut result_bit = 0;
        // Get each nested array.
        for index_array in 0..8 {
            // Apply OR logic bit-by-bit, index-by-index.
            result_bit |= input_array[index_array][bit];
        }
        // Store bit in storage.
        word[bit] = result_bit;
    }

    // Return storage.
    word
}

// AND 8 Multi: applies AND logic bit-by-bit on all index 0, then 1, etc. 
// Returns a vector of 16 bits.
fn and8multi(input_array: [[u8; 16]; 8]) -> [u8; 16] {
    let mut word = [0; 16];

    // Loop through each bit index.
    for bit in 0..16 {
        let mut result_bit = 1;
        // Get each nested array.
        for index_array in 0..8 {
            // Apply AND logic bit-by-bit, index-by-index.
            result_bit &= input_array[index_array][bit];
        }
        // Store bit in storage.
        word[bit] = result_bit;
    }

    // Return storage.
    word
}

// XOR 8 Multi: applies XOR logic bit-by-bit on all index 0, then 1, etc.  
// Returns a vector of 16 bits.
fn xor8multi(input_array: [[u8; 16]; 8]) -> [u8; 16] {
    let mut word = [0; 16];

    // Loop through each bit index.
    for bit in 0..16 {
        let mut result_bit = 0;
        // Get each nested array.
        for index_array in 0..8 {
            // Apply XOR logic bit-by-bit, index-by-index.
            result_bit ^= input_array[index_array][bit];
        }
        word[bit] = result_bit;
    }

    // Return storage.
    word
}

// NAND 8 Multi: applies NOT AND logic bit-by-bit on all index 0, then 1, etc. 
// Returns a vector of 16 bits.
fn nand8multi(input_array: [[u8; 16]; 8]) -> [u8; 16] {
    let mut word = [0; 16];

    // Loop through each bit index.
    for bit in 0..16 {
        let mut result_bit = 1;
        // Get each nested array.
        for index_array in 0..8 {
            // Apply AND logic bit-by-bit, index-by-index.
            result_bit &= input_array[index_array][bit];
        }
        // Apply NOT to the result and mask to keep only the last bit.
        result_bit = !result_bit & 1;
        word[bit] = result_bit;
    }

    // Return storage.
    word
}

// NOR 8 Multi: applies NOT OR logic bit-by-bit on all index 0, then 1, etc. 
// Returns a vector of 16 bits.
fn nor8multi(input_array: [[u8; 16]; 8]) -> [u8; 16] {
    let mut word = [0; 16];

    // Loop through each bit index.
    for bit in 0..16 {
        let mut result_bit = 0;
        // Get each nested array.
        for index_array in 0..8 {
            // Apply OR logic bit-by-bit, index-by-index.
            result_bit |= input_array[index_array][bit];
        }
        // Apply NOT to the result and mask to keep only the last bit.
        result_bit = !result_bit & 1;
        word[bit] = result_bit;
    }
    
    word
}

#[cfg(test)]
mod test {
    use super::*;

    fn array_test_value() -> [[u8; 16]; 8] {
        let array = [
            [0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1], 
            [0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 0], 
            [0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 1, 0, 1, 0, 1, 0],
            [0, 1, 1, 0, 1, 1, 1, 0, 0, 1, 1, 1, 1, 0, 1, 1],
            [0, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 0], 
            [0, 1, 1, 0, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0], 
            [0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1], 
            [0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 1, 0, 1, 0, 1, 0],
        ];

        array
    }

    #[test]
    // Test or8multi.
    fn return_1_array_after_applying_or_logic_bit_by_bit_per_index_position() {
        let input = array_test_value();
        let expected = [0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1];

        assert_eq!(or8multi(input), (expected));
    }

    #[test]
    // Test and8multi.
    fn return_1_array_after_applying_and_logic_bit_by_bit_per_inder_position() {
        let input = array_test_value();
        let expected = [0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0];

        assert_eq!(and8multi(input), (expected));
    }

    #[test]
    // Test xor8multi.
    fn return_1_array_after_applying_xor_logic_bit_by_bit_per_inder_position() {
        let input = array_test_value();
        let expected = [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1];

        assert_eq!(xor8multi(input), (expected));
    }

    #[test]
    // Test nand8multi.
    fn return_1_array_after_applying_nand_logic_bit_by_bit_per_inder_position() {
        let input = array_test_value();
        let expected = [1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1];

        assert_eq!(nand8multi(input), (expected));
    }

    #[test]
    // Test nor8multi.
    fn return_1_array_after_applying_nor_logic_bit_by_bit_per_inder_position() {
        let input = array_test_value();
        let expected = [1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0];

        assert_eq!(nor8multi(input), (expected));
    }
}