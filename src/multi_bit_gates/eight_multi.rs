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
fn or8multi(inp: [[u8; 16]; 8]) -> [u8; 16] { 
    let mut storage = [0; 16];

    // Loop through each bit index.
    for i in 0..16 {
        let mut result = 0;
        // Get each nested array.
        for a in 0..8 {
            // Apply OR logic bit-by-bit, index-by-index.
            result |= inp[a][i];
        }
        // Store bit in storage.
        storage[i] = result;
    }

    // Return storage.
    storage
}

// AND 8 Multi: applies AND logic bit-by-bit on all index 0, then 1, etc. 
// Returns a vector of 16 bits.
fn and8multi(inp: [[u8; 16]; 8]) -> [u8; 16] {
    let mut storage = [0; 16];

    // Loop through each bit index.
    for i in 0..16 {
        let mut result = 1;
        // Get each nested array.
        for a in 0..8 {
            // Apply AND logic bit-by-bit, index-by-index.
            result &= inp[a][i];
        }
        // Store bit in storage.
        storage[i] = result;
    }

    // Return storage.
    storage
}

// XOR 8 Multi: applies XOR logic bit-by-bit on all index 0, then 1, etc.  
// Returns a vector of 16 bits.
fn xor8multi(inp: [[u8; 16]; 8]) -> [u8; 16] {
    let mut storage = [0; 16];

    // Loop through each bit index.
    for i in 0..16 {
        let mut result = 0;
        // Get each nested array.
        for a in 0..8 {
            // Apply XOR logic bit-by-bit, index-by-index.
            result ^= inp[a][i];
        }
        storage[i] = result;
    }

    // Return storage.
    storage
}

// NAND 8 Multi: applies NOT AND logic bit-by-bit on all index 0, then 1, etc. 
// Returns a vector of 16 bits.
fn nand8multi(inp: [[u8; 16]; 8]) -> [u8; 16] {
    let mut storage = [0; 16];

    // Loop through each bit index.
    for i in 0..16 {
        let mut result = 1;
        // Get each nested array.
        for a in 0..8 {
            // Apply AND logic bit-by-bit, index-by-index.
            result &= inp[a][i];
        }
        // Apply NOT to the result and mask to keep only the last bit.
        result = !result & 1;
        storage[i] = result;
    }

    // Return storage.
    storage
}

// NOR 8 Multi: applies NOT OR logic bit-by-bit on all index 0, then 1, etc. 
// Returns a vector of 16 bits.
fn nor8multi(inp: [[u8; 16]; 8]) -> [u8; 16] {
    let mut storage = [0; 16];

    // Loop through each bit index.
    for i in 0..16 {
        let mut result = 0;
        // Get each nested array.
        for a in 0..8 {
            // Apply OR logic bit-by-bit, index-by-index.
            result |= inp[a][i];
        }
        // Apply NOT to the result and mask to keep only the last bit.
        result = !result & 1;
        storage[i] = result;
    }
    
    storage
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