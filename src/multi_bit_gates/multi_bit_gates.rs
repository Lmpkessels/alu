// Multi 16 bit NOT gate flips each 'bit' in the array.
// ex. logic: a = 1, (NOT(a)) = 0.
pub fn not16(inp: [u8; 16]) -> [u8; 16] {
    // Default set values.
    let mut output = [0; 16];
    let mut bit = 0;

    for bit in 0..16 {
        // Use NOT logic on bit values in array.
        output[bit] = (!inp[bit]) & 1;
    } 

    // Return array of 16 boolean values.
    output
}

// Multi-16-bit AND gate, receives 2 arrays from 'stack'.
// Makes both arrays go bit-by-bit through a AND logic gate.
// Returns one array after using AND gate.
// ex. logic: a = 1, b = 0, (a AND b) = 0.
pub fn and16(inp_a: [u8; 16], inp_b: [u8; 16]) -> [u8; 16] {
    // Default set values.
    let mut output = [0; 16];
    let mut bit = 0;

    for bit in 0..16 {
        // Use AND logic on bit-by-bit throug arrays.
        output[bit] = (inp_a[bit] & inp_b[bit]) & 1;
        
    }

    // Return array of 16 boolean values.
    output
}

// Multi-16-bit OR gate, receives 2 arrays from 'stack'.
// Makes both arrays go bit-by-bit through a OR logic gate.
// Returns one array after using OR gate.
// ex. logic: a = 1, b = 0, (a OR b) = 1.
pub fn or16(inp_a: [u8; 16], inp_b: [u8; 16]) -> [u8; 16] {
    // Default set values.
    let mut output = [0; 16];
    let mut bit = 0;

    for bit in 0..16 {
        // Use OR logic bit-by-bit through arrays.
        output[bit] = (inp_a[bit] | inp_b[bit]) & 1;        
    }

    // Return array of 16 boolean values.
    output
}


// Multi 16 bit mux gate, receives 2 arrays from 'stack'.
// Checks if selector 'sel' is 'true' if so returns array 'b' else 'a'.
// ex. logic: inp_a = 1, inp_b = 0, sel = 0 (Output = 1)
pub fn mux16(inp_a: [u8; 16], inp_b: [u8; 16], sel: bool) -> [u8; 16] {
    // Default set values.
    let mut output = [0; 16];
    let mut bit = 0;

    for bit in 0..16 {
        // If sel is true.
        output[bit] = if sel  {
            // Output is input b.
            inp_b[bit]
        } else {
            // Output is input a.
            inp_a[bit]
        };
    }

    // Return array of 16 boolean values.
    output
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn loops_through_input_takes_index_bit_by_bit_applies_not() {
        let input = [0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1];
        let expected = [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0];
        assert_eq!(not16(input), (expected));
    }

    #[test]
    fn loops_through_a_and_b_takes_index_bit_by_bit_applies_and() {
        let inp_a = [1, 0, 1, 1, 0, 1, 1, 1, 1, 0, 0, 1, 0, 0, 1, 1];
        let inp_b = [1, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 1];
        let expected = [1, 0, 1, 1, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 1];

        assert_eq!(and16(inp_a, inp_b), (expected));
    }

    #[test]
    fn loops_through_a_and_b_takes_index_bit_by_bit_applies_or() {
        let inp_a = [1, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 1, 1, 0, 1];
        let inp_b = [1, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 1, 1, 0, 1];
        let expected = [1, 0, 0, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1];

        assert_eq!(or16(inp_a, inp_b), (expected));
    }

    #[test]
    fn if_sel_is_false_a_else_if_true_b() {
        let inp_a = [1, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 1, 1, 0, 1];
        let inp_b = [0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1];
        let sel_false = false;
        let sel_true = true;

        assert_eq!(mux16(inp_a, inp_b, sel_false), 
        ([1, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 1, 1, 0, 1]));

        assert_eq!(mux16(inp_a, inp_b, sel_true), 
        ([0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1]));
    }
}