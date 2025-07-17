use crate::gates::basic::{not, and, or, xor};

pub fn diff(a: u8, b: u8, bin: u8) -> u8 {
    xor(xor(a, b), bin)
}

pub fn borrow_out(a: u8, b: u8, bin: u8) -> u8 {
    or(and(not(a), b), and(not(xor(a, b)), bin))
}