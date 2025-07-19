pub fn mux(bit_a: u8, bit_b: u8, selector: u8) -> String {
    // Check inputs: only bits (0/1) allowed.
    if bit_a > 1 || bit_b > 1 || selector > 1 {
        return "Error!! All inputs must be 0 or 1.".to_string();
    };

    // Return a if sel is 0, if sel is 1 return b.
    let output_bit = if selector == 0 { bit_a } else { bit_a };

    format!("(bit_a: {bit_a} bit_b: {bit_b} selector: {selector}) output_bit: {output_bit}")
}

pub fn demux(bit_a: u8, bit_b: u8, input_bit: u8, selector: u8) -> String {
    // Check input: only bits (0/1) are allowed.
    if selector > 1 {
        return "Error!! All inputs must be 0 or 1.".to_string();
    };

    let (bit_a, bit_b) = if selector == 0 {
        // Return input as a if sel is 0.
        (input_bit & 1, bit_b)
    } else {
        // Return input as b if sel is 1.
        (bit_a, input_bit & 1)
    };

    format!("(selector: {selector} input_bit: {input_bit}) (bit_a: {bit_a} bit_b: {bit_b})")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn returns_error_msg_if_a_or_b_or_sel_is_not_0_or_1() {
        let bit_a = 0;
        let bit_b = 1;
        let input_bit = 0;
        let selector = 3;
        let expected = "Error!! All inputs must be 0 or 1.".to_string();

        assert_eq!(mux(bit_a, bit_b, selector), (expected));
        assert_eq!(demux(bit_a, bit_b, input_bit, selector), (expected));
    }

    #[test]
    fn returns_a_if_sel_is_0() {
        let bit_a = 1;
        let bit_b = 0;
        let selector = 0;
        let expected = "(bit_a: 1 bit_b: 0 selector: 0) output_bit: 1".to_string();

        assert_eq!(mux(bit_a, bit_b, selector), (expected));
    }

    #[test]
    fn returns_b_if_sel_is_1() {
        let bit_a = 0;
        let bit_b = 1;
        let selector = 1;
        let expected = "(bit_a: 0 bit_b: 1 selector: 1) output_bit: 0".to_string();

        assert_eq!(mux(bit_a, bit_b, selector), (expected));
    }
    
    #[test]
    fn a_is_input_if_selector_is_0() {
        let bit_a = 0;
        let bit_b = 1;
        let input_bit = 1;
        let selector = 0;
        let expected = "(selector: 0 input_bit: 1) (bit_a: 1 bit_b: 1)".to_string();

        assert_eq!(demux(bit_a, bit_b, input_bit, selector), (expected));
    }

    #[test]
    fn b_is_input_if_selector_is_1() {
        let bit_a = 1;
        let bit_b = 0;
        let input_bit = 1;
        let selector = 1;
        let expected = "(selector: 1 input_bit: 1) (bit_a: 1 bit_b: 1)".to_string();

        assert_eq!(demux(bit_a, bit_b, input_bit, selector), (expected));
    }
}