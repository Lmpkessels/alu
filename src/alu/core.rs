use crate::operators::transformer::{
    int_to_32bit, int_to_16bit, bit32_to_int
};
use crate::adders::{
    add_32bit_o, sub_32bit_u, multiply_16x16bit, div_32bit
};

// Operation featurs of ALU.
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

/// ALU (Arithmetic Logic Unit).
///
/// Takes two integers and an operation as input.
/// Internally, the integers are transformed into binary words,
/// the chosen operation is applied, and the result is transformed
/// back into an integer.
///
/// Supported operations:
/// - Add
/// - Subtract
/// - Multiply
/// - Divide
///
/// Returns the result as a decimal integer.
fn alu(a: u32, b: u32, gate: Operation) -> (u32, u8) {
    let word32_a = int_to_32bit(a);
    let word32_b = int_to_32bit(b);

    let word16_a = int_to_16bit(a);
    let word16_b = int_to_16bit(b);

    let addition = add_32bit_o(word32_a, word32_b);
    let subtraction = sub_32bit_u(word32_a, word32_b);
    let multiplication = multiply_16x16bit(word16_a, word16_b);
    let division = div_32bit(word32_a, word32_b);

    let dec_addition = bit32_to_int(addition);
    let dec_subtraction = bit32_to_int(subtraction);
    let dec_multiplication = bit32_to_int(multiplication);
    let dec_division = bit32_to_int(division);

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
        let expected = (109, 0);

        assert_eq!((result), (expected));
    }

    #[test]
    fn subtracts_a_from_b() {
        let a = 99;
        let b = 18;
        let operator = Operation::Subtract;
        let result = alu(a, b, operator);
        let expected = (81, 0);

        assert_eq!((result), (expected));
    }

    #[test]
    fn multiply_a_by_b() {
        let a = 33;
        let b = 23;
        let operator = Operation::Multiply;
        let result = alu(a, b, operator);
        let expected = (759, 0);

        assert_eq!((result), (expected));
    }

    #[test]
    fn devides_a_by_b() {
        let a = 28;
        let b = 11;
        let operator = Operation::Divide;
        let result = alu(a, b, operator);
        let expected = (2, 0);

        assert_eq!((result), (expected));
    }
}