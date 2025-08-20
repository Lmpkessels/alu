use crate::gates::nand;
/*
fn not is a logic gate that returns NOT.

- NOT inverts the input: true becomes false, false becomes true.

--- Truth Table ---
a   NOT(a)
0     1
1     0
*/
pub fn not(bit_a: u8) -> u8 {
    nand(bit_a, bit_a) & 1
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
pub fn and(bit_a: u8, bit_b: u8) -> u8 {
    (bit_a & bit_b) & 1
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
pub fn or(bit_a: u8, bit_b: u8) -> u8 {
    (bit_a | bit_b) & 1
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn computes_bit_by_bit_not_logic() {
        assert_eq!(not(0), 1);
        assert_eq!(not(1), 0);
    }

    #[test]
    fn computes_bit_by_bit_and_logic() {
        assert_eq!(and(0, 0), (0));
        assert_eq!(and(0, 1), (0));
        assert_eq!(and(1, 0), (0));
        assert_eq!(and(1, 1), (1));
    }

    #[test]
    fn computes_bit_by_bit_or_logic() {
        assert_eq!(or(0, 0), (0));
        assert_eq!(or(0, 1), (1));
        assert_eq!(or(1, 0), (1));
        assert_eq!(or(1, 1), (1));
    }
}