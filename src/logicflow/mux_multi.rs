/* 
4-WAY and 8-WAY MULTIPLEXERS

Both functions take in a nested array and a selector.

Logic:
If selector is 0 -> return A  
If selector is 1 -> return B  
... and so on

The function returns an array of 8 bits when a match is found.
*/

// 4-WAY MULTIPLEXER: selects and returns one of 4 arrays based on selector.
fn mux4way(inp: &[[u8; 8]; 4], sel: u8) -> [u8; 8] {
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

    // Handle invalid selector
    panic!("Invalid selector");
}

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



fn main() {
    // Create argument for input mux4way.
    let check = [
        [1, 0, 1, 1, 0, 1, 0, 1],
        [1, 0, 0, 1, 0, 0, 1, 1],
        [1, 1, 1, 1, 1, 1, 1, 1],
        [0, 1, 0, 1, 0, 1, 0, 1],
    ];

    // Check if mux4way works as progrmmed for.
    mux4way(&check, 0);
    mux4way(&check, 1);
    mux4way(&check, 2);
    mux4way(&check, 3);

    println!("\n");

    // Create argument for input mux8way.
    let check_2 = [
        [1, 0, 1, 1, 0, 1, 0, 1],
        [1, 0, 0, 1, 0, 0, 1, 1],
        [1, 1, 1, 1, 1, 1, 1, 1],
        [0, 1, 0, 1, 0, 1, 0, 1],
        [1, 0, 1, 1, 0, 1, 0, 1],
        [1, 0, 0, 1, 0, 0, 1, 1],
        [1, 1, 1, 1, 1, 1, 1, 1],
        [0, 1, 0, 1, 0, 1, 0, 1],
    ];

    // Check if mux8way works as progrmmed for.
    mux8way(&check_2, 0);
    mux8way(&check_2, 1);
    mux8way(&check_2, 2);
    mux8way(&check_2, 3);
    mux8way(&check_2, 4);
    mux8way(&check_2, 5);
    mux8way(&check_2, 6);
    mux8way(&check_2, 7);
}