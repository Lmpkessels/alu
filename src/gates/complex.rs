// MUX returns if selector is 0 bit_a else bit_b. 
pub fn mux(bit_a: u8, bit_b: u8, selector: u8) -> u8 {
    let output_bit = if selector == 0 { 
        return bit_a; 
    } else { 
        return bit_b;
    };
}

// DEMUX if selector is 0 bit_a is returned as input bit with bit_b. 
// Else if selector is 1 bit_b is returned as input bit with bit_a.
pub fn demux(bit_a: u8, bit_b: u8, input_bit: u8, selector: u8) -> (u8, u8) {
    if selector == 0 {
        return (input_bit & 1, bit_b);
    } else {
        return (bit_a, input_bit & 1);
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn returns_a_if_sel_is_0() {
        let a = 1;
        let b = 0;
        let sel = 0;
        let result = mux(a, b, sel);
        let expected = 1;

        assert_eq!((result), (expected));
    }

    #[test]
    fn returns_b_if_sel_is_1() {
        let a = 1;
        let b = 0;
        let sel = 1;
        let result = mux(a, b, sel);
        let expected = 0;

        assert_eq!((result), (expected));
    }
    
    #[test]
    fn a_is_input_if_selector_is_0() {
        let input = 1;
        let a = input;
        let b = 0;
        let selector = 0;
        let result = demux(a, b, input, selector);
        let expected = (input, b);

        assert_eq!((result), (expected));
    }

    #[test]
    fn b_is_input_if_selector_is_1() {
        let input = 0;
        let a = 1;
        let b = input;
        let selector = 1;
        let result = demux(a, b, input, selector);
        let expected = (a, input);

        assert_eq!((result), (expected));
    }
}