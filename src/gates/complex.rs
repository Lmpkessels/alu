pub fn mux(mut a: u8, mut b: u8, sel: u8) -> String {
    // Check inputs: only bits (0/1) allowed.
    if a > 1 || b > 1 || sel > 1 {
        return "Error!! All inputs must be 0 or 1.".to_string();
    };

    // Return a if sel is 0, if sel is 1 return b.
    let out = if sel == 0 { a } else { b };

    format!("(a: {a} b: {b} sel: {sel}) out: {out}")
}

pub fn demux(mut a: u8, mut b: u8, inp: u8, sel: u8) -> String {
    // Check input: only bits (0/1) are allowed.
    if sel > 1 {
        return "Error!! All inputs must be 0 or 1.".to_string();
    };

    let (a, b) = if sel == 0 {
        // Return input as a if sel is 0.
        (inp & 1, b)
    } else {
        // Return input as b if sel is 1.
        (a, inp & 1)
    };

    format!("(sel: {sel} inp: {inp}) (a: {a} b: {b})")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn returns_error_msg_if_a_or_b_or_sel_is_not_0_or_1() {
        let a = 0;
        let b = 1;
        let inp = 0;
        let sel = 3;
        let expected = "Error!! All inputs must be 0 or 1.".to_string();

        assert_eq!(mux(a, b, sel), (expected));
        assert_eq!(demux(a, b, inp, sel), (expected));
    }

    #[test]
    fn returns_a_if_sel_is_0() {
        let a = 1;
        let b = 0;
        let sel = 0;
        let expected = "(a: 1 b: 0 sel: 0) out: 1".to_string();

        assert_eq!(mux(a, b, sel), (expected));
    }

    #[test]
    fn returns_b_if_sel_is_1() {
        let a = 0;
        let b = 1;
        let sel = 1;
        let expected = "(a: 0 b: 1 sel: 1) out: 1".to_string();

        assert_eq!(mux(a, b, sel), (expected));
    }
    
    #[test]
    fn a_is_input_if_selector_is_0() {
        let a = 0;
        let b = 1;
        let inp = 1;
        let sel = 0;
        let expected = "(sel: 0 inp: 1) (a: 1 b: 1)".to_string();

        assert_eq!(demux(a, b, inp, sel), (expected));
    }

    #[test]
    fn b_is_input_if_selector_is_1() {
        let a = 1;
        let b = 0;
        let inp = 1;
        let sel = 1;
        let expected = "(sel: 1 inp: 1) (a: 1 b: 1)".to_string();

        assert_eq!(demux(a, b, inp, sel), (expected));
    }
}