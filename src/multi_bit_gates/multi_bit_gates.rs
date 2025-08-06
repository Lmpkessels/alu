// Multi 16 bit NOT gate flips each 'bit' in the array.
// ex. logic: a = 1, (NOT(a)) = 0.
fn not16(input_word: [u8; 16]) -> [u8; 16] {
    let mut output_word = [0; 16];

    for bit in 0..16 {
        output_word[bit] = (!input_word[bit]) & 1;
    } 

    output_word
}

// Multi-16-bit AND gate, receives 2 arrays from 'stack'.
// Makes both arrays go bit-by-bit through a AND logic gate.
// Returns one array after using AND gate.
// ex. logic: a = 1, b = 0, (a AND b) = 0.
fn and16(word_a: [u8; 16], word_b: [u8; 16]) -> [u8; 16] {
    let mut output_word = [0; 16];

    for bit in 0..16 {
        output_word[bit] = (word_a[bit] & word_b[bit]) & 1;        
    }

    output_word
}

// Multi-16-bit OR gate, receives 2 arrays from 'stack'.
// Makes both arrays go bit-by-bit through a OR logic gate.
// Returns one array after using OR gate.
// ex. logic: a = 1, b = 0, (a OR b) = 1.
fn or16(word_a: [u8; 16], word_b: [u8; 16]) -> [u8; 16] {
    let mut output_word = [0; 16];

    for bit in 0..16 {
        output_word[bit] = (word_a[bit] | word_b[bit]) & 1;        
    }

    output_word
}


// Multi 16 bit mux gate, receives 2 arrays from 'stack'.
// Checks if selector 'sel' is 'true' if so returns array 'b' else 'a'.
// ex. logic: inp_a = 1, inp_b = 0, sel = 0 (Output = 1)
fn mux16(word_a: [u8; 16], word_b: [u8; 16], selector: bool) -> [u8; 16] {
    let mut output_word = [0; 16];

    for bit in 0..16 {
        output_word[bit] = if selector  {
            word_b[bit]
        } else {
            word_a[bit]
        };
    }

    output_word
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn loops_through_input_takes_index_bit_by_bit_applies_not() {
        let input_word = [0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1];
        let expected_word = [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0];
        assert_eq!(not16(input_word), (expected_word));
    }

    #[test]
    fn loops_through_a_and_b_takes_index_bit_by_bit_applies_and() {
        let word_a = [1, 0, 1, 1, 0, 1, 1, 1, 1, 0, 0, 1, 0, 0, 1, 1];
        let word_b = [1, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 1];
        let expected_word = [1, 0, 1, 1, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 1];

        assert_eq!(and16(word_a, word_b), (expected_word));
    }

    #[test]
    fn loops_through_a_and_b_takes_index_bit_by_bit_applies_or() {
        let word_a = [1, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 1, 1, 0, 1];
        let word_b = [1, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 1, 1, 0, 1];
        let expected_word = [1, 0, 0, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1];

        assert_eq!(or16(word_a, word_b), (expected_word));
    }

    #[test]
    fn if_sel_is_false_a_else_if_true_b() {
        let word_a = [1, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 1, 1, 0, 1];
        let word_b = [0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1];
        let selector_false = false;
        let selector_true = true;

        assert_eq!(mux16(word_a, word_b, selector_false), (word_a));

        assert_eq!(mux16(word_a, word_b, selector_true), (word_b));
    }
}