fn mux(mut a: u8, mut b: u8, sel: u8) {
    // Check inputs: only bits (0/1) allowed.
    if a > 1 || b > 1 || sel > 1 {
        println!("Error!! All inputs must be 0 or 1.");
        return;
    }

    // Mutable output, default 0, can grow in size to 1.
    let mut out = 0;

    // Default table to display values.
    let table = "(a: {a} b: {b} sel: {sel}) out: {out}";

    // Check sel.
    if sel == 0 {
        // Output is `a` if sel is 0.
        out = a;
        println!("{table}");
        return;
    } else {
        // Output is `b` if sel is 1.
        out = b;
        println!("{table}");
        return;
    }
}

fn demux(sel: u8) {
    // Check input: only bits (0/1) are allowed.
    if sel > 1 {
        println!("Error!! Input must be 0 or 1.");
        return;
    }

    // Input value.
    let inp = "in";

    // Default a & b using String literals.
    let mut a = "0";
    let mut b = "0";

    // Default table to display values.
    let table = "sel: {sel} (a: {a} b: {b})";

    // Check sel.
    if sel == 0 {
        // a is `in` if sel is 0.
        a = inp; 
        println!("{table}");
        return;
    } else {
        // b is `in` if sel is 0.
        b = inp; 
        println!("{table}");
        return;
    } 
}