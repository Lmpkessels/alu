/*
fn and is a logic gate that returns AND.

- AND returns true only if both inputs are true.

--- Truth Table ---
a   b   AND(a, b)
0   0       0
0   1       0
1   0       0
1   1       1
*/
pub fn and(a: bool, b: bool) -> bool {
    a && b
}

/*
fn or is a logic gate that returns OR.

- OR returns true if at least one input is true.

--- Truth Table ---
a   b   OR(a, b)
0   0       0
0   1       1
1   0       1
1   1       1
*/
pub fn or(a: bool, b: bool) -> bool {
    a || b
}

/*
fn not is a logic gate that returns NOT.

- NOT inverts the input: true becomes false, false becomes true.

--- Truth Table ---
a   NOT(a)
0     1
1     0
*/
pub fn not(a: bool) -> bool {
    !a
}

/*
fn xor is a logic gate that returns XOR.

- XOR returns true only if one (and only one) input is true.

--- Truth Table ---
a   b   XOR(a, b)
0   0       0
0   1       1
1   0       1
1   1       0
*/
pub fn xor(a: &bool, b: &bool) -> bool {
    or(and(*a, not(*b)), and(not(*a), *b))
}

/*
fn nand is a logic gate that returns NOT AND.

- AND returns true only if both inputs are true.
- NOT inverts the result.

--- Truth Table ---
a   b   AND(a, b)   NAND(a, b)
0   0       0           1
0   1       0           1
1   0       0           1
1   1       1           0
*/
pub fn nand(a: &bool, b: &bool) -> bool {
    not(and(*a, *b))
}

/*
fn nor is a logic gate that returns NOT OR.

- OR returns true if at least one input is true.
- NOT inverts the result.

--- Truth Table ---
a   b   OR(a, b)   NOR(a, b)
0   0       0           1
0   1       1           0
1   0       1           0
1   1       1           0
*/
pub fn nor(a: &bool, b: &bool) -> bool {
    not(or(*a, *b))
}

/*
fn xnor is a logic gate that returns NOT XOR.

- XOR returns true only if one (and only one) input is true.
- NOT inverts the result.

--- Truth Table ---
a   b   XOR(a, b)   XNOR(a, b)
0   0       0           1
0   1       1           0
1   0       1           0
1   1       0           1
*/
pub fn xnor(a: &bool, b: &bool) -> bool {
    not(xor(a, b))
}