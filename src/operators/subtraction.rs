use crate::gates::basic::{not, and, or, xor};

// Returns difference after applying bit_a XOR bit_b = bit XOR borrow_out = result. 
// So it knows what to write down at the bits column.
pub fn difference_bit(bit_a: u8, bit_b: u8, borrow_in_bit: u8) -> u8 {
    xor(xor(bit_a, bit_b), borrow_in_bit)
}

// Check if a borrow if needed so it never ends up in underflow when using unassigned
// math.
pub fn borrow_out_bit(bit_a: u8, bit_b: u8, borrow_in_bit: u8) -> u8 {
    or(and(not(bit_a), bit_b), and(not(xor(bit_a, bit_b)), borrow_in_bit))
}