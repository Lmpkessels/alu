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
fn mux8way(input: &[[u8; 32]; 8], selector: u8) -> [u8; 32] {
    // Logic: if selector is 0, return A. If 1, return B. Etc.
    if selector == 0 {
        let a = input[0];
        println!("A: {a:?}");
        return a;
    }
    if selector == 1 {
        let b = input[1];
        println!("B: {b:?}");
        return b;
    }
    if selector == 2 {
        let c = input[2];
        println!("C: {c:?}");
        return c;
    }
    if selector == 3 {
        let d = input[3];
        println!("D: {d:?}");
        return d;
    }
    if selector == 4 {
        let e = input[4];
        println!("E: {e:?}");
        return e;
    }
    if selector == 5 {
        let f = input[5];
        println!("F: {f:?}");
        return f;
    }
    if selector == 6 {
        let g = input[6];
        println!("G: {g:?}");
        return g;
    }
    if selector == 7 {
        let h = input[7];
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
        let array0 = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
        0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 0, 1];

        let array1 = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
        0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 1];

        let array2 = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
        0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1];

        let array3 = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
        0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1];

        let array4 = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
        0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 0, 1];

        let array5 = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
        0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 1];

        let array6 = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
        0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1];
        
        let array7 = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
        0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1];

        let input = [
            array0,
            array1,
            array2,
            array3,
            array4,
            array5,
            array6,
            array7,
        ];

        let selector = 0;
        let expected = array0;

        assert_eq!(mux8way(&input, selector), (expected));

        let selector = 1;
        let expected = array1;

        assert_eq!(mux8way(&input, selector), (expected));

        let selector = 2;
        let expected = array2;

        assert_eq!(mux8way(&input, selector), (expected));

        let selector = 3;
        let expected = array3;

        assert_eq!(mux8way(&input, selector), (expected));

        let selector = 4;
        let expected = array4;

        assert_eq!(mux8way(&input, selector), (expected));

        let selector = 5;
        let expected = array5;

        assert_eq!(mux8way(&input, selector), (expected));

        let selector = 6;
        let expected = array6;
        assert_eq!(mux8way(&input, selector), (expected));

        let selector = 7;
        let expected = array7;
        assert_eq!(mux8way(&input, selector), (expected));
    }
}