/* 
8-WAY MULTIPLEXERS

Takes in a nested array and a selector.

Logic:
If selector is 0 -> return A  
If selector is 1 -> return B  
... and so on

The function returns an array of 8 bits when a match is found.
*/

// 8-WAY MULTIPLEXER: selects and returns one of 8 arrays based on selector.
fn mux8way(inp: &[[u8; 8]; 8], sel: u8) -> [u8; 8] {
    // Logic: if selector is 0, return A. If 1, return B. Etc.
    if sel == 0 {
        let a = inp[0];
        println!("A: {a:?}");
        return a;
    }
    if sel == 1 {
        let b = inp[1];
        println!("B: {b:?}");
        return b;
    }
    if sel == 2 {
        let c = inp[2];
        println!("C: {c:?}");
        return c;
    }
    if sel == 3 {
        let d = inp[3];
        println!("D: {d:?}");
        return d;
    }
    if sel == 4 {
        let e = inp[4];
        println!("E: {e:?}");
        return e;
    }
    if sel == 5 {
        let f = inp[5];
        println!("F: {f:?}");
        return f;
    }
    if sel == 6 {
        let g = inp[6];
        println!("G: {g:?}");
        return g;
    }
    if sel == 7 {
        let h = inp[7];
        println!("H: {h:?}");
        return h;
    }

    // Handle invalid selector
    panic!("Invalid selector!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn returns_correct_array_for_each_selector() {
        let a0 = [1, 0, 1, 1, 0, 1, 0, 1];
        let a1 = [1, 0, 0, 1, 0, 0, 1, 1];
        let a2 = [1, 1, 1, 1, 1, 1, 1, 1];
        let a3 = [0, 1, 0, 1, 0, 1, 0, 1];
        let a4 = [1, 0, 1, 1, 0, 1, 0, 1];
        let a5 = [1, 0, 0, 1, 0, 0, 1, 1];
        let a6 = [1, 1, 1, 1, 1, 1, 1, 1];
        let a7 = [0, 1, 0, 1, 0, 1, 0, 1];

        let inp = [
            a0,
            a1,
            a2,
            a3,
            a4,
            a5,
            a6,
            a7,
        ];

        let sel = 0;
        let expected = a0;

        assert_eq!(mux8way(&inp, sel), (expected));

        let sel = 1;
        let expected = a1;

        assert_eq!(mux8way(&inp, sel), (expected));

        let sel = 2;
        let expected = a2;

        assert_eq!(mux8way(&inp, sel), (expected));

        let sel = 3;
        let expected = a3;

        assert_eq!(mux8way(&inp, sel), (expected));

        let sel = 4;
        let expected = a4;

        assert_eq!(mux8way(&inp, sel), (expected));

        let sel = 5;
        let expected = a5;

        assert_eq!(mux8way(&inp, sel), (expected));

        let sel = 6;
        let expected = a6;
        assert_eq!(mux8way(&inp, sel), (expected));

        let sel = 7;
        let expected = a7;
        assert_eq!(mux8way(&inp, sel), (expected));
    }
}