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
fn or8multi(inp: &[[u8; 16]; 8]) -> Vec<u8> { 
    let mut storage = Vec::with_capacity(16);

    // Loop through each bit index.
    for i in 0..16 {
        let mut result = 0;
        // Get each nested array.
        for a in 0..8 {
            // Apply OR logic bit-by-bit, index-by-index.
            result |= inp[a][i];
        }
        // Store bit in storage.
        storage.push(result);
    }

    // Return storage.
    storage
}

// AND 8 Multi: applies AND logic bit-by-bit on all index 0, then 1, etc. 
// Returns a vector of 16 bits.
fn and8multi(inp: &[[u8; 16]; 8]) -> Vec<u8> {
    let mut storage = Vec::with_capacity(16);

    // Loop through each bit index.
    for i in 0..16 {
        let mut result = 1;
        // Get each nested array.
        for a in 0..8 {
            // Apply AND logic bit-by-bit, index-by-index.
            result &= inp[a][i];
        }
        // Store bit in storage.
        storage.push(result);
    }

    // Return storage.
    storage
}

// XOR 8 Multi: applies XOR logic bit-by-bit on all index 0, then 1, etc.  
// Returns a vector of 16 bits.
fn xor8multi(inp: &[[u8; 16]; 8]) -> Vec<u8> {
    let mut storage = Vec::with_capacity(16);

    // Loop through each bit index.
    for i in 0..16 {
        let mut result = 0;
        // Get each nested array.
        for a in 0..8 {
            // Apply XOR logic bit-by-bit, index-by-index.
            result ^= inp[a][i];
        }
        storage.push(result);
    }

    // Return storage.
    storage
}

// NAND 8 Multi: applies NOT AND logic bit-by-bit on all index 0, then 1, etc. 
// Returns a vector of 16 bits.
fn nand8multi(inp: &[[u8; 16]; 8]) -> Vec<u8> {
    let mut storage = Vec::with_capacity(16);

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
        storage.push(result);
    }

    // Return storage.
    storage
}

// NOR 8 Multi: applies NOT OR logic bit-by-bit on all index 0, then 1, etc. 
// Returns a vector of 16 bits.
fn nor8multi(inp: &[[u8; 16]; 8]) -> Vec<u8> {
    let mut storage = Vec::with_capacity(16);

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
        storage.push(result);
    }
    
    storage
}


fn main() {
    // Check if everything works.    
    let test = [
        [0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1], 
        [0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 0], 
        [0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 1, 0, 1, 0, 1, 0],
        [0, 1, 1, 0, 1, 1, 1, 0, 0, 1, 1, 1, 1, 0, 1, 1],
        [0, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 0], 
        [0, 1, 1, 0, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0], 
        [0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1], 
        [0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 1, 0, 1, 0, 1, 0],
        ];

    // Give each functions parameter an argument.
    let test0 = or8multi(&test);
    let test1 = and8multi(&test);
    let test2 = xor8multi(&test);
    let test3 = nand8multi(&test);
    let test4 = nor8multi(&test);

    // Check if it's correct.
    println!(
        "OR: {test0:?}
        \nAND: {test1:?}
        \nXOR: {test2:?}
        \nNAND: {test3:?}
        \nNOR: {test4:?}"
    );
}