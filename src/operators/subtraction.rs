use crate::gates::basic::{not, and, or, xor};

pub fn difference_bit(bit_a: u8, bit_b: u8, borrow_in_bit: u8) -> u8 {
    xor(xor(bit_a, bit_b), borrow_in_bit)
}

pub fn borrow_out_bit(bit_a: u8, bit_b: u8, borrow_in_bit: u8) -> u8 {
    or(and(not(bit_a), bit_b), and(not(xor(bit_a, bit_b)), borrow_in_bit))
}