use crate::operators::transformer::{decimal_to_binary, binary_to_decimal};
use crate::adders::{addition_32_bit, subtraction_32_bit};

enum Operation {
    Add,
    Subtract,
}

fn alu(a: u32, b: u32, gate: Operation) -> u32 {
    let byte_a = decimal_to_binary(a);
    let byte_b = decimal_to_binary(b);

    let addition = addition_32_bit(byte_a, byte_b);
    let subtraction = subtraction_32_bit(byte_a, byte_b);

    let dec_add = binary_to_decimal(addition);
    let dec_subtr = binary_to_decimal(subtraction);

    match gate {
        Operation::Add => dec_add,
        Operation::Subtract => dec_subtr, 
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn adds_up_a_and_b() {
        let a = 32;
        let b = 77;
        let operator = Operation::Add;
        let result = alu(a, b, operator);
        let expected = 109;

        assert_eq!((result), (expected));
    }

    #[test]
    fn subtracts_a_from_b() {
        let a = 99;
        let b = 18;
        let operator = Operation::Subtract;
        let result = alu(a, b, operator);
        let expected = 81;

        assert_eq!((result), (expected));
    }
}