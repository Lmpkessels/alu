use crate::operators::transformer::{
    decimal_to_binary, binary_to_decimal, addition_decimal_to_binary
};
use crate::adders::{
    addition_32_bit, subtraction_32_bit, multiply_16_x_16, division_32
};

enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

fn alu(a: u32, b: u32, gate: Operation) -> u32 {
    let word_a = decimal_to_binary(a);
    let word_b = decimal_to_binary(b);

    let multiply_word_a = addition_decimal_to_binary(a);
    let multiply_word_b = addition_decimal_to_binary(b);

    let addition = addition_32_bit(word_a, word_b);
    let subtraction = subtraction_32_bit(word_a, word_b);
    let multiplication = multiply_16_x_16(multiply_word_a, multiply_word_b);
    let division = division_32(word_a, word_b);

    let dec_addition = binary_to_decimal(addition);
    let dec_subtraction = binary_to_decimal(subtraction);
    let dec_multiplication = binary_to_decimal(multiplication);
    let dec_division = binary_to_decimal(division);

    match gate {
        Operation::Add => dec_addition,
        Operation::Subtract => dec_subtraction,
        Operation::Multiply => dec_multiplication,
        Operation::Divide => dec_division,
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

    #[test]
    fn multiply_a_by_b() {
        let a = 33;
        let b = 23;
        let operator = Operation::Multiply;
        let result = alu(a, b, operator);
        let expected = 759;

        assert_eq!((result), (expected));
    }

    #[test]
    fn devides_a_by_b() {
        let a = 28;
        let b = 11;
        let operator = Operation::Divide;
        let result = alu(a, b, operator);
        let expected = 2;

        assert_eq!((result), (expected));
    }
}