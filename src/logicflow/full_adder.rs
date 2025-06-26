mod gates;
use gates::*;

// Full adder returning Sum and Carry out in boolean data-type.
fn full_addr(a: &bool, b: &bool, cin: &bool) -> (bool, bool) {
    // Sum logic -> a AND b = bit, then Bit AND Cin, is end Bit.
    let sum = xor(&xor(a, b), cin);
    // Cout logic -> (a AND b = Bit), OR (a XOR b = Bit, AND Cin = Bit) = Bit.
    let cout = or(and(*a, *b), and(*cin, xor(a, b)));

    // Return both values as owner.
    (sum, cout)
}

// eight bit full adder, receiving as argument two arrays.
// Fn returns a array with 8 boolean expressions + overflow.
fn eight_bit_full_addr(a: &[bool; 8], b: &[bool; 8]) -> ([bool; 8], bool) {
    // Array, boolean expression false by default, array range 8.
    let mut total = [false; 8];
    // Carry out, by default false.
    let mut cout = false;

    // Get (i) in range 0..8. Then reverse to start at righ-most-bit (lsb).
    for i in (0..8).rev() {
        // Function creating a total then assigning it to sum and cin.
        let (addr, overflow) = full_addr(&a[i], &b[i], &cout);
        // Implement addr in total[i].
        total[i] = addr;
        // Carry out is overflow.
        cout = overflow;
    }

    // Return array and carry out(overflow in this case).
    (total, cout)
}