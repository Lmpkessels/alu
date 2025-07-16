/*
fn not is a logic gate that returns NOT.

- NOT inverts the input: true becomes false, false becomes true.

--- Truth Table ---
a   NOT(a)
0     1
1     0
*/
pub fn not(a: u8) -> u8 {
    nand(a, a)
}

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
pub fn and(a: u8, b: u8) -> u8 {
(a & b) & 1
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
pub fn or(a: u8, b: u8) -> u8 {
    (a | b) & 1
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
pub fn xor(a: u8, b: u8) -> u8 {
    or(and(a, not(b)), and(not(a), b))
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
pub fn nand(a: u8, b: u8) -> u8 {
    !(a & b) & 1
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
pub fn nor(a: u8, b: u8) -> u8 {
    not(or(a, b))
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
pub fn xnor(a: u8, b: u8) -> u8 {
    not(xor(a, b))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn reverse_bit_using_not() {
        assert_eq!(not(0), 1);
        assert_eq!(not(1), 0);
    }

    #[test]
    fn applies_and_logic_bit_by_bit() {
        assert_eq!(and(0, 0), (0));
        assert_eq!(and(0, 1), (0));
        assert_eq!(and(1, 0), (0));
        assert_eq!(and(1, 1), (1));
    }

    #[test]
    fn applies_or_logic_bit_by_bit() {
        assert_eq!(or(0, 0), (0));
        assert_eq!(or(0, 1), (1));
        assert_eq!(or(1, 0), (1));
        assert_eq!(or(1, 1), (1));
    }

    #[test]
    fn applies_xor_logic_bit_by_bit() {
        assert_eq!(xor(0, 0), (0));
        assert_eq!(xor(0, 1), (1));
        assert_eq!(xor(1, 0), (1));
        assert_eq!(xor(1, 1), (0));
    }

    #[test]
    fn applies_nand_logic_bit_by_bit() {
        assert_eq!(nand(0, 0), (1));    
        assert_eq!(nand(0, 1), (1));    
        assert_eq!(nand(1, 0), (1));    
        assert_eq!(nand(1, 1), (0));    
    }

    #[test]
    fn applies_nor_logic_bit_by_bit() {
        assert_eq!(nor(0, 0), (1));
        assert_eq!(nor(0, 1), (0));
        assert_eq!(nor(1, 0), (0));
        assert_eq!(nor(1, 1), (0));
    }

    #[test]
    fn applies_xnor_logic_bit_by_bit() {
        assert_eq!(xnor(0, 0), (1));
        assert_eq!(xnor(0, 1), (0));
        assert_eq!(xnor(1, 0), (0));
        assert_eq!(xnor(1, 1), (1));
    }
}