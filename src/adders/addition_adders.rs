use crate::operators::addition::{sum, carry_out};

fn half_adder(a: u8, b: u8, cin: u8) -> (u8, u8) {
    (sum(a, b, cin), carry_out(a, b, cin))
}

// Full adder returning Sum and Carry out in boolean data-type.
fn full_addr(a: u8, b: u8, cin: u8) -> (u8, u8) {
    // Sum logic -> a AND b = bit, then Bit AND Cin, is end Bit.
    let sum = sum(a, b, cin);
    // Cout logic -> (a AND b = Bit), OR (a XOR b = Bit, AND Cin = Bit) = Bit.
    let cout = carry_out(a, b, cin);

    // Return both values as owner.
    (sum, cout)
}

// eight bit full adder, receiving as argument two arrays.
// Fn returns a array with 8 boolean expressions + overflow.
fn eight_bit_full_addr(a: [u8; 8], b: [u8; 8]) -> ([u8; 8], u8) {
    // Array, boolean expression false by default, array range 8.
    let mut total = [0; 8];
    // Carry out, by default false.
    let mut cout = 0;

    // Get (i) in range 0..8. Then reverse to start at righ-most-bit (lsb).
    for i in (0..8).rev() {
        // Function creating a total then assigning it to sum and cin.
        let (addr, overflow) = full_addr(a[i], b[i], cout);
        // Implement addr in total[i].
        total[i] = addr;
        // Carry out is overflow.
        cout = overflow;
    }

    // Return array and carry out(overflow in this case).
    (total, cout)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_bit_adder_returns_correct_sum_and_carry() {
        assert_eq!(full_addr(0, 0, 0), (0, 0));
        assert_eq!(full_addr(0, 1, 0), (1, 0));
        assert_eq!(full_addr(1, 0, 1), (0, 1));
        assert_eq!(full_addr(1, 1, 1), (1, 1));
    }

    #[test]
    fn eight_bit_adder_correctly_adds_without_overflow() {
        let a = [0, 1, 1, 0, 1, 0, 1, 0];
        let b = [1, 0, 1, 0, 1, 0, 1, 0];
        let expected = ([0, 0, 0, 1, 0, 1, 0, 0], 1);
        assert_eq!(eight_bit_full_addr(a, b), (expected));
    }

    #[test]
    fn eight_bit_adder_handles_full_overflow() {
        let a = [1; 8];
        let b = [1; 8];
        let expected_sum = [1, 1, 1, 1, 1, 1, 1, 0];
        let expected_carry = 1;
        assert_eq!(eight_bit_full_addr(a, b), (expected_sum, expected_carry));
    }

    #[test]
    fn applies_sum_carry_out_logic_and_returns_one_bit() {
        assert_eq!(half_adder(1, 0, 1), (0, 1));
    }
}
