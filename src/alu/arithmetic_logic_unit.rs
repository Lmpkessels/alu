use crate::operators::transformer::{
    int_to_32bit, int_to_16bit, t2bit_to_int
};
use crate::adders::{
    add_32bit, sub_32bit, multiply_16x16bit, div_32bit
};

enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

fn alu(a: u32, b: u32, gate: Operation) -> u32 {
    let word_a = int_to_32bit(a);
    let word_b = int_to_32bit(b);

    let multiply_word_a = int_to_16bit(a);
    let multiply_word_b = int_to_16bit(b);

    let addition = add_32bit(word_a, word_b);
    let subtraction = sub_32bit(word_a, word_b);
    let multiplication = multiply_16x16bit(multiply_word_a, multiply_word_b);
    let division = div_32bit(word_a, word_b);

    let dec_addition = t2bit_to_int(addition);
    let dec_subtraction = t2bit_to_int(subtraction);
    let dec_multiplication = t2bit_to_int(multiplication);
    let dec_division = t2bit_to_int(division);

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