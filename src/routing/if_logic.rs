use crate::gates::basic::{not, and, or};

/*
fn _if is a logic expression that returns True
only if `a` is true and `b` is false.

--- Logic ---
a AND (NOT b)

--- Truth Table ---
a   b   NOT(b)  AND(a, NOT(b))
0   0     1           0
0   1     0           0
1   0     1           1
1   1     0           0
*/
pub fn _if(a: u8, b: u8) -> u8 {
   and(a, not(b))
}

/*
fn if_then is a logical implication.

Returns True in all cases except when `a` is true and `b` is false.
In words: IF a is true THEN b must be true.

--- Logic ---
NOT a OR b   ≡   IF a THEN b

--- Truth Table ---
a   b   NOT(a)  OR(NOT(a), b)
0   0     1          1
0   1     1          1
1   0     0          0
1   1     0          1
*/
pub fn if_then(a: u8, b: u8) -> u8 {
   or(not(a), b)
}

/*
fn if_then_else is a logical expression that simulates:

IF a THEN b ELSE c

Returns True if:
- `a` is true and `b` is true, OR
- `a` is false and `c` is true.

--- Logic ---
(a AND b) OR (NOT a AND c)

--- Truth Table ---
a   b   c   AND(a, b)    NOT(a)     AND(NOT(a), c)  OR(AND(a, b), AND(NOT(a), c))
0   0   0       0           1             0                     0
0   1   0       0           1             0                     0
1   0   1       0           0             0                     0
1   1   1       1           0             0                     1
*/
pub fn if_then_else(a: u8, b: u8, c: u8) -> u8 {
   or(and(a, b), and(not(a), c))
}

fn main() {

}

#[cfg(test)]
mod test {
   use super::*;

   #[test]
   fn a_returns_true_only_if_b_is_false() {
      assert_eq!(_if(0, 0), (0));
      assert_eq!(_if(0, 1), (0));
      assert_eq!(_if(1, 0), (1));
      assert_eq!(_if(1, 1), (0));
   }

   #[test]
   fn if_then_only_returns_false_if_a_is_true_and_b_is_false() {
      assert_eq!(if_then(0, 0), (1));
      assert_eq!(if_then(0, 1), (1));
      assert_eq!(if_then(1, 0), (0));
      assert_eq!(if_then(1, 1), (1));
   }

   #[test]
   fn if_then_else_returns_true_if_a_is_0_or_a_and_b_are_both_0_and_c_is_1() {
      assert_eq!(if_then_else(0, 0, 0), (0));
      assert_eq!(if_then_else(0, 0, 1), (1));
      assert_eq!(if_then_else(0, 1, 1), (1));
      assert_eq!(if_then_else(1, 1, 1), (1));
      assert_eq!(if_then_else(1, 0, 1), (0));
      assert_eq!(if_then_else(1, 1, 0), (1));
   }
}