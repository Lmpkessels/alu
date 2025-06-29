// Multi 16 bit NOT gate flips each 'bit' in the array.
// ex. logic: a = 1, (NOT(a)) = 0.
pub fn not16(inp: [bool; 16]) -> [bool; 16] {
    // Default set values.
    let mut output = [false; 16];
    let mut bit = 0;

    while bit < 16 {
        // Use NOT logic on bit values in array.
        output[bit] = !inp[bit];
        // Increment bit by 1 to move to the next index. Loop ends if bit == 16.
        bit += 1;
    } 

    // Return array of 16 boolean values.
    output
}

// Multi-16-bit AND gate, receives 2 arrays from 'stack'.
// Makes both arrays go bit-by-bit through a AND logic gate.
// Returns one array after using AND gate.
// ex. logic: a = 1, b = 0, (a AND b) = 0.
pub fn and16(inp_a: [bool; 16], inp_b: [bool; 16]) -> [bool; 16] {
    // Default set values.
    let mut output = [false; 16];
    let mut bit = 0;

    while bit < 16 {
        // Use AND logic on bit-by-bit throug arrays.
        output[bit] = inp_a[bit] & inp_b[bit];
        // Increment bit by 1 to move to the next index. Loop ends if bit == 16.
        bit += 1;
    }

    // Return array of 16 boolean values.
    output
}

// Multi-16-bit OR gate, receives 2 arrays from 'stack'.
// Makes both arrays go bit-by-bit through a OR logic gate.
// Returns one array after using OR gate.
// ex. logic: a = 1, b = 0, (a OR b) = 1.
pub fn or16(inp_a: [bool; 16], inp_b: [bool; 16]) -> [bool; 16] {
    // Default set values.
    let mut output = [false; 16];
    let mut bit = 0;

    while bit < 16 {
        // Use OR logic bit-by-bit through arrays.
        output[bit] = inp_a[bit] || inp_b[bit];
        // Increment bit by 1 to move to the next index. Loop ends if bit == 16.
        bit += 1;
    }

    // Return array of 16 boolean values.
    output
}


// Multi 16 bit mux gate, receives 2 arrays from 'stack'.
// Checks if selector 'sel' is 'true' if so returns array 'b' else 'a'.
// ex. logic: inp_a = 1, inp_b = 0, sel = 0 (Output = 1)
pub fn mux16(inp_a: [bool; 16], inp_b: [bool; 16], sel: bool) 
-> [bool; 16] {
    // Default set values.
    let mut output = [false; 16];
    let mut bit = 0;

    while bit < 16 {
        // If sel is true.
        output[bit] = if sel {
            // Output is input b.
            inp_b[bit]
        } else {
            // Output is input a.
            inp_a[bit]
        };
        // Increment bit by 1 to move to the next index. Loop ends if bit == 16.
        bit += 1;
    }

    // Return array of 16 boolean values.
    output
}